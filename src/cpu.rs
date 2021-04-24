use crate::ram::Ram;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
    stack: Vec<u16>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
            stack: Vec::new(),
        }
    }

    pub fn exec_instruction(&mut self, ram: &mut Ram) {
        let hi = ram.read_byte(self.pc) as u16;
        let lo = ram.read_byte(self.pc + 1) as u16;
        let opcode: u16 = (hi <<  8) | lo;
        println!("LO: {:#x}, HI: {:#x}, PC: {:#x}, INS: {:#x}", lo, hi, self.pc, opcode);
        
        if hi == 0 && lo == 0 {
            panic!();
        }

        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = opcode & 0x000F;
        let x = (opcode & 0x0F00) >> 8;
        let y = (opcode & 0x00F0) >> 4;
        
        match (opcode & 0xF000) >> 12 {
            0x0 => {
                match nn {
                    0xE0 => {
                        // clear display
                        println!("Clear display");
                        self.pc += 2;
                    },
                    0xEE => {
                        // return from subroutine
                        if let Some(addr) = self.stack.pop() {
                            self.pc = addr;
                        }
                    },
                    _ => panic!("Unrecognised 0x00** opcode {:#x} at {:#x}", opcode, self.pc),
                }
            },
            0x1 => {
                // jumps to nnn
                self.pc = nnn;
            },
            0x2 => {
                // calls subroutine at nnn
                self.stack.push(self.pc + 2);
                self.pc = nnn;
            },
            0x3 => {
                // if(Vx==NN)
                let vx = self.read_reg_vx(x);
                if vx == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x6 => {
                // vx = nn
                self.write_reg_vx(x, nn);
                self.pc += 2;
            },
            _ => panic!("Unrecognised opcode {:#x} at {:#x}", opcode, self.pc),
        }

        //self.pc += 2;
    }

    pub fn write_reg_vx(&mut self, index: u16, value: u8) {
        self.vx[index as usize] = value;
    }
    
    pub fn read_reg_vx(&mut self, index: u16) -> u8 {
        self.vx[index as usize]
    }
}
