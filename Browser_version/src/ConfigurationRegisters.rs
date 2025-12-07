use modular_bitfield::bitfield;
use modular_bitfield::prelude::{B1, B6, B3, B2, B5, B8, B15,B4};

#[bitfield (bytes=4)]
#[derive(Debug)]
pub struct COR0 {
    GWE_CYCLE:B3,
    GTS_CYCLE:B3,
    LOCK_CYCLE:B3,
    MATCH_CYCLE:B3,
    DONE_CYCLE:B3,
    RESERVED_1:B2,
    OSCFSEL:B6,
    RESERVED_2:B1,
    DRIVE_DONE:B1,
    RESERVED_3:B1,
    ECLK_EN:B1,
    RESERVED_4:B5,
}

#[bitfield (bytes=4)]
#[derive(Debug)]
pub struct BPI_SPI_Configuration {
    pub SPI_READ_OPCODE:B8,
    pub SPI_BUSWIDTH:B2,
    pub SPI_32BIT_ADDR:B1,
    RESERVED_1:B1,
    BPI_SYNC_RCR:B15,
    BPI_SYNC_MODE:B1,
    RESERVED_2:B4,

}
