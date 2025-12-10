//use IDCODE_Decoder::DecodeData;



mod IDCODE_Decoder;
//use IDCODE_Decoder::*;

//use super::IDCODE_Decoder::*;


/// Unit tests
#[cfg(test)]
mod tests {



    use crate::IDCODE_Decoder::DecodeData;

    #[test]
    fn test_parse_line_methods(){

        //use IDCODE_Decoder::*;
        //use super::IDCODE_Decoder::DecodeData;
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
}
