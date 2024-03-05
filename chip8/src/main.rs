mod computer;
mod operation;
mod cpu;
mod memory;
mod frame_buffer;
mod input;
use computer::Chip8Computer;
use operation::Operation;

fn main() {
    let mut emulator_instance = Chip8Computer::new();
    emulator_instance.map_operation_to_function(&Operation::new(0x0000))
}

