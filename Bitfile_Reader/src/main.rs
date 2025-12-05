mod TypeDefinitions;
mod LookupHelpers;
mod ConfigurationRegisters;
mod IDCODE_Decoder;

use std::io::{BufReader, Read, Seek};

use crate::TypeDefinitions::{ConfigRegs, Opcodes, Type1Packet, Type2Packet};


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let file_path = match std::env::args().nth(1) {
        Some(file_path) => file_path,
        //_ =>"Kaynak_data/simple_counter.bit".to_string()
        _ =>"Kaynak_data/bin_counter_bitfile.bin".to_string()
    };

    println!("Opening file: {}", file_path);

    let input_file = std::fs::File::open(file_path)?;

    let mut bitfile = BufReader::new(input_file);

    let seek_result = seek_to_preamble(&mut bitfile);

    if seek_result.is_err() {
        eprintln!("Unable find preamble");
        return Err(seek_result.err().unwrap());
    }

    let lookup_utils = LookupHelpers::LookupData::new();
    //println!("Lookup data : {}",lookup_utils);
    let id_code_decoder = IDCODE_Decoder::DecodeData::new();

    
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
    
                let mut cmd_operand_bytes = [0u8;4];

                bitfile.read_exact(&mut cmd_operand_bytes)?;
                cmd_operand_bytes.reverse();

                match config_register {
                    ConfigRegs::COR0 => {
                        let cor0 = ConfigurationRegisters::COR0::from(cmd_operand_bytes);
                        println!("COR0 Write : {:x}{:x}{:x}{:x} {:?}", cmd_operand_bytes[3], cmd_operand_bytes[2], cmd_operand_bytes[1], cmd_operand_bytes[0], cor0);
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

                        println!("ID Code Config register {}. Bit file is prepared for the following device : {}",Opcodes::from(pk1.opcode()), device_guess);

                    }

                    _ => {
                        println!("Config register {} to {:?} with value : {:#x}",Opcodes::from(pk1.opcode()), config_register, u32::from_le_bytes(cmd_operand_bytes));
                    }
                }

                if pk1.word_count() > 1{ // word count 1 tane olmali
                    println!("{} words follows", pk1.word_count() - 1);
                    for i in 0..pk1.word_count() - 1{
                        let dw = read_BE_DW(&mut bitfile)?;
                        //println!(" {:#x}",dw);
                    }
                    continue;
                }


            },
            2 => {

                println!("Bulk data {} ( {}  Double Words ) ( {} bytes ) ", Opcodes::from(pk2.opcode()), pk2.word_count(), pk2.word_count() * 4);

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


fn read_packet(file : &mut impl Read) -> Result<(Type1Packet, Type2Packet), std::io::Error> {

    let mut dw_bytes = [0u8;4];

    file.read_exact(&mut dw_bytes)?;
    dw_bytes.reverse();
    let pk1 = Type1Packet::from(dw_bytes);
    let pk2 = Type2Packet::from(dw_bytes);
    Ok((pk1,pk2))

}
fn read_BE_DW( file: &mut impl Read) -> Result<(u32), Box<dyn std::error::Error>> {

    let mut dw_bytes = [0u8;4];

    file.read_exact(&mut dw_bytes)?;
    let dw = u32::from_be_bytes(dw_bytes);
    Ok(dw)

}
fn seek_to_preamble<R: Read + Seek>(file: &mut R)-> Result<(), Box<dyn std::error::Error>> {

    const PREAMBLE: [u8; 4] = [0xaa,0x99,0x55,0x66];

    let mut seek_bytes: [u8; 1] = [0u8;1];

    while seek_bytes[0]!=0xaa {
        file.read_exact(&mut seek_bytes)?;
    }

    file.seek(std::io::SeekFrom::Current(-1))?;


    let mut dw_bytes = [0u8;4];

    file.read_exact(&mut dw_bytes)?;

    while dw_bytes != PREAMBLE {
        file.read_exact(&mut dw_bytes)?;
    }

    file.seek(std::io::SeekFrom::Current((PREAMBLE.len() as i64) - 4))?;

    Ok(())
}
