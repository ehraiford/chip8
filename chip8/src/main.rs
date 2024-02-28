fn main() {
    let emulator_instance = Chip8Computer::new();


}

struct Chip8Computer {
    cpu: Cpu,
    memory: Memory,
    frame_buffer: FrameBuffer,
}

impl Chip8Computer {
    pub fn new() -> Self {
        Chip8Computer {
            cpu: Cpu::new(),
            memory: Memory::new(),
            frame_buffer: FrameBuffer::new(),
        }
    }

    fn map_operation_to_function(&mut self, operation: &Operation) {
        match operation.get_opcode() {
            0x0 => {
                match operation.value {
                    0x00E0 => {
                        self.clear_frame_buffer();
                    },
                    0x00EE => {
                        self.return_subroutine();
                    },
                    _ => {
                        panic!("sys addr is not implemented in this emulator.");
                    }
                }
            },
            0x1 => {
                self.jump();
            },
            0x2 => {
                self.call();
            },
            0x3 => {
                self.skip_equal_immediate();
            },
            0x4 => {
                self.skip_not_equal_immediate();
            },
            0x5 => {
                self.set_equal_register();
            },
            0x6 => {
                self.load_immediate();
            },
            0x7 => {
                self.add();
            }

            _ => {
                panic!("This operation hasn't been added yet.");
            }
        }
    }

    ///*CLS*:
    ///Clears the display
    fn clear_frame_buffer(&mut self) {
        self.frame_buffer.clear();
    }
    ///*RET*:
    ///Returns from the subroutine
    fn return_subroutine(&mut self) {
        self.cpu.program_counter = self.cpu.get_top_of_stack(&self.memory)
    }
    ///*JP*:
    ///Jumps to the specified address.
    fn jump(&self) {
        todo!()
    }
    ///*CALL*:
    ///Calls a subroutine at the specified address
    fn call(&self) {
        todo!()
    }
    ///*SE*:
    ///Skips the next instruction if the value in the specified register equals the specified value
    ///0x3xkk: Skips next instruction if Vx == kk.
    fn skip_equal_immediate(&self) {
        todo!()
    }
    ///*SNE*:
    ///Skips the next instruction if the value in the specified register does not equal the specified value
    ///0x4xkk: Skips next instruction if Vx != kk.
    fn skip_not_equal_immediate(&self) {
        todo!()
    }
    ///*SE*:
    ///Skips the next instruction if the values in the two specified registers are not equal
    ///0x5xy0: Skips next instruction if Vx == Vy.
    fn set_equal_register(&self) {
        todo!()
    }
    ///*LD&: 
    ///Loads the given value into the specified register.
    ///0x6xkk: Vx = kk.
    fn load_immediate(&self) {
        todo!()
    }
    ///*ADD*:
    ///Adds the given value to the specified register
    ///0x7xkk Vx = Vx + kk.
    fn add(&self) {
        todo!()
    }
    /// *LD*:
    ///Moves the value within the second specified register into the first.
    ///0x8xy0 Vx = Vy.
    fn load_register(&self) {
        todo!()
    }
}
struct Cpu {
    data_registers: [u8; 16],
    index_register: u16,
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,

}
impl Cpu {
    pub fn new() -> Self {
        Cpu {
            data_registers: [0; 16],
            index_register: 0,
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0,
        }
    }

    pub fn push_to_stack(&mut self, value: u8, memory: &mut Memory) {
        if self.stack_pointer >= 64 {
            panic!("Stack overflow!");
        } else {
            memory.data[memory.data.len() - self.stack_pointer as usize] = value;
            self.stack_pointer += 1;
        }
    }

    pub fn get_top_of_stack(&self, memory: &Memory) -> u16 {
        let stack_offset = memory.data.len() - self.stack_pointer as usize;
        let mut stack_frame: u16 = (memory.data[stack_offset - 1]as u16) << 8;
        stack_frame |= memory.data[stack_offset] as u16;
        stack_frame
    }

}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

struct FrameBuffer {
    buffer: [[u8; 4]; 8]
}

impl FrameBuffer {
    fn new() -> Self {
        FrameBuffer {
            buffer: [[0; 4]; 8],
        }
    }

    fn get_frame_buffer(&self) -> [[u8; 4]; 8] {
        self.buffer
    }

    fn set_bit(&mut self, x: u64, y: u32, turn_on: bool) {
    }

    fn clear(&mut self) {
        for mut line in self.buffer {
            for mut chunk in line {
                chunk = 0;
            }
        }
    }
}

struct Memory {
    data: [u8; 4096]
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; 4096],
        }
    }
}

struct Operation {
    value: u16
}

impl Operation {
    ///Returns the 4 leftmost bits of an operation's value.
    ///Value is in the 4 right bits of the return value
    fn get_opcode(&self) -> u8 {
        (self.value >> 12) as u8
    }
}