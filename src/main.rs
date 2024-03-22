mod computer;
mod cpu;
mod debug;
mod frame_buffer;
mod input;
mod instruction;
mod memory;
use computer::Chip8Computer;
use instruction::Instruction;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
extern crate lazy_static;


fn main() {
    let roms: [String; 5] = [
        "roms/1-chip8-logo.ch8".to_string(),
        "roms/2-ibm-logo.ch8".to_string(),
        "roms/3-corax+.ch8".to_string(),
        "roms/4-flags.ch8".to_string(),
        "roms/c8_test.c8".to_string(),
     ];
    let bytes = read_bytes_from_file(PathBuf::from(
        roms[3].clone()
    ));
    let mut emulator_instance = Chip8Computer::new();
    emulator_instance.load_rom(bytes);

    for _ in 0..10000 {       
        emulator_instance.execute_loop();
    }
}

fn read_bytes_from_file(file_path: PathBuf) -> Vec<u8> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => panic!("Error opening the file: {}.", e),
    };

    let mut byte_buffer = Vec::new();
    if let Err(e) = file.read_to_end(&mut byte_buffer) {
        panic!("Error reading the file: {}.", e)
    };

    return byte_buffer;
}
