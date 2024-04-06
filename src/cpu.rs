use std::fmt::Display;

use crate::memory::Memory;

pub struct Cpu {
    pub data_registers: [u8; 16],
    pub index_register: u16,
    pub stack_pointer: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub program_counter: u16,
}
impl Cpu {
    pub fn new() -> Self {
        Cpu {
            data_registers: [0; 16],
            index_register: 0,
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0x200,
        }
    }

    ///Pushes the given value to the stack and increments the stack pointer
    pub fn push_stack(&mut self, value: u16, memory: &mut Memory) {
        if self.stack_pointer >= 16 {
            panic!("Stack overflow!");
        } else {
            memory.stack[self.stack_pointer as usize] = value;
            self.stack_pointer += 1;
        }
    }

    pub fn pop_stack(&mut self, memory: &mut Memory) -> u16 {
        self.stack_pointer -= 1;
        let stack_frame = memory.stack[self.stack_pointer as usize];
        stack_frame
    }
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = "CPU: \n".to_string();
        string.push_str(&format!("PROGRAM COUNTER: {}\n", self.program_counter));
        string.push_str(&format!("STACK POINTER: {}\n", self.stack_pointer));
        for (i, chunk) in self.data_registers.chunks(4).into_iter().enumerate() {
            for (j, register) in chunk.iter().enumerate() {
                let register_start = format!("V{}:", (i * 4 + j));
                string.push_str(&format!(
                    "[{:<width$} {:03}]    ",
                    register_start,
                    register,
                    width = 4
                ));
            }
            string.push('\n');
        }
        string.push_str(&format!("INDEX REGISTER: {}\n", self.index_register));
        string.push_str(&format!("DELAY TIMER: {}\n", self.delay_timer));
        string.push_str(&format!("SOUND TIMER: {}\n", self.sound_timer));

        write!(f, "{}", string)
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
