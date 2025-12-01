pub struct CPU {
    pub register_a: u8,
    pub register_b: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_b: 0,
            status: 0,
            program_counter: 0,
        }
    }

    
}