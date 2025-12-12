mod TypeDefinitions;
mod LookupHelpers;
mod ConfigurationRegisters;
mod IDCODE_Decoder;
mod Bit_Header_Decoder;
mod File_Parse_Helpers;

use std::io::{BufReader, Read, Seek, Result, Error};
use std::process;
use TypeDefinitions::{ConfigRegs, Opcodes};


fn main() -> Result<()> {

    let file_path = match std::env::args().nth(1) {
        Some(file_path) => file_path,
        _ =>"Kaynak_data/simple_counter_quadspi_externalclkdiv2.bit".to_string()
    };

    println!("Opening file: {}", file_path);

    let input_file = std::fs::File::open(&file_path).inspect_err(|e| {
        eprintln!("Unable to open file: {} , {}", file_path,e);
    })?;
     

    let mut bitfile = BufReader::new(input_file);
    
    if file_path.ends_with(".bit"){
        Bit_Header_Decoder::dump_bit_header(&mut bitfile);
    }

    File_Parse_Helpers::seek_to_preamble(&mut bitfile).inspect_err(|e| {
        println!("Unable to find preamble: {}", e);
    })?;
    

    let lookup_utils = LookupHelpers::LookupData::new();
    //println!("Lookup data : {}",lookup_utils);
    let id_code_decoder = IDCODE_Decoder::DecodeData::new();

    
    loop{

        let packet_read_result = File_Parse_Helpers::read_packet(&mut bitfile);

        if packet_read_result.is_err() {
            println!("End of file reached");
            break;
        }

        let (pk1,pk2) = packet_read_result.unwrap();

        match pk1.header_type() {
            1 => {


                let op_code = TypeDefinitions::Opcodes::from(pk1.opcode());
                let config_register = lookup_utils.lookup_config_reg_from_id(pk1.reg_adr());

                //println!("OP code : {:?}, config_register : {:?}",op_code,config_register);

                if pk1.word_count() == 0 {
                    // no operand

                    let opc = Opcodes::from(pk1.opcode());

                    if opc != Opcodes::Nop{
                        println!("Config register {} to {:?} without operand", opc, config_register);
                    }

                    continue;
                }
    
                let mut cmd_operand_bytes = [0u8;4];

                bitfile.read_exact(&mut cmd_operand_bytes)?;
                cmd_operand_bytes.reverse();

                match config_register {
                    ConfigRegs::COR0 => {
                        let cor0 = ConfigurationRegisters::COR0::from(cmd_operand_bytes);
                        println!("COR0 Write : {:x}{:x}{:x}{:x} {:?}", cmd_operand_bytes[3], cmd_operand_bytes[2], cmd_operand_bytes[1], cmd_operand_bytes[0], cor0);
                    }

                    ConfigRegs::BSPI => {
                        let parsed_BSPI = ConfigurationRegisters::BPI_SPI_Configuration::from_bytes(cmd_operand_bytes);

                        let bus_width = match parsed_BSPI.SPI_BUSWIDTH() {
                            0 => 1,
                            1 => 2,
                            2 => 4,
                            _ => 0
                        };
                        println!("BPI_SPI_Configuration : SPI_READ_OPCODE : {:#x}, bus_width : {}, 32-bit adr : {}",parsed_BSPI.SPI_READ_OPCODE(), bus_width, parsed_BSPI.SPI_BUSWIDTH() == 1);
                    }

                    ConfigRegs::CMD => {
                        let cmd_reg = lookup_utils.lookup_cmd_reg_from_id(cmd_operand_bytes[0]);
                        println!("Command register {}. Command : {:?}",Opcodes::from(pk1.opcode()),cmd_reg);
                    }

                    ConfigRegs::IDCODE => {
                        let id_code = u32::from_le_bytes(cmd_operand_bytes);

                        let device_guess = match id_code_decoder.try_to_guess_device_id(id_code){
                            Some(device_guess) => device_guess,
                            _ => format!(" {:#x} Not present in Known_ID_Codes.txt", id_code >> 16)

                        };

                        println!("ID Code Config register {}. This bitstream is prepared for the following device : {}",Opcodes::from(pk1.opcode()), device_guess);

                    }

                    _ => {
                        println!("Config register {} to {:?} with value : {:#x}",Opcodes::from(pk1.opcode()), config_register, u32::from_le_bytes(cmd_operand_bytes));
                    }
                }

                if pk1.word_count() > 1{ // word count 1 tane olmali
                    println!("{} words follows", pk1.word_count() - 1);
                    for i in 0..pk1.word_count() - 1{
                        let dw = File_Parse_Helpers::read_BE_DW(&mut bitfile)?;
                        //println!(" {:#x}",dw);
                    }
                    continue;
                }


            },
            2 => {

                println!("Bulk data {} ( {}  Double Words ) ( {} bytes ) ", Opcodes::from(pk2.opcode()), pk2.word_count(), pk2.word_count() * 4);

                for i in 0..pk2.word_count(){
                    let dw = File_Parse_Helpers::read_BE_DW(&mut bitfile)?;
                    //println!(" {:#x}",dw);

                }
            }
            _ => {
                eprintln!("Unknown packet type");
                return Err(Error::new(std::io::ErrorKind::Other, "Unknown packet type"));
            }
        }

    }



    Ok(())
}

