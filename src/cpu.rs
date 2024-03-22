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
        if self.stack_pointer >= 32 {
            panic!("Stack overflow!");
        } else {
            let most_significant_byte = (value >> 8) as u8;
            let least_significant_byte = (value & 0x00FF) as u8;
            
            memory.data[self.stack_pointer as usize] = most_significant_byte;
            self.stack_pointer += 1;

            memory.data[self.stack_pointer as usize] = least_significant_byte;
            self.stack_pointer += 1;
        }
    }

    pub fn pop_stack(&mut self, memory: &mut Memory) -> u16 {
        self.stack_pointer -= 2;
        let stack_frame = self.get_top_of_stack(memory);
        stack_frame
    }

    fn get_top_of_stack(&self, memory: &Memory) -> u16 {
        let most_sig_byte = memory.data[self.stack_pointer as usize];
        let least_sig_byte = memory.data[self.stack_pointer as usize + 1];
        
        let stack_frame: u16 = ((most_sig_byte as u16) << 8) | least_sig_byte as u16;
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
                let register_start = format!("V{}:", (i*4 + j)); 
                string.push_str(&format!("[{:<width$} {:03}]    ", register_start, register, width=4));
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
