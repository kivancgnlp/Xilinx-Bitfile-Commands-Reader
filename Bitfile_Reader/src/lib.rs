
mod IDCODE_Decoder;


#[cfg(test)]
#[test]
fn test_idcode_decode(){
    let d = IDCODE_Decoder::DecodeData::new();
    println!("IDCode : {:?}",d.try_to_guess_device_id(0x362d093));
    println!("IDCode : {:?}",d.try_to_guess_device_id(0x03727093));
    println!("IDCode : {:?}",d.try_to_guess_device_id(0x04E4093));

}