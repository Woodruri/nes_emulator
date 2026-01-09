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

    lazy_static! {
        public static ref CPU_OP_CODES:Vec<OpCode> vec![

            //========================= BRK/NOP ===================================
            OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),
            OpCode::new(0xea, "NOP", 1, 2, AddressingMode::NoneAddressing),


            //========================= ARITHMETIC ===================================
            //Add with carry
            OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
            OpCode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPage_X),
            OpCode::new(0x6d, "ADC", 3, 4, AddressingMode::Absolute),
            OpCode::new(0x7d, "ADC", 3, 4, AddressingMode::Absolute_X), //+1 if page crossed
            OpCode::new(0x79, "ADC", 3, 4, AddressingMode::Absolute_Y), //+1 if page crossed
            OpCode::new(0x61, "ADC", 2, 6, AddressingMode::Indirect_X),
            OpCode::new(0x71, "ADC", 2, 5, AddressingMode::Indirect_Y), //+1 if page crossed

            //Decrement memory
            OpCode::new(0xc6, "DEC", 2, 5, AddressingMode::ZeroPage),
            OpCode::new(0xd6, "DEC", 2, 6, AddressingMode::ZeroPage_X),
            OpCode::new(0xce, "DEC", 3, 6, AddressingMode::Absolute),
            OpCode::new(0xde, "DEC", 3, 7, AddressingMode::Absolute_X),

            //Decrement X
            OpCode::new(0xca, "DEX", 1, 2, AddressingMode::NoneAddressing),
            
            //Decrement Y
            OpCode::new(0x88, "DEY", 1, 2, AddressingMode::NoneAddressing),

            //Increment memory
            OpCode::new(0xe6, "INC", 2, 5, AddressingMode::ZeroPage),
            OpCode::new(0xf6, "INC", 2, 6, AddressingMode::ZeroPage_X),
            OpCode::new(0xee, "INC", 3, 6, AddressingMode::Absolute),
            OpCode::new(0xfe, "INC", 3, 7, AddressingMode::Absolute_X),

            //Increment X
            OpCode::new(0xe8, "INX", 1, 2, AddressingMode::NoneAddressing),

            //Increment Y
            OpCode::new(0xc8, "INY", 1, 2, AddressingMode::NoneAddressing),

            //========================= LOGICAL ARITHMETIC ===================================
            //Logical AND
            OpCode::new(0x29, "AND", 2, 2, AddressingMode::Immediate),
            OpCode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPage_X),
            OpCode::new(0x2d, "AND", 3, 4, AddressingMode::Absolute),
            OpCode::new(0x3d, "AND", 3, 4, AddressingMode::Absolute_X), //+1 if page crossed
            OpCode::new(0x39, "AND", 3, 4, AddressingMode::Absolute_Y), //+1 if page crossed
            OpCode::new(0x21, "AND", 2, 6, AddressingMode::Indirect_X),
            OpCode::new(0x31, "AND", 2, 5, AddressingMode::Indirect_Y), //+1 if page crossed

            //XOR
            OpCode::new(0x49, "EOR", 2, 2, AddressingMode::Immediate),
            OpCode::new(0x45, "EOR", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0x55, "EOR", 2, 4, AddressingMode::ZeroPage_X),
            OpCode::new(0x4d, "EOR", 3, 4, AddressingMode::Absolute),
            OpCode::new(0x5d, "EOR", 3, 4, AddressingMode::Absolute_X), //+1 if page crossed
            OpCode::new(0x59, "EOR", 3, 4, AddressingMode::Absolute_Y), //+1 if page crossed
            OpCode::new(0x41, "EOR", 2, 6, AddressingMode::Indirect_X),
            OpCode::new(0x51, "EOR", 2, 5, AddressingMode::Indirect_Y), //+1 if page crossed

            //========================= BRANCH ===================================
            //Branch if Carry clear
            OpCode::new(0x90, "BCC", 2, 2, AddressingMode::NoneAddressing),
            //Branch if Carry set
            OpCode::new(0xb0, "BCS", 2, 2, AddressingMode::NoneAddressing),
            //Branch if Equal
            OpCode::new(0xf0, "BEQ", 2, 2, AddressingMode::NoneAddressing),
            //Branch if Minus
            OpCode::new(0x30, "BMI", 2, 2, AddressingMode::NoneAddressing),
            //Branch if Not Equal
            OpCode::new(0xD0, "BNE", 2, 2, AddressingMode::NoneAddressing),
            //Branch if Positive
            OpCode::new(0x10, "BPL", 2, 2, AddressingMode::NoneAddressing),
            //Branch if Overflow clear
            OpCode::new(0x50, "BVC", 2, 2, AddressingMode::NoneAddressing),
            //Branch if Overflow set
            OpCode::new(0x70, "BVS", 2, 2, AddressingMode::NoneAddressing),
            //+1 if branch succeeds, +2 if to new page for all of the above

            //Jump
            OpCode::new(0x4c, "JMP", 3, 3, AddressingMode::NoneAddressing), //Acts as immediate addressing mode
            OpCode::new(0x6c, "JMP", 3, 5, AddressingMode::NoneAddressing), //Acts as indirect but need to add bug

            //Jump to subroutine
            OpCode::new(0x20, "JSR", 3, 6, AddressingMode::Absolute)

            //========================= SHIFTS ===================================
            //Arithmetic shift left
            OpCode::new(0x0a, "ASL", 1, 2, AddressingMode::NoneAddressing),
            OpCode::new(0x06, "ASL", 2, 5, AddressingMode::ZeroPage),
            OpCode::new(0x16, "ASL", 2, 6, AddressingMode::ZeroPage_X),
            OpCode::new(0x0e, "ASL", 3, 6, AddressingMode::Absolute),
            OpCode::new(0x1e, "ASL", 3, 7, AddressingMode::Absolute_X),

            //========================= COMPARES ===================================
            //Compare
            OpCode::new(0xc9, "CMP", 2, 2, AddressingMode::Immediate),
            OpCode::new(0xc5, "CMP", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0xd5, "CMP", 2, 4, AddressingMode::ZeroPage_X),
            OpCode::new(0xcd, "CMP", 3, 4, AddressingMode::Absolute),
            OpCode::new(0xdd, "CMP", 3, 4, AddressingMode::Absolute_X), //+1 if page crossed
            OpCode::new(0xd9, "CMP", 3, 4, AddressingMode::Absolute_Y), //+1 if page crossed
            OpCode::new(0xc1, "CMP", 2, 6, AddressingMode::Indirect_X),
            OpCode::new(0xd1, "CMP", 2, 5, AddressingMode::Indirect_Y), //+1 if page crossed

            //Compare x
            OpCode::new(0xe0, "CPX", 2, 2, AddressingMode::Immediate),
            OpCode::new(0xe4, "CPX", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0xec, "CPX", 3, 4, AddressingMode::Absolute),

            //Compare y
            OpCode::new(0xc0, "CPY", 2, 2, AddressingMode::Immediate),
            OpCode::new(0xc4, "CPY", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0xcc, "CPY", 3, 4, AddressingMode::Absolute),

            //========================= STORES & LOADS ===================================
            //Load Accumulator
            OpCode::new(0xa9, "LDA", 2, 2, AddressingMode::Immediate),
            OpCode::new(0xa5, "LDA", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0xb5, "LDA", 2, 4, AddressingMode::ZeroPage_X),
            OpCode::new(0xad, "LDA", 3, 4, AddressingMode::Absolute),
            OpCode::new(0xbd, "LDA", 3, 4, AddressingMode::Absolute_X), //+1 if page crossed
            OpCode::new(0xb9, "LDA", 3, 4, AddressingMode::Absolute_Y), //+1 if page crossed
            OpCode::new(0xa1, "LDA", 2, 4, AddressingMode::Indirect_X),
            OpCode::new(0xb1, "LDA", 2, 4, AddressingMode::Indirect_Y), //+1 if page crossed

            //Store Accumulator
            OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X),
            OpCode::new(0x8d, "STA", 2, 4, AddressingMode::Absolute),
            OpCode::new(0x9d, "STA", 2, 5, AddressingMode::Absolute_X),
            OpCode::new(0x99, "STA", 2, 5, AddressingMode::Absolute_Y),
            OpCode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X),
            OpCode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y),
            
            //========================= FLAGS ===================================

            //Bit test
            OpCode::new(0x24, "BIT", 2, 3, AddressingMode::ZeroPage),
            OpCode::new(0x2c, "BIT", 3, 4, AddressingMode::Absolute),

            //Clear Carry flag
            OpCode::new(0x18, "CLC", 1, 2, AddressingMode::NoneAddressing),
            //Clear Decimal mode
            OpCode::new(0xD8, "CLD", 1, 2, AddressingMode::NoneAddressing),
            //Clear Interrupt disable
            OpCode::new(0x58, "CLI", 1, 2, AddressingMode::NoneAddressing),
            //Clear Overflow flag
            OpCode::new(0xB8, "CLV", 1, 2, AddressingMode::NoneAddressing),
            

            //========================= TRANSFERS ===================================
            //Transfer accumulator to x 
            OpCode::new(0xaa, "TAX", 1, 2, AddressingMode::NoneAddressing),


        ];

        public static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
            let mut map = HashMap::new();
            for cpuop in &'CPU_OP_CODES {
                map.insert(cpuop.code, cpuop)
            }
        }
    }

}