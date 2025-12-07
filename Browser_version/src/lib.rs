mod TypeDefinitions;
mod LookupHelpers;
mod ConfigurationRegisters;
mod IDCODE_Decoder;
mod Bit_Header_Decoder;

use std::io::{Cursor, Read, Seek};
use wasm_bindgen::prelude::*;
use crate::TypeDefinitions::{ConfigRegs, Opcodes, Type1Packet, Type2Packet};

// Embed the ID codes data directly
const ID_CODES_DATA: &str = r#"# This is a minimal dataset for the device_id to device name mapping
# Format is upper 16 bits of the idcode and device description seperated by white space after hexadecimal number.
# Hexadecimal number should start with "0x" and there must be a white space between them and their description

0x362D Artix-7 (XC7A35T)
0x3727 Zynq-7000 (XC7Z010)
0x04e4 UltraScale (XCKU025)"#;

// Helper function to append a line to output
fn append_line(output: &mut String, s: &str) {
    output.push_str(s);
    output.push('\n');
}

#[wasm_bindgen]
pub fn process_bitfile(bytes: &[u8]) -> String {
    let mut output = String::new();
    
    let mut bitfile = Cursor::new(bytes);
    
    // Check if it's a .bit file by checking the magic header
    if bytes.len() >= 13 {
        let magic_header = &bytes[0..13];
        if magic_header == [0_u8,0x09,0x0F,0xF0,0x0F,0xF0,0x0F,0xF0,0x0F,0xF0,0x00,0x00,0x01] {
            dump_bit_header_wasm(&mut bitfile, &mut output);
        }
    }
    
    let seek_result = seek_to_preamble(&mut bitfile);
    
    if seek_result.is_err() {
        append_line(&mut output, "Unable to find preamble");
        return output;
    }
    
    let lookup_utils = LookupHelpers::LookupData::new();
    let id_code_decoder = create_idcode_decoder();
    
    loop {
        let packet_read_result = read_packet(&mut bitfile);
        
        if packet_read_result.is_err() {
            append_line(&mut output, "End of file reached");
            break;
        }
        
        let (pk1, pk2) = packet_read_result.unwrap();
        
        match pk1.header_type() {
            1 => {
                let config_register = lookup_utils.lookup_config_reg_from_id(pk1.reg_adr());
                
                if pk1.word_count() == 0 {
                    let opc = Opcodes::from(pk1.opcode());
                    if opc != Opcodes::Nop {
                        append_line(&mut output, &format!("Config register {} to {:?} without operand", opc, config_register));
                    }
                    continue;
                }
                
                let mut cmd_operand_bytes = [0u8;4];
                
                if bitfile.read_exact(&mut cmd_operand_bytes).is_err() {
                    break;
                }
                cmd_operand_bytes.reverse();
                
                match config_register {
                    ConfigRegs::COR0 => {
                        let cor0 = ConfigurationRegisters::COR0::from(cmd_operand_bytes);
                        append_line(&mut output, &format!("COR0 Write : {:x}{:x}{:x}{:x} {:?}", 
                            cmd_operand_bytes[3], cmd_operand_bytes[2], 
                            cmd_operand_bytes[1], cmd_operand_bytes[0], cor0));
                    }
                    
                    ConfigRegs::BSPI => {
                        let parsed_BSPI = ConfigurationRegisters::BPI_SPI_Configuration::from(cmd_operand_bytes);
                        
                        let bus_width = match parsed_BSPI.SPI_BUSWIDTH() {
                            0 => 1,
                            1 => 2,
                            2 => 4,
                            _ => 0
                        };
                        append_line(&mut output, &format!("BPI_SPI_Configuration : SPI_READ_OPCODE : {:#x}, bus_width : {}, 32-bit adr : {}",
                            parsed_BSPI.SPI_READ_OPCODE(), bus_width, parsed_BSPI.SPI_BUSWIDTH() == 1));
                    }
                    
                    ConfigRegs::CMD => {
                        let cmd_reg = lookup_utils.lookup_cmd_reg_from_id(cmd_operand_bytes[0]);
                        append_line(&mut output, &format!("Command register {}. Command : {:?}", Opcodes::from(pk1.opcode()), cmd_reg));
                    }
                    
                    ConfigRegs::IDCODE => {
                        let id_code = u32::from_le_bytes(cmd_operand_bytes);
                        
                        let device_guess = match id_code_decoder.try_to_guess_device_id(id_code) {
                            Some(device_guess) => device_guess,
                            _ => format!(" {:#x} Not present in Known_ID_Codes.txt", id_code >> 16)
                        };
                        
                        append_line(&mut output, &format!("ID Code Config register {}. This bitstream is prepared for the following device : {}",
                            Opcodes::from(pk1.opcode()), device_guess));
                    }
                    
                    _ => {
                        append_line(&mut output, &format!("Config register {} to {:?} with value : {:#x}",
                            Opcodes::from(pk1.opcode()), config_register, u32::from_le_bytes(cmd_operand_bytes)));
                    }
                }
                
                if pk1.word_count() > 1 {
                    append_line(&mut output, &format!("{} words follows", pk1.word_count() - 1));
                    for _i in 0..pk1.word_count() - 1 {
                        if read_BE_DW(&mut bitfile).is_err() {
                            break;
                        }
                    }
                    continue;
                }
            },
            2 => {
                append_line(&mut output, &format!("Bulk data {} ( {}  Double Words ) ( {} bytes ) ",
                    Opcodes::from(pk2.opcode()), pk2.word_count(), pk2.word_count() * 4));
                
                for _i in 0..pk2.word_count() {
                    if read_BE_DW(&mut bitfile).is_err() {
                        break;
                    }
                }
            },
            _ => {
                append_line(&mut output, "Unknown packet type");
                break;
            }
        }
    }
    
    output
}

