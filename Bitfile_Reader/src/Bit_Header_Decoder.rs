use std::collections::HashMap;
use std::io::Read;


fn parse_TLV(mut file_input_stream: impl Read) -> Result<(String,String), Box<dyn std::error::Error>>{
    let mut tag = [0_u8];
    let mut len = [0_u8,0];
    let mut byte = [0_u8];


    file_input_stream.read_exact(&mut tag) ?;
    file_input_stream.read_exact(&mut len) ?;

    let length = u16::from_be_bytes(len);

    //println!("length: {}", length);

    let mut value_str_buffer = Vec::<u8>::new();

    for i in 0..length {
        file_input_stream.read_exact(&mut byte) ?;
        value_str_buffer.push(byte[0]);
    }

    let val_str = String::from_utf8(value_str_buffer) ?;

    let tag_str = String::from(tag[0] as char);

    Ok((tag_str,val_str))
}

pub(crate) fn dump_bit_header(mut file_input_stream: impl Read){

    let mut magic_header = [0u8;13];

    file_input_stream.read_exact(&mut magic_header);

    if magic_header != [0_u8,0x09,0x0F,0xF0,0x0F,0xF0,0x0F,0xF0,0x0F,0xF0,0x00,0x00,0x01]{
        println!("Header magic doesnt match");
        return;
    }

    println!("*********** Bit file header dump ( this part will not be present on the flash ) *********");

    let mut tag_map = HashMap::<String,String>::new();
    tag_map.insert("a".to_string(), "Design name".to_string());
    tag_map.insert("b".to_string(), "Part name".to_string());
    tag_map.insert("c".to_string(), "Date".to_string());
    tag_map.insert("d".to_string(), "Time".to_string());
    tag_map.insert("e".to_string(), "Bitstream length".to_string());


    while let Ok((tag,value)) = parse_TLV(&mut file_input_stream){

        if tag_map.contains_key(&tag) {
            println!("{:<14}({}) : {}", tag_map[&tag], tag, value);
        }else{
            println!("Unknown tag {}", tag);
            break;
        }

        if tag == "e"{
            break;
        }

    }


    println!("*********** Bit file header dump end *********");


}