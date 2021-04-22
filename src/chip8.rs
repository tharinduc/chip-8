use crate::ram::Ram;
use crate::cpu::{Cpu, PROGRAM_START};

pub struct Chip8 {
    ram: Ram,
    cpu: Cpu
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            ram: Ram::new(),
            cpu: Cpu::new()
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for i in 0..data.len() {
            self.ram.write_byte(PROGRAM_START + i as u16, data[i]); 
        }
    }

    pub fn exec_instruction(&mut self) {
        self.cpu.exec_instruction(&mut self.ram);
    }
}

