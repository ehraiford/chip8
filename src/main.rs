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
use std::time::Instant;

use crate::frame_buffer::GUI;

fn main() {
    let mut gui = GUI::new();
    gui.present();
    // let bytes = read_bytes_from_file(PathBuf::from("/Users/evanraiford/Desktop/code/chip8/chip8/roms/2-ibm-logo.ch8"));
    // let mut emulator_instance = Chip8Computer::new();
    // emulator_instance.load_rom(bytes);

    // let start_time = Instant::now();
    // for _ in 0..1_000_000 {
    //     emulator_instance.tick();
    // }
    // println!(
    //     "Time taken was: {}",
    //     (Instant::now() - start_time).as_secs_f64()
    // );
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
