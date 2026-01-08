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

const PROGRAM_COUNTER_READ_LOCATION:u16 = 0xFFFC;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

//for little endian architecutre of NES :(
trait Mem {
    fn mem_read(&self, addr: u16) -> u8;

    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xFF) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }
}

impl Mem for CPU {

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0 ; 0xFFFF],
        }
    }


    /*
    public funcs ================================================
    */

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;

        //NES is defined to read the PC from 0xFFFC on reset
        self.program_counter = self.mem_read_u16(PROGRAM_COUNTER_READ_LOCATION);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(PROGRAM_COUNTER_READ_LOCATION, 0x8000);
    }

    pub fn load_and_read(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    //main loop
    pub fn run(&mut self) {

        loop {
            let opscode =  self.mem_read(self.program_counter);
            self.program_counter += 1;

            match opscode {

                //reset/BRK
                00 => return,

                //LDA, Load accumulator
                //immediate
                0xA9 => {
                    self.lda(&AddressingMode::Immediate);
                    self.program_counter += 1;
                }
                //zero page
                0xA5 => {
                    self.lda(&AddressingMode::ZeroPage)
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
    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

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

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {

        match mode {

            AddressingMode::Immediate => self.program_counter,

            //only zero page (first 256 locations)
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,

            //full memory space (4 bytes)
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),

            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            }
            //same as zeropage, except full range
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            }

            //dereferences from base location + 
            AddressingMode::Indirect_X => {
                let base = self.mem_read_u16(self.program_counter);

                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read((ptr.wrapping_add(1)) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read_u16(self.program_counter);

                let ptr:u8 = (base as u8).wrapping_add(self.register_y);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) | (lo as u16)
            }
            AddressingMode::NoneAddressing => {
                panic!("mode {:?}is not supported", mode);
            }
        }
    }

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

        cpu.load_and_read(instruction_set);

        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & ZERO_FLAG == 0b00);
        assert!(cpu.status & NEGATIVE_FLAG == 0);
    }
    
    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        let instruction_set = vec![0xa9, 0x00, 0x00];

        cpu.load_and_read(instruction_set);
        assert!(cpu.status & ZERO_FLAG == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu=CPU::new();
        let instruction_set = vec![0xa9, 0x0a, 0xaa, 0x00];
        cpu.load_and_read(instruction_set);

        assert_eq!(cpu.register_x, 10);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_0xe8_inx_overflow() {
        let mut cpu = CPU::new();
        let instruction_set = vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00];
        cpu.load_and_read(instruction_set);

        assert_eq!(cpu.register_x, 1);
    }

    #[test]
    fn test_all_5_ops_together() {
        let mut cpu = CPU::new();
        let instruction_set = vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00];
        cpu.load_and_read(instruction_set);
        
        assert_eq!(cpu.register_x, 0xc1)
    }
}