use crate::error::EmulatorErr;
use crate::op::Opecode;
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
            register,
            port,
            rom,
        }
    }

    fn fetch(&self) -> u8 {
        let pc = self.register.pc();
        if self.rom.size() <= pc {
            return 0;
        }

        self.rom.read(pc) 
    }

    fn decode(&self, data: u8) -> Result<(Opecode, u8), EmulatorErr> {
        let op = data >> 4;
        let im = data & 0x0f;

        if let Some(opecode) = FromPrimitive::from_u8(op) {
            match opecode {
                Opecode::AddA
                |Opecode::AddB
                |Opecode::MovA
                |Opecode::MovB
                |Opecode::MovA2B
                |Opecode::Jmp
                |Opecode::Jnc
                |Opecode::OutIm => Ok(opecode, im),
                Opecode::InA | Opecode::InB | Opecode::OutB => Ok((opecode,0)),
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
                Opecode::MovA => self.mov_a(im),
                Opecode::MovB => self.mov_b(im),
                Opecode::AddA => self.add_a(im),
                Opecode::AddB => self.add_b(im),
                Opecode::MovA2B => self.mov_a2b(),
                Opecode::
                Opecode::
                Opecode::
                Opecode::
                Opecode::
                Opecode::
                Opecode::
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

    fn mov_a2b(&mut self) {
        
    }




}
