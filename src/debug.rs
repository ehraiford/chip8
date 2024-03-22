use lazy_static::lazy_static;
use std::panic;
use std::sync::Once;
use std::time::Instant;
use std::{convert::TryInto, sync::Mutex, time::Duration};
use crate::Instruction;

///An optional struct that contains useful data for debugging the emulator.
pub struct DebugData {
    instruction_counter: usize,
    average_cycle_time: Duration,
    last_instructions: (Vec<Instruction>, u32),
    last_executed_at: Instant,
}

impl DebugData {
    pub fn new(instruction_buffer_len: u32) -> Self {
        //std::env::set_var("RUST_BACKTRACE", "1");
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            panic::set_hook(Box::new(|panic_info| {
                panic_with_debug_details(panic_info);
            }));
        }); 
        DebugData {
            last_instructions: (Vec::new(), instruction_buffer_len),
            instruction_counter: 0,
            average_cycle_time: Duration::new(0, 0),
            last_executed_at: Instant::now()
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
    pub fn update_debug_data(&mut self, last_instruction: Instruction) {
        self.push_instruction(last_instruction);
        
        let current_time = Instant::now();
        let last_duration = Duration::from(current_time - self.last_executed_at);
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
        // let clear_command = match cfg!(windows) {
        //     true => "cls".to_string(),
        //     false => "clear".to_string(),
        // };
        // let _ = std::process::Command::new(&clear_command).status();
        let mut string = "DEBUG DATA:\n".to_string();
        string.push_str(&format!("INSTRUCTION COUNTER: {}\n", self.instruction_counter));
        string.push_str(&format!("EXECUTION SPEED: {} Hz\n", 1.0 / self.average_cycle_time.as_secs_f32()));
        string.push_str(&format!("LAST {} INSTRUCTIONS: [ ", self.last_instructions.1));
        for instruction in &self.last_instructions.0 {
            string.push_str(&format!("{} ", instruction));
        }
        string.push(']');

        write!(f, "{string}")
    }
}

lazy_static!{
    pub static ref DEBUG_DATA: Mutex<DebugData> = Mutex::new(DebugData::default());
}

fn panic_with_debug_details(panic_info: &panic::PanicInfo){
    let debug = DEBUG_DATA.lock().unwrap();
    println!("{}", debug);

    println!("{}", panic_info);
}