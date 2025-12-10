

use Bitfile_Reader::IDCODE_Decoder;

// Integration tests
#[test]
fn test_idcode_decoder_basic() {
    let d = IDCODE_Decoder::DecodeData::new();

    let mut result = d.try_to_guess_device_id(0x362d093);
    assert_eq!(result.unwrap(),"Artix-7 (XC7A35T)");

    result = d.try_to_guess_device_id(0x03727093);
    assert_eq!(result.unwrap(),"Zynq-7000 (XC7Z010)");

    result = d.try_to_guess_device_id(0x04E4093);
    assert_eq!(result.unwrap(),"UltraScale (XCKU025)");
    
    
}