use crate::ram::Ram;
use rand::{thread_rng, Rng};


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
        println!("HI: {:#X}, LO: {:#X}, PC: {:#X}, INS: {:#X}", hi, lo, self.pc, opcode);
        print!("Vx: ");
        for x in 0..self.vx.len() {
            print!("{:#X} ", self.vx[x]);
        }
        println!("");

        if hi == 0 && lo == 0 {
            panic!();
        }

        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        
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
                    _ => panic!("Unrecognised 0x00** opcode {:#X} at {:#X}", opcode, self.pc),
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
            0x4 => {
                // if(Vx!=NN)
                let vx = self.read_reg_vx(x);
                if vx != nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x5 => {
                // if(Vx==Vy)
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);
                if vx == vy {
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
            0x7 => {
                // Vx += NN
                let vx = self.read_reg_vx(x);
                self.write_reg_vx(x, vx.wrapping_add(nn));
                self.pc += 2;
            },
            0x8 => {
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);

                match n {
                    0x0 => {
                        // Vx=Vy
                        self.write_reg_vx(x, vy);
                    },
                    0x1 => {
                        // Vx=Vx|Vy
                        self.write_reg_vx(x, vx | vy);
                    },
                    0x2 => {
                        // Vx=Vx&Vy
                        self.write_reg_vx(x, vx & vy);
                    },
                    0x3 => {
                        // Vx=Vx^Vy
                        self.write_reg_vx(x, vx ^ vy);
                    },
                    0x4 => {
                        // Vx += Vy
                        let sum = vx as u16 + vy as u16;
                        self.write_reg_vx(x, sum as u8);
                        if sum > 0xFF {
                            self.write_reg_vx(0xF, 1);
                        } else {
                            self.write_reg_vx(0xF, 0);
                        }
                    },
                    0x5 => {
                        // Vx -= Vy
                        let diff = vx as i8 - vy as i8;
                        self.write_reg_vx(x, diff as u8);
                        if diff < 0 {
                            self.write_reg_vx(0xF, 0);
                        } else {
                            self.write_reg_vx(0xF, 1);
                        }
                    },
                    0x6 => {
                        // Vx>>=1
                        let vx = self.read_reg_vx(x);
                        self.write_reg_vx(0xF, vx & 0x1);
                        self.write_reg_vx(x, vx >> 1);
                    },
                    0x7 => {
                        // Vx=Vy-Vx
                        let diff = vy as i8 - vx as i8;
                        self.write_reg_vx(x, diff as u8);
                        if diff < 0 {
                            self.write_reg_vx(0xF, 0);
                        } else {
                            self.write_reg_vx(0xF, 1);
                        }
                    },
                    0xE => {
                        // Vx<<=1
                        let vx = self.read_reg_vx(x);
                        self.write_reg_vx(0xF, vx & 0x80);
                        self.write_reg_vx(x, vx << 1);
                    },
                    _ => panic!("Unrecognised 0x8XY* opcode {:#X} at {:#X}", opcode, self.pc),
                }

                self.pc += 2;
            },
            0x9 => {
                // if(Vx!=Vy)
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);
                if vx != vy {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0xA => {
                // I = NNN
                self.i = nnn;
                self.pc += 2;
            },
            0xB => {
                // PC=V0+NNN
                let v0 = self.read_reg_vx(0);
                self.pc = v0 as u16 + nnn;
            },
            0xC => {
                // Vx=rand()&NN
                let mut rng = thread_rng();
                let num = rng.gen_range(0..255);
                self.write_reg_vx(x, num & nn);
                self.pc += 2;
            },
            0xD => {
                // draw(Vx,Vy,N)
                self.debug_draw_sprite(ram, x, y, n);
                self.pc += 2;
            },
            0xF => {
                // I +=Vx
                let vx = self.read_reg_vx(x);
                self.i += vx as u16;
                self.pc += 2;
            },
            _ => panic!("Unrecognised opcode {:#X} at {:#X}", opcode, self.pc),
        }
    }

    pub fn debug_draw_sprite(&mut self, ram: &mut Ram, x: u8, y: u8, height: u8) {
        for a in 0..height {
            let mut b = ram.read_byte(self.i + a as u16);
            for _ in 0..8 {
                match (b & 0b1000_0000) >> 7 {
                    0 => print!("_"),
                    1 => print!("*"),
                    _ => unreachable!(),
                }
                b = b << 1;
            }
            print!("\n");
        }
    }

    pub fn write_reg_vx(&mut self, index: u8, value: u8) {
        self.vx[index as usize] = value;
    }
    
    pub fn read_reg_vx(&mut self, index: u8) -> u8 {
        self.vx[index as usize]
    }
}
