mod TypeDefinitions;
mod LookupHelpers;
mod ConfigurationRegisters;

use std::fs::File;
use std::io::{Read, Seek};

use crate::TypeDefinitions::{ConfigRegs, Opcodes};
use modular_bitfield::prelude::*;
use crate::TypeDefinitions::Opcodes::Write;

#[bitfield (bytes=4)]
#[derive(Debug)]
pub struct Type1Packet {
    word_count: B11,
    reserved_1: B2,
    reg_adr: B5,
    reserved_2: B9,
    opcode : B2,
    header_type:B3
}

#[bitfield (bytes=4)]
#[derive(Debug)]
pub struct Type2Packet {
    word_count: B27,
    opcode : B2,
    header_type:B3
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let file_path = match std::env::args().nth(1) {
        Some(file_path) => file_path,
        _ =>"Kaynak_data/bin_counter_bitfile.bin".to_string()
    };

    println!("Opening file: {}", file_path);

    let mut bitfile = std::fs::File::open(file_path)?;

    seek_to_preamble(&mut bitfile)?;

    let lookup_utils = LookupHelpers::LookupData::new();

    //for i in 0..20{
    loop{

        let packet_read_result = read_packet(&mut bitfile);

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
                    continue;
                }
                if pk1.word_count() > 1{ // word count 1 tane olmali
                    println!("Consuming unexpected word count");
                    for i in 0..pk1.word_count(){
                        let dw = read_BE_DW(&mut bitfile)?;
                        println!(" {:#x}",dw);
                    }
                    continue;
                }
                let mut cmd_operand_bytes = [0u8;4];

                bitfile.read_exact(&mut cmd_operand_bytes)?;
                cmd_operand_bytes.reverse();

                match config_register {
                    ConfigRegs::COR0 => {
                        let cor0 = ConfigurationRegisters::COR0::from(cmd_operand_bytes);
                        println!("COR0 Write : {:?}", cor0);
                    }

                    ConfigRegs::CMD => {
                        let cmd_reg = lookup_utils.lookup_cmd_reg_from_id(u32::from_be_bytes(cmd_operand_bytes) as u8);
                        println!("Command register {}. Command : {:?}",Opcodes::from(pk1.opcode()),cmd_reg);
                    }

                    _ => {
                        println!("Config register {} to {:?} with value : {:#x}",Opcodes::from(pk1.opcode()), config_register, u32::from_be_bytes(cmd_operand_bytes));
                    }
                }


            },
            2 => {

                println!("{:?}", pk2);

                for i in 0..pk2.word_count(){
                    let dw = read_BE_DW(&mut bitfile)?;
                    //println!(" {:#x}",dw);

                }
            }
            _ => {
                eprintln!("Unknown packet type");
                //panic!("Unknown packet type");
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Unknown packet type")));
            }
        }



    }



    Ok(())
}


fn read_packet(file : &mut File) -> Result<(Type1Packet,Type2Packet), std::io::Error> {

    let mut dw_bytes = [0u8;4];

    file.read_exact(&mut dw_bytes)?;
    dw_bytes.reverse();
    let pk1 = Type1Packet::from(dw_bytes);
    let pk2 = Type2Packet::from(dw_bytes);
    Ok((pk1,pk2))

}
fn read_BE_DW( file: &mut File) -> Result<(u32), Box<dyn std::error::Error>> {

    let mut dw_bytes = [0u8;4];

    file.read_exact(&mut dw_bytes)?;
    let dw = u32::from_be_bytes(dw_bytes);
    Ok(dw)

}
fn seek_to_preamble( file: &mut File) -> Result<(), Box<dyn std::error::Error>> {

    const PREAMBLE: [u8; 4] = [0xaa,0x99,0x55,0x66];

    let mut dw_bytes = [0u8;4];

    file.read_exact(&mut dw_bytes)?;

    while dw_bytes != PREAMBLE {
        file.read_exact(&mut dw_bytes)?;
    }

    file.seek(std::io::SeekFrom::Current((PREAMBLE.len() as i64) - 4))?;

    Ok(())
}
