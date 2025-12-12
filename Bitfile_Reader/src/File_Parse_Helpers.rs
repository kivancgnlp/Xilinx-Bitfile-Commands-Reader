use std::io::{Error, ErrorKind, Read, Result};


//use crate::TypeDefinitions::Type1Packet;
use crate::TypeDefinitions::{Type1Packet, Type2Packet};

pub(crate) fn read_packet(file : &mut impl Read) -> Result<(Type1Packet, Type2Packet)> {

    let mut dw_bytes = [0u8;4];

    file.read_exact(&mut dw_bytes)?;
    dw_bytes.reverse();
    let pk1 = Type1Packet::from(dw_bytes);
    let pk2 = Type2Packet::from(dw_bytes);
    Ok((pk1,pk2))

}
pub(crate) fn read_BE_DW(file: &mut impl Read) -> Result<u32> {

    let mut dw_bytes = [0u8;4];

    file.read_exact(&mut dw_bytes)?;
    let dw = u32::from_be_bytes(dw_bytes);
    Ok(dw)

}

/// Basic implementation of seeking the preamble [0xaa,0x99,0x55,0x66]
pub(crate) fn seek_to_preamble<R: Read>(file: &mut R) -> Result<()> {

    //
    const PREAMBLE: [u8; 4] = [0xaa,0x99,0x55,0x66];

    let mut seek_bytes: [u8; 1] = [0u8;1];

    while seek_bytes[0]!=0xaa {
        file.read_exact(&mut seek_bytes)?;
    }

    let mut dw_bytes = [0u8;3];

    file.read_exact(&mut dw_bytes)?;

    if PREAMBLE[1..] != dw_bytes {
        return Err(Error::new(ErrorKind::InvalidData, "Invalid preamble."));
    }


    Ok(())
}
