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
    pub fn push_to_stack(&mut self, value: u16, memory: &mut Memory) {
        if self.stack_pointer >= 32 {
            panic!("Stack overflow!");
        } else {
            let most_significant_byte = (value >> 8) as u8;
            let least_significant_byte = (value & 0x00FF) as u8;

            self.stack_pointer += 1;
            memory.data[memory.data.len() - self.stack_pointer as usize] = least_significant_byte;

            self.stack_pointer += 1;
            memory.data[memory.data.len() - self.stack_pointer as usize] = most_significant_byte;
        }
    }

    pub fn get_top_of_stack(&self, memory: &Memory) -> u16 {
        let stack_offset = memory.data.len() - self.stack_pointer as usize;
        let mut stack_frame: u16 = (memory.data[stack_offset - 1] as u16) << 8;
        stack_frame |= memory.data[stack_offset] as u16;
        stack_frame
    }
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
