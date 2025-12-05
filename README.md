# Xilinx Bitfile Readback Utility

For checking configuration and commands in the bitfile.


# Compiling

# Build & Run instructions : 

1. Install Rust: https://rustup.rs
2. Clone or unzip project
3. cd project
4. cargo build
5. cargo run -- [bit file path]

# Example run output

```
Opening file: Kaynak_data/simple_counter_quadspi_externalclkdiv2.bit
BPI_SPI_Configuration : SPI_READ_OPCODE : 0x6b, bus_width : 4, 32-bit adr : false
Command register Write. Command : BSPI_READ
Config register Write to TIMER with value : 0x0
Config register Write to WBSTAR with value : 0x10203040
Command register Write. Command : IPROG
Command register Write. Command : RCRC
Config register Write to RBCRC_SW with value : 0x0
COR0 Write : 6403fe5 COR0 { GWE_CYCLE: 5, GTS_CYCLE: 4, LOCK_CYCLE: 7, MATCH_CYCLE: 7, DONE_CYCLE: 3, RESERVED_1: 0, OSCFSEL: 32, RESERVED_2: 0, DRIVE_DONE: 0, RESERVED_3: 1, ECLK_EN: 1, RESERVED_4: 0 }
Config register Write to COR1 with value : 0x0
ID Code Config register Write. Bit file is prepared for the following device : Artix-7 (XC7A35T)
Command register Write. Command : SWITCH
Config register Write to MASK with value : 0x1
Config register Write to CTL0 with value : 0x101
Config register Write to MASK with value : 0x0
Config register Write to CTL1 with value : 0x0
Config register Write to FAR with value : 0x0
Command register Write. Command : WCFG
Bulk data Write ( 547420  Double Words ) ( 2189680 bytes ) 
Config register Write to CRC with value : 0xf6589aaf
Command register Write. Command : GRESTORE
Command register Write. Command : DGHIGH_LFRM
Command register Write. Command : START
Config register Write to FAR with value : 0x3be0000
Config register Write to MASK with value : 0x101
Config register Write to CTL0 with value : 0x101
Config register Write to CRC with value : 0x7db41709
Command register Write. Command : DESYNC

```

# Notes about IDCode decoding 

The application currently uses a small dataset for generating IDCODE to device description. You can add lines to the "Kaynak_data/Known_ID_Codes.txt" file if you need to verify bitstream's IDCODE for your device.
