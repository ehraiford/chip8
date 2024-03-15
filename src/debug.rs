use lazy_static::lazy_static;
use crate::Instruction;
use std::{convert::TryInto, sync::Mutex, time::Duration};

///An optional struct that contains useful data for debugging the emulator.
struct DebugData {
    instruction_counter: usize,
    average_cycle_time: Duration,
    last_instructions: (Vec<Instruction>, u32),
}

impl DebugData {
    pub fn new(instruction_buffer_len: u32) -> Self {
        DebugData {
            last_instructions: (Vec::new(), instruction_buffer_len),
            instruction_counter: 0,
            average_cycle_time: Duration::new(0, 0),
        }
    }

    ///Pushes the provided instruction into the last_iunstructions vec.
    ///
    /// If the vec is already at the specified capacity, the oldest instruction is popped out.
    fn push_instruction(&mut self, instruction: Instruction) {
        if self.last_instructions.0.len() == self.last_instructions.1.try_into().unwrap() {
            self.last_instructions.0.pop();
        }
        self.last_instructions.0.push(instruction);
    }

    /// Updates the debug data stored in the struct.
    ///
    /// This is incrementing the instruction count, updating the last_instruction vec, and recalculating the average cycle time.
    pub fn update_debug_data(&mut self, last_instruction: Instruction, last_duration: Duration) {
        self.push_instruction(last_instruction);

        let mut total_duration =
            self.average_cycle_time * self.instruction_counter.try_into().unwrap();
        total_duration += last_duration;
        self.instruction_counter += 1;
        self.average_cycle_time = total_duration / self.instruction_counter.try_into().unwrap();
    }
}

impl Default for DebugData {
    fn default() -> Self {
        DebugData::new(10)
    }
}

impl std::fmt::Display for DebugData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


lazy_static!{
    static ref DEBUG_DATA: Mutex<DebugData> = Mutex::new(DebugData::default());
}