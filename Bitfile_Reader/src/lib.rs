//use IDCODE_Decoder::DecodeData;



pub mod IDCODE_Decoder;
mod File_Parse_Helpers;
mod TypeDefinitions;


/// Unit tests
#[cfg(test)]
mod tests {
    use std::io::{Cursor, Read};



    #[test]
    fn test_parse_line_methods(){
        use crate::IDCODE_Decoder::DecodeData;


        let mut result = DecodeData::parse_line_method_1("");
        assert_eq!(result,None);

        result = DecodeData::parse_line_method_1(" ");
        assert_eq!(result,None);

        result = DecodeData::parse_line_method_1("# This is a minimal dataset");
        assert_eq!(result,None);

        result = DecodeData::parse_line_method_1("362D Artix-7 (XC7A35T)");
        assert_eq!(result,None);

        result = DecodeData::parse_line_method_1("0x362DArtix-7(XC7A35T)");
        assert_eq!(result,None);

        result = DecodeData::parse_line_method_1("0x362D Artix-7 (XC7A35T)");
        assert_eq!(result.unwrap(),(0x362D,"Artix-7 (XC7A35T)"));

    }

    #[test]
    fn seek_to_preamle_test(){
        use crate::File_Parse_Helpers;

        let byte_buffer = vec![0xFF_u8,0xFF,0xAA,0x99,0x55,0x66,0x20];
        let mut cursor= Cursor::new(byte_buffer);

        let result = File_Parse_Helpers::seek_to_preamble(&mut cursor);

        assert!(result.is_ok());
        assert_eq!(cursor.position(),6);
    }

    #[test]
    fn packet_read_test(){
        use crate::File_Parse_Helpers;

        let mut cursor= Cursor::new(vec![0x30,0x02,0x20,0x01]);// Sample TYPE 1 Frame

        let mut result = File_Parse_Helpers::read_packet(&mut cursor);

        assert!(result.is_ok());

        assert_eq!(result.unwrap().0.header_type(),1);

        cursor= Cursor::new(vec![0x50,0x3D,0x0D,0xA6]); // Sample TYPE 2 Frame

        result = File_Parse_Helpers::read_packet(&mut cursor);
        assert!(result.is_ok());

        let (f1,f2) = result.unwrap();

        assert_eq!(f1.header_type(),2);

        assert_eq!(f2.word_count(), 4001190)





    }
}
