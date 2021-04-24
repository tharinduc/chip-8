mod chip8;
mod ram;
mod cpu;

use std::fs::File;
use std::io::Read;

use chip8::Chip8;

fn main() -> std::io::Result<()> {
    let mut file = File::open("roms/INVADERS")?;
    let mut rom_data = Vec::<u8>::new();
    
    file.read_to_end(&mut rom_data)?;

    let mut chip8 = Chip8::new();
    chip8.load_rom(&rom_data);
//    println!("Chip8: {:?}", chip8);

    loop {
        chip8.exec_instruction();
    }

}
