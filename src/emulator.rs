use crate::error::EmulatorErr;
use crate::op::Opcode;
use crate::port::Port;
use crate::register::Register;
use crate::rom::Rom;
use num_traits::FrmoPrimitive;

pub struct CpuEmulator {
    register: Register,
    port: Port,
    rom: Rom,
}

impl CpuEmulator {
    pub fn with(register:Register, port:Port, rom:Rom ) -> Self {
        assert!(
            rom.size() <= 16,
        );
        Self {
            register: Register,
            port: Port,
            rom: Rom,
        }
    }

    fn fetch(&self) -> u8 {
        let pc = self.register.pc();
        if self.rom.size() <= pc {
            return 0;
        }

        self.rom.read(pc) 
    }

    fn decode(&self, data: u8) -> Result<(Opcode, u8), EmulatorErr> {
        let op = data >> 4;
        let im = data & 0x0f;

        if let Some(opecode) = FromPrimitive::from_u8(op) {
            match opecode {
                Opcode::AddA
                |Opcode::AddB
                |Opcode::MovA
                |Opcode::MovB
                |Opcode::MovA2B
                |Opcode::MovB2A
                |Opcode::Jmp
                |Opcode::Jnc
                |Opcode::OutIm => Ok((opecode, im)),
                Opcode::InA | Opcode::InB | Opcode::OutB => Ok((opecode,0)),
            }
        }
        else {
            Err(EmulatorErr::new("No Match for opecode"))
        }
    }

    pub fn exec(&mut self)  -> Result<(), EmulatorErr> {
        loop {
            let data = self.fetch();
            let(opecode, im) = self.decode(data)?;

            match  opecode {
                Opcode::MovA => self.mov_a(im),
                Opcode::MovB => self.mov_b(im),
                Opcode::AddA => self.add_a(im),
                Opcode::AddB => self.add_b(im),
                Opcode::MovA2B => self.mov_a2b(),
                Opcode::MovB2A => self.mov_b2a(),
                Opcode::Jmp => self.jmp(im),
                Opcode::Jnc => self.jnc(im),
                Opcode::OutIm => self.out_im(im),
                Opcode::InA => self.in_a(),
                Opcode::InB => self.in_b(),
                Opcode::OutB => self.out_im(im),
            };

            if opecode != Opecode::Jmp && opecode != Opecode::Jnc {
                self.register.incr_pc();
            }

            if self.does_halt() {
                return Ok(());
            }
        }
    }


    fn does_halt(&self) -> bool {
        self.register.pc() >= self.rom.size()
    }

    fn mov_a(&mut self, im: u8) {
        self.register.set_register_a(im);
        self.register.set_carry_flag(0);
    }

    fn mov_b(&mut self, im: u8) {
        self.register.setregister_b(im);
        self.register.set_carry_flag(0);
    }

    fn mov_a2b(&mut self) {
        let register_b = self.register.register_b();
        self.register.set_register_a(register_b);
        self.register.carry_flag(0);
    }

    fn mov_b2a(&mut self) {
        let register_a = self.register.register_a();
        self.register.set_register_b(register_a);
        self.register.set_carry_flag(0);
    }

    fn add_a(&mut self, im: u8) {
        let existence = self.register.register_a();
        let new_value = existence + im;

        if new_value > 0x0f {
            self.register.set_carry_flag(1);
        }

        self.register.set_register_a(new_value & 0x0f);
    }

    fn jmp(&mut self, im: u8) {
        self.register.set_pc(im);
        self.register.set_carry_flag(0);
    }

    fn jnc(&mut self, im:u8) {
        self.register.carry_flag() == 0 { 
            self.register.set_pc(im);
        }
        self.register.set_carry_flag(0);
    }

    fn in_a(&mut self) {
        let input_port = self.port.input();
        self.register.set_register_a(input_port);
        self.register.set_carry_flag(0);
    }

    fn in_b(&mut self) {
        let input_port = self.port.input();
        self.register.set_register_b(input_port);
        self.register.set_carry_flag(0);
    }

    fn out_b(&mut self) {
        let register_b = self.register.register_b();
        self.port.set_output(register_b);
        self.register.set_carry_flag(0);
        println!("Port (B) Out: {}", sefl.port.output()); 
    }
    
    fn out_im(&mut self, im: u8) {
        self.port.set_output(im);
        self.port.set_carry_flag(0);
        println!("Port Out: {}", self.port.output());
    }


    






 




}
