use crate::CPU::AddressingMode;
use std::collections::HashMap;


pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    fn new(code: u8, mnemonic: &'static str, len: u8, cycles: u8, mode: AddressingMode) {
        OpCode {
            code: code,
            mnemonic: mnemonic,
            len: len,
            cycles: cycles,
            mode: mode,
        }
    }

}

lazy_static! {
    public static ref CPU_OP_CODES:Vec<OpCode> vec![

        //========================= BRK ===================================
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),


        //========================= TAX ===================================
        OpCode::new(0xaa, "TAX", 1, 2, AddressingMode::NoneAddressing),


        //========================= INX ===================================
        OpCode::new(0xe8, "INX", 1, 2, AddressingMode::NoneAddressing),

        //========================= LDA ===================================
        OpCode::new(0xa9, "LDA", 2, 2, AddressingMode::Immediate),

        OpCode::new(0xa5, "LDA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb5, "LDA", 2, 4, AddressingMode::ZeroPage_X),

        OpCode::new(0xad, "LDA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbd, "LDA", 3, 4, AddressingMode::Absolute_X), //+1 if page crossed
        OpCode::new(0xb9, "LDA", 3, 4, AddressingMode::Absolute_Y), //+1 if page crossed

        OpCode::new(0xa1, "LDA", 2, 4, AddressingMode::Indirect_X),
        OpCode::new(0xb1, "LDA", 2, 4, AddressingMode::Indirect_Y), //+1 if page crossed

        //========================= STA ===================================
        OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X),

        OpCode::new(0x8d, "STA", 2, 4, AddressingMode::Absolute),
        OpCode::new(0x9d, "STA", 2, 5, AddressingMode::Absolute_X),
        OpCode::new(0x99, "STA", 2, 5, AddressingMode::Absolute_Y),

        OpCode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y),


    ]
}