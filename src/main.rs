mod computer;
mod cpu;
mod frame_buffer;
mod input;
mod memory;
mod operation;
use computer::Chip8Computer;
use operation::Operation;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let bytes = read_bytes_from_file(PathBuf::from("/Users/evanraiford/Desktop/code/chip8/roms/2-ibm-logo.ch8"));
    let mut emulator_instance = Chip8Computer::new();
    emulator_instance.load_rom(bytes);

    for _ in 0..50 {
        emulator_instance.execute_loop();
    }
}

fn read_bytes_from_file(file_path: PathBuf) -> Vec<u8> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => panic!("Error opening the file: {e}."),
    };

    let mut byte_buffer = Vec::new();
    if let Err(e) = file.read_to_end(&mut byte_buffer) {
        panic!("Error reading the file: {e}.")
    };

    return byte_buffer;
}
