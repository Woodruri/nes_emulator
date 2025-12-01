/* 6502 has the following flags, each one bit:
Bit 0: Carry flag: set if last instruction caused overflow from bit 7 or underflow from bit 0
Bit 1:Zero flag: set if result of last op was 0
Bit 2:Interrupt disable: set if SEI (set interrupt disable instruction). While set, will ignore 
    interrupts until CLI instruction is called
Bit 3:Decimal mode: set with SED (Set decimal flag) and disabled with CLD (Clear decimal flag) instructions
    while set, processor uses Binary Coded Decimal arithmatic during add or sub 
Bit 4:Break Command: Set when BRK instruction called and interrupt generated to process it
Bit 5: Always set 1, unused
Bit 6:Overflow flag: set if during arithmetic op, invalid 2s complement is returned
Bit 7:Negative flag: set if result of last op had a 7th bit set to 1 */

const CARRY_FLAG:u8 = 0b0000_0001;
const ZERO_FLAG:u8 = 0b0000_0010;
const INTERRUPT_DISABLE:u8 = 0b0000_0100;
const DECIMAL_MODE:u8 = 0b0000_1000;
const BREAK_COMMAND:u8 = 0b0001_0000;
// const UNUSED_FLAG:u8 = 0b0010_0000; //don't use this, it's there for accuracy
const OVERFLOW_FLAG:u8 = 0b0100_0000;
const NEGATIVE_FLAG:u8 = 0b1000_0000;

pub struct CPU {
    pub register_a: u8,
    pub register_b: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_b: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    //main loop
    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opscode {

                //reset/BRK
                00 => return,

                //LDA, Load accumulator, immediate
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;

                    self.lda(param);
                }

                //TAX, transfer accumulator to X, implied
                0xAA => self.tax(),

                //INX, increment register x, implied
                0xE8 => self.inx(),

                //default
                _ => todo!("How did you get here?")
            }
        }
    }

    /*
    op codes ================================================
    */
    //0xA9
    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    //0xAA
    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    //0xE8
    fn inx(&mut self) {
        self.register_x += 1;
        self.update_zero_and_negative_flags(self.register_x);
    }


    /*
    support funcs ================================================
    */
    fn update_zero_and_negative_flags(&mut self, register: u8) {
        //set 0 flag if param == 0
        if register == 0 {
            self.status = self.status | ZERO_FLAG;
        }
        //unset it if not
        else {
            self.status = self.status & 0b1111_1101;
        }
        //set negative flag if 7th bit is set in reg_a
        if register & NEGATIVE_FLAG != 0 {
            self.status = self.status | NEGATIVE_FLAG;
        }
        //unset it if not
        else {
            self.status = self.status & 0b0111_1111;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data(){
        let mut cpu = CPU::new();
        let instruction_set = vec![0xa9, 0x05, 0x00];

        cpu.interpret(instruction_set);

        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & ZERO_FLAG == 0b00);
        assert!(cpu.status & NEGATIVE_FLAG == 0);
    }
    
    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        let instruction_set = vec![0xa9, 0x00, 0x00];

        cpu.interpret(instruction_set);
        assert!(cpu.status & ZERO_FLAG == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu=CPU::new();
        cpu.register_a = 10;
        let instruction_set = vec![0xaa, 0x00];
        cpu.interpret(instruction_set);

        assert_eq!(cpu.register_x, 10);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_0xe8_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        let instruction_set = vec![0xe8, 0xe8, 0x00];
        cpu.interpret(instruction_set);

        assert_eq!(cpu.register_x, 1);
    }

    #[test]
    fn test_all_5_ops_together() {
        let mut cpu = CPU::new();
        let instruction_set = vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00];
        cpu.interpret(instruction_set);
        
        assert_eq!(cpu.register_x, 0xc1)
    }
}