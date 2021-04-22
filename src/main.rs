mod chip8;
mod ram;

use std::fs::File;
use std::io::Read;

use chip8::Chip8;

fn main() -> std::io::Result<()> {
    let mut file = File::open("roms/INVADERS")?;
    let mut game_data = Vec::<u8>::new();
    
    file.read_to_end(&mut game_data)?;

    let mut chip8 = Chip8::new();
    chip8.load_rom(&game_data);
    println!("Chip8: {:?}", chip8);
    Ok(())
}
