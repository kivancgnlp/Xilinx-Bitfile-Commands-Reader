use std::fmt::{Display, Formatter};
use crate::TypeDefinitions::Opcodes::{Nop, Read, Reserved, Write};


#[derive(Debug,Copy,Clone,PartialEq,Eq)]
#[repr(u8)]
pub(crate) enum ConfigRegs {
    CRC = 0,
    FAR,
    FDRI,
    FDRO,
    CMD,
    CTL0,
    MASK,
    STAT,
    LOUT,
    COR0,
    MFWR,
    CBC,
    IDCODE,
    AXSS,
    COR1,
    //
    WBSTAR = 0x10,
    TIMER = 0x11,
    RBCRC_SW = 0x13,
    BOOTSTS = 0x16,
    CTL1 = 0x18,
    BSPI = 0x1F,
}

#[derive(Debug)]
#[repr(u8)]
pub(crate) enum Opcodes {
    Nop = 0,
    Read = 1,
    Write = 2,
    Reserved = 3,
}


impl Display for Opcodes{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        
        let str_rep = match self {
            Nop => "Nop",
            Read => "Read",
            Write => "Write",
            Reserved => "Reserved",
        };
        write!(f, "{}", str_rep)
    }
}

impl From<u8> for Opcodes{
    fn from(value: u8) -> Self {
        match value {
            0 => Nop,
            1 => Read,
            2 => Write,
            3 => Reserved,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub(crate) enum CmdRegs {
    NULL,
    WCFG,
    MFW,
    DGHIGH_LFRM,
    RCFG,
    START,
    RCAP,
    RCRC,
    AGHIGH,
    SWITCH,
    GRESTORE,
    SHUTDOWN,
    GCAPTURE,
    DESYNC,
    Reserved,
    IPROG,
    CRCC,
    LTIMER,
    BSPI_READ,
    FALL_EDGE,
    IPPROG,
}

