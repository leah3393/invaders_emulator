use crate::condition_codes::ConditionCodes;
use crate::registers::Register;

#[derive(Default)]
pub struct State {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: usize,
    pub sp: usize,
    pub memory: Vec<u8>,
    pub cc: ConditionCodes,
    pub int_enable: u8
}

impl State {
    pub fn set_8bit_reg(&mut self, reg: &Register, value: u8){
        match reg {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
            Register::M => {
                let mem = (((self.h as u16) << 8) | self.l as u16) as usize;
                self.memory[mem] = value;
            },
            _ => ()
        };
    }

    pub fn push_return_addr(&mut self, ret: u16){
        self.memory[self.sp-1] = ((ret >> 8) & 0xff) as u8;
        self.memory[self.sp-2] = (ret & 0xff) as u8;
        self.sp -= 2;
    }

    pub fn pop_return_addr(&mut self) -> u16 {
        let addr1 = self.memory[self.sp-1];
        let addr2 = self.memory[self.sp-2];

        let ret = ((addr1 as u16) << 8) | addr2 as u16;

        self.sp += 2;

        ret
    }

    pub fn move_reg_to_reg(&mut self, reg_to: &Register, reg_from: &Register){
        let value = match reg_from {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            Register::M => {
                let mem = (((self.h as u16) << 8) | self.l as u16) as usize;
                self.memory[mem]
            },
            _ => 0
        };

        self.set_8bit_reg(reg_to, value);
    }

    pub fn debug(&self, instruction: String) {
        println!("*********************");
        println!("Executing: {} @ {:04x}",instruction, self.pc);
        println!("Registers: [a: {:02x}, b, {:02x}, c: {:02x}, d: {:02x}, e: {:02x}, h: {:02x}, l: {:02x}]", self.a, self.b, self.c, self.d, self.e, self.h, self.l);
        println!("Stack Pointer: {:04x}", self.sp);
        println!("*********************");
    }
}