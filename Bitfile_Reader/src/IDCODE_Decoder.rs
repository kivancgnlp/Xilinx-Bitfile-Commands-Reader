use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

pub(crate) struct DecodeData{
    decoding_map: HashMap<u16, String>,
}

impl DecodeData {

    pub fn new() -> DecodeData {

        let decoding_map = Self::parse_id_codes("Kaynak_data/Known_ID_Codes.txt");

        DecodeData{decoding_map: decoding_map}
    }

    fn parse_id_codes(path: &str) -> HashMap<u16, String> {
        let mut dec_map = HashMap::<u16, String>::new();

        if let Ok(file) = fs::File::open(path) {
            let reader = BufReader::new(file);

            for line in reader.lines() {
                if let Ok(l) = line {
                    if l.starts_with("#") || l.is_empty(){ // Skip comments and empty lines
                        continue;
                    }
                    if let Some(space_place) = l.find(char::is_whitespace) {
                        if let Some(x_place) = l.find("0x" ) {
                            if let Ok(key) = u16::from_str_radix(&l[x_place+2..space_place],16){
                                dec_map.insert(key, l[space_place+1..].to_string());
                            }

                        }

                    }
                }
            }

        }else {
            eprintln!("Could not open file: {}", path);
        }

        dec_map

    }

    pub fn try_to_guess_device_id(&self, code_value:u32) -> Option<String> {
        let device_id:u16 = (code_value >> 12) as u16;

        if self.decoding_map.contains_key(&device_id) {
            return Some(self.decoding_map[&device_id].clone());
        }

        None
    }
}
