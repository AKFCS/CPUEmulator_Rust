#[derive(Clone)]
pub struct Register {
    register_a: u8,
    register_b: u8,
    carry_flag: u8,
    pc: u8,
}

impl Default for Register {
    fn default() -> Self {
        Self {
            register_a: u8::default(),
            register_b: u8::default(),
            caryy_flag: u8::default(),
            pc: u8::default(),
        }
    }
}

impl Register {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pc(&self) -> u8 {
        self.pc
    }

    pub fn set_pc(&mut self, new_value: u8) {
        self.pc = new_value;
    }

    pub fn incr_pc(&mut self) {
        self.pc += 1;
    }

    pub fn carry_flag(&self) -> u8 {
        self.carry_flag
    }

    pub fn sef_carry_flag(&mut self, new_value: u8) -> u8 {
        self.carry_flag = new_value;
    }

    pub fn register_a(&self) -> u8 {
        self.register_a
    }

    pub fn set_register_a(&mut self, new_value: u8) {
        self.register_a = new_value;
    }

    pub fn register_b(&self) -> u8 {
        self.register_b
    }

    pub fn set_register_b (&mut self, new_value: u8) {
        self.register_b = new_value;
    }



    

}
