use std::collections::HashMap;
use std::fs::read_to_string;

pub(crate) struct DecodeData{
    decoding_map: HashMap<u16, String>,
}

impl DecodeData {
    pub fn new() -> DecodeData {
        let mut decoding_map = HashMap::new();
        let src_data = read_to_string("Kaynak_data/Known_ID_Codes.txt");

        if let Ok(src_data) = src_data {
            src_data.split("\n").for_each(|line| {
                let mut pair = line.split(" ").into_iter().collect::<Vec<&str>>();
                if pair.len() == 2{
                    let key_subset = &pair[0][2..]; // TODO : Should be a better way
                    let val = u16::from_str_radix(key_subset,16);
                    if let Ok(val) = val {
                        decoding_map.insert(val, pair[1].to_string());
                    }

                }

            })
        }

        DecodeData{decoding_map: decoding_map}
    }

    pub fn decode_id(&self, code_value:u32) -> String {
        let device_id:u16 = (code_value >> 12) as u16;

        if self.decoding_map.contains_key(&device_id) {
            return self.decoding_map[&device_id].clone();
        }

        format!("Unknown ID : {:#x}", device_id)
    }
}