fn create_idcode_decoder() -> IDCODE_Decoder::DecodeData {
    use std::collections::HashMap;
    
    let mut decoding_map = HashMap::<u16, String>::new();
    
    for line in ID_CODES_DATA.lines() {
        if line.starts_with("#") || line.is_empty() {
            continue;
        }
        
        if let Some((key, value)) = parse_id_code_line(line) {
            decoding_map.insert(key, value.to_string());
        }
    }
    
    IDCODE_Decoder::DecodeData::from_map(decoding_map)
}

fn parse_id_code_line(l: &str) -> Option<(u16, &str)> {
    if let Some(space_place) = l.find(char::is_whitespace) {
        if let Some(x_place) = l.find("0x") {
            if let Some(hex_val_str) = l.get(x_place + 2..space_place) {
                if let Ok(key) = u16::from_str_radix(hex_val_str, 16) {
                    if let Some(desc_str) = l.get(space_place + 1..) {
                        return Some((key, desc_str));
                    }
                }
            }
        }
    }
    None
}

fn dump_bit_header_wasm(mut file_input_stream: impl Read, output: &mut String) {
    use std::collections::HashMap;
    
    let mut magic_header = [0u8;13];
    
    if file_input_stream.read_exact(&mut magic_header).is_err() {
        return;
    }
    
    if magic_header != [0_u8,0x09,0x0F,0xF0,0x0F,0xF0,0x0F,0xF0,0x0F,0xF0,0x00,0x00,0x01] {
        output.push_str("Header magic doesnt match\n");
        return;
    }
    
    output.push_str("*********** Bit file header dump ( this part will not be present on the flash ) *********\n");
    
    let mut tag_map = HashMap::<String,String>::new();
    tag_map.insert("a".to_string(), "Design name".to_string());
    tag_map.insert("b".to_string(), "Part name".to_string());
    tag_map.insert("c".to_string(), "Date".to_string());
    tag_map.insert("d".to_string(), "Time".to_string());
    tag_map.insert("e".to_string(), "Bitstream length".to_string());
    
    while let Ok((tag, value)) = parse_TLV(&mut file_input_stream) {
        if tag_map.contains_key(&tag) {
            output.push_str(&format!("{:<14}({}) : {}\n", tag_map[&tag], tag, value));
        } else {
            output.push_str(&format!("Unknown tag {}\n", tag));
            break;
        }
        
        if tag == "e" {
            break;
        }
    }
    
    output.push_str("*********** Bit file header dump end *********\n");
}

fn parse_TLV(mut file_input_stream: impl Read) -> Result<(String,String), Box<dyn std::error::Error>> {
    let mut tag = [0_u8];
    let mut len = [0_u8,0];
    let mut byte = [0_u8];
    
    file_input_stream.read_exact(&mut tag)?;
    file_input_stream.read_exact(&mut len)?;
    
    let length = u16::from_be_bytes(len);
    
    let mut value_str_buffer = Vec::<u8>::new();
    
    for _i in 0..length {
        file_input_stream.read_exact(&mut byte)?;
        value_str_buffer.push(byte[0]);
    }
    
    let val_str = String::from_utf8(value_str_buffer)?;
    let tag_str = String::from(tag[0] as char);
    
    Ok((tag_str,val_str))
}

fn read_packet(file: &mut impl Read) -> Result<(Type1Packet, Type2Packet), std::io::Error> {
    let mut dw_bytes = [0u8;4];
    
    file.read_exact(&mut dw_bytes)?;
    dw_bytes.reverse();
    let pk1 = Type1Packet::from(dw_bytes);
    let pk2 = Type2Packet::from(dw_bytes);
    Ok((pk1,pk2))
}

fn read_BE_DW(file: &mut impl Read) -> Result<u32, Box<dyn std::error::Error>> {
    let mut dw_bytes = [0u8;4];
    
    file.read_exact(&mut dw_bytes)?;
    let dw = u32::from_be_bytes(dw_bytes);
    Ok(dw)
}

fn seek_to_preamble<R: Read + Seek>(file: &mut R) -> Result<(), Box<dyn std::error::Error>> {
    const PREAMBLE: [u8; 4] = [0xaa,0x99,0x55,0x66];
    
    let mut seek_bytes: [u8; 1] = [0u8;1];
    
    while seek_bytes[0]!=0xaa {
        if file.read_exact(&mut seek_bytes).is_err() {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Could not find 0xaa")));
        }
    }
    
    file.seek(std::io::SeekFrom::Current(-1))?;
    
    let mut dw_bytes = [0u8;4];
    
    if file.read_exact(&mut dw_bytes).is_err() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Could not read preamble")));
    }
    
    while dw_bytes != PREAMBLE {
        if file.read_exact(&mut dw_bytes).is_err() {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Could not find preamble")));
        }
    }
    
    file.seek(std::io::SeekFrom::Current((PREAMBLE.len() as i64) - 4))?;
    
    Ok(())
}
