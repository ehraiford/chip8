use crate::cpu::Cpu;
use crate::frame_buffer::FrameBuffer;
use crate::input::Input;
use crate::memory::Memory;
use crate::Instruction;
use std::fmt::Display;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct Chip8Computer {
    pub cpu: Cpu,
    pub memory: Memory,
    pub frame_buffer: FrameBuffer,
    pub input: Input,
    response_sender: Sender<EmulatorResponse>,
    clock_speed_hz: u16,
}

impl Chip8Computer {
    pub fn new(response_sender: Sender<EmulatorResponse>) -> Self {
        Chip8Computer {
            cpu: Cpu::new(),
            memory: Memory::new(),
            frame_buffer: FrameBuffer::new(response_sender),
            response_sender,
            input: Input::new(),
            clock_speed_hz: 10,
        }
    }

    pub fn execute_loop(&mut self) {
        let instruction = self.tick();

        #[cfg(feature = "debug")]
        {
            let mut debug = crate::debug::DEBUG_DATA.lock().unwrap();
            debug.update_debug_data(instruction);
            println!("{}", self);
            println!("{}", debug);
        }
    }

    pub fn tick(&mut self) -> Instruction {
        let instruction = self
            .memory
            .read_instruction(self.cpu.program_counter)
            .into();
        self.cpu.program_counter += 2;
        self.map_operation_to_function(&instruction);

        self.cpu.sound_timer = self.cpu.sound_timer.saturating_sub(0);
        self.cpu.delay_timer = self.cpu.delay_timer.saturating_sub(0);

        instruction
    }

    pub fn map_operation_to_function(&mut self, operation: &Instruction) {
        match operation.get_opcode() {
            0x0 => match operation.value {
                0x00E0 => {
                    self.clear_frame_buffer();
                }
                0x00EE => {
                    self.return_subroutine();
                }
                _ => {
                    panic!(
                        "sys addr (0x{:04X}) is not implemented in this emulator.",
                        operation.value
                    );
                }
            },
            0x1 => {
                self.jump(operation);
            }
            0x2 => {
                self.call(operation);
            }
            0x3 => {
                self.skip_equal_immediate(operation);
            }
            0x4 => {
                self.skip_not_equal_immediate(operation);
            }
            0x5 => {
                self.skip_equal_register(operation);
            }
            0x6 => {
                self.load_immediate(operation);
            }
            0x7 => {
                self.add_immediate(operation);
            }
            0x8 => match operation.value & 0x000F {
                0x0 => {
                    self.move_register(operation);
                }
                0x1 => {
                    self.or_register(operation);
                }
                0x2 => {
                    self.and_register(operation);
                }
                0x3 => {
                    self.xor_register(operation);
                }
                0x4 => {
                    self.add_register(operation);
                }
                0x5 => {
                    self.subtract_register(operation);
                }
                0x6 => {
                    self.shift_right(operation);
                }
                0x7 => {
                    self.subtract_register_not(operation);
                }
                0xE => {
                    self.shift_left(operation);
                }
                _ => {
                    println!("Instruction failed: 0x{:0x}", operation.value);
                    self.memory.print_rom();
                    panic!("Unsupported value!");
                }
            },
            0x9 => match operation.value & 0x000F {
                0x00 => {
                    self.skip_not_equal_register(operation);
                }
                _ => {
                    println!("Instruction failed: 0x{:0x}", operation.value);
                    self.memory.print_rom();
                    panic!("Unsupported value!");
                }
            },
            0xA => {
                self.load_address(operation);
            }
            0xB => {
                self.jump_register(operation);
            }
            0xC => {
                self.random(operation);
            }
            0xD => {
                self.draw(operation);
            }
            0xE => match operation.value & 0x00FF {
                0x9E => {
                    self.skip_pressed(operation);
                }
                0xA1 => {
                    self.skip_not_pressed(operation);
                }
                _ => {
                    println!("Instruction failed: 0x{:0x}", operation.value);
                    self.memory.print_rom();
                    panic!("Unsupported value!");
                }
            },
            0xF => match operation.value & 0x00FF {
                0x07 => {
                    self.load_delay(operation);
                }
                0x0A => {
                    self.load_keypress(operation);
                }
                0x15 => {
                    self.store_delay(operation);
                }
                0x18 => {
                    self.store_sound(operation);
                }
                0x1E => {
                    self.add_index(operation);
                }
                0x29 => {
                    self.index_sprite(operation);
                }
                0x33 => {
                    self.store_bcd(operation);
                }
                0x55 => {
                    self.store_registers(operation);
                }
                0x65 => {
                    self.load_registers(operation);
                }
                _ => {
                    println!("Instruction failed: 0x{:0x}", operation.value);
                    panic!("Unsupported value!");
                }
            },
            _ => {
                println!("Instruction failed: 0x{:0x}", operation.value);

                panic!("Unsupported value!");
            }
        }
    }

    ///*CLS*:
    ///Clears the display
    pub fn clear_frame_buffer(&mut self) {
        self.frame_buffer.clear();
    }
    ///*RET*:
    ///Returns from the subroutine
    pub fn return_subroutine(&mut self) {
        self.cpu.program_counter = self.cpu.pop_stack(&mut self.memory);
    }
    ///*JP*:
    ///Jumps to the specified address.
    pub fn jump(&mut self, operation: &Instruction) {
        self.cpu.program_counter = operation.get_address_immediate();
    }
    ///*CALL*:
    ///Calls a subroutine at the specified address
    ///0x2nnn: Puts current PC on the stack and sets PC to nnn.
    pub fn call(&mut self, operation: &Instruction) {
        self.cpu
            .push_stack(self.cpu.program_counter, &mut self.memory);
        self.cpu.program_counter = operation.get_address_immediate();
    }
    ///*SE*:
    ///Skips the next instruction if the value in the specified register equals the specified value
    ///0x3xkk: Skips next instruction if Vx == kk.
    pub fn skip_equal_immediate(&mut self, operation: &Instruction) {
        let immediate_value = operation.get_immediate();
        let register_value = self.cpu.data_registers[operation.get_register() as usize];
        if immediate_value == register_value {
            self.cpu.program_counter += 2;
        }
    }
    ///*SNE*:
    ///Skips the next instruction if the value in the specified register does not equal the specified value
    ///0x4xkk: Skips next instruction if Vx != kk.
    pub fn skip_not_equal_immediate(&mut self, operation: &Instruction) {
        let immediate_value = operation.get_immediate();
        let register_value = self.cpu.data_registers[operation.get_register() as usize];

        if immediate_value != register_value {
            self.cpu.program_counter += 2;
        }
    }
    ///*SE*:
    ///Skips the next instruction if the values in the two specified registers are equal
    ///0x5xy0: Skips next instruction if Vx == Vy.
    pub fn skip_equal_register(&mut self, operation: &Instruction) {
        let register_one_value = self.cpu.data_registers[operation.get_register() as usize];
        let register_two_value = self.cpu.data_registers[operation.get_second_register() as usize];

        if register_one_value == register_two_value {
            self.cpu.program_counter += 2;
        }
    }
    ///*LD&:
    ///Loads the given value into the specified register.
    ///0x6xkk: Vx = kk.
    pub fn load_immediate(&mut self, operation: &Instruction) {
        self.cpu.data_registers[operation.get_register() as usize] = operation.get_immediate();
    }
    ///*ADD*:
    ///Adds the given value to the specified register
    ///0x7xkk Vx = Vx + kk.
    pub fn add_immediate(&mut self, operation: &Instruction) {
        let immediate = operation.get_immediate();
        let register_value = self.cpu.data_registers[operation.get_register() as usize];
        let result = register_value.overflowing_add(immediate).0;
        self.cpu.data_registers[operation.get_register() as usize] = result;
    }
    /// *LD*:
    ///Moves the value within the second specified register into the first.
    ///0x8xy0: Vx = Vy.
    pub fn move_register(&mut self, operation: &Instruction) {
        let moved_value = self.cpu.data_registers[operation.get_second_register() as usize];
        self.cpu.data_registers[operation.get_register() as usize] = moved_value;
    }
    /// *OR*:
    ///Bitwise ors Vx and Vy and stores the result in Vx
    ///0x8xy1: Vx |= Vy.
    pub fn or_register(&mut self, operation: &Instruction) {
        let or_value = self.cpu.data_registers[operation.get_second_register() as usize];
        self.cpu.data_registers[operation.get_register() as usize] |= or_value;
    }
    /// *AND*:
    ///Bitwise ands Vx and Vy and stores the result in Vx
    ///0x8xy2: Vx &= Vy.
    pub fn and_register(&mut self, operation: &Instruction) {
        let and_value = self.cpu.data_registers[operation.get_second_register() as usize];
        self.cpu.data_registers[operation.get_register() as usize] &= and_value;
    }
    /// *XOR*:
    ///Bitwise xors Vx and Vy and stores the result in Vx
    ///0x8xy3: Vx ^= Vy.
    pub fn xor_register(&mut self, operation: &Instruction) {
        let xor_value = self.cpu.data_registers[operation.get_second_register() as usize];
        self.cpu.data_registers[operation.get_register() as usize] ^= xor_value;
    }
    /// *ADD*:
    ///Adds the contents of two registers together and stores the result in the first
    ///Sets VF to whether or not there is a carry.
    ///0x8xy4: Vx += Vy.
    pub fn add_register(&mut self, operation: &Instruction) {
        let first_value = self.cpu.data_registers[operation.get_register() as usize];
        let second_value = self.cpu.data_registers[operation.get_second_register() as usize];
        let result = first_value.overflowing_add(second_value);
        self.cpu.data_registers[operation.get_register() as usize] = result.0;
        self.cpu.data_registers[0xF] = result.1 as u8;
    }
    /// *SUB*:
    ///Subtracts the content of register 2 from register 1 and stores the result in the first
    ///Sets VF to the OPPOSITE of whether or not there is a carry.
    ///0x8xy5: Vx -= Vy.
    pub fn subtract_register(&mut self, operation: &Instruction) {
        let first_value = self.cpu.data_registers[operation.get_register() as usize];
        let second_value = self.cpu.data_registers[operation.get_second_register() as usize];
        let result = first_value.overflowing_sub(second_value);
        self.cpu.data_registers[operation.get_register() as usize] = result.0;
        self.cpu.data_registers[0xF] = !result.1 as u8;
    }
    /// *SHR*:
    ///Shifts the value in a register right by one and stores the result. VF is set to the bit that was consumed.
    ///0x8xy6: Vx = Vx >> 1.
    pub fn shift_right(&mut self, operation: &Instruction) {
        let second_value = self.cpu.data_registers[operation.get_second_register() as usize];

        self.cpu.data_registers[operation.get_register() as usize] = second_value;
        let last_bit = self.cpu.data_registers[operation.get_register() as usize] & 0x01;
        self.cpu.data_registers[operation.get_register() as usize] >>= 1;
        self.cpu.data_registers[0xF] = (last_bit != 0) as u8;
    }
    /// *SUBN*:
    ///Subtracts the content of register 1 from register 2 and stores the result in the first.
    ///0x8xy7 Vx = Vy - Vx.
    pub fn subtract_register_not(&mut self, operation: &Instruction) {
        let first_value = self.cpu.data_registers[operation.get_register() as usize];
        let second_value = self.cpu.data_registers[operation.get_second_register() as usize];
        let result = second_value.overflowing_sub(first_value);
        self.cpu.data_registers[operation.get_register() as usize] = result.0;
        self.cpu.data_registers[0xF] = !result.1 as u8;
    }
    /// *SHL*:
    ///Shifts the value in a register left by one and stores the result. VF is set to the bit that was consumed.
    ///0x8xyE: Vx = Vx << 1.
    pub fn shift_left(&mut self, operation: &Instruction) {
        let second_value = self.cpu.data_registers[operation.get_second_register() as usize];

        self.cpu.data_registers[operation.get_register() as usize] = second_value;
        let first_bit = self.cpu.data_registers[operation.get_register() as usize] & 0x80;
        self.cpu.data_registers[operation.get_register() as usize] <<= 1;
        self.cpu.data_registers[0xF] = (first_bit != 0) as u8;
    }
    /// *SNE*:
    ///Skips the next instruction if the values in the two given registers are not equal
    ///0x9xy0: Skips next instruction if Vx != Vy.
    pub fn skip_not_equal_register(&mut self, operation: &Instruction) {
        let first_value = self.cpu.data_registers[operation.get_register() as usize];
        let second_value = self.cpu.data_registers[operation.get_second_register() as usize];

        if second_value != first_value {
            self.cpu.program_counter += 2;
        }
    }
    ///*LD I*:
    ///Loads given large immediate value into I Register
    ///0xAnnn: I = nnn.
    pub fn load_address(&mut self, operation: &Instruction) {
        self.cpu.index_register = operation.get_address_immediate();
    }
    ///*JP I*:
    ///Sets PC to nnn + V0
    ///0xBnnn: PC = nnn + V0.
    pub fn jump_register(&mut self, operation: &Instruction) {
        let jump_address = self.cpu.data_registers[0] as u16 + operation.get_address_immediate();
        self.cpu.program_counter = jump_address;
    }
    /// *RND*:
    ///Generates a random 8-bit value, ANDs it with an immediate, and stores the result in Vx.
    ///0xCxkk: Vx = 0x?? + kk.
    pub fn random(&mut self, operation: &Instruction) {
        let mut rng = rand::thread_rng();
        let random_number: u8 = rand::Rng::gen_range(&mut rng, 0..255);
        let result = random_number & operation.get_immediate();
        self.cpu.data_registers[operation.get_register() as usize] = result;
    }
    /// *DRW*:
    ///Reads n bytes from memory starting at the address in Register I and displays them starting at (Vx, Vy).
    ///Sprites are XORed onto the screen with existing pixels. VF is set to whether any pixels are erased because of this.
    ///0xDxyn
    pub fn draw(&mut self, operation: &Instruction) {
        let num_bytes = operation.get_small_immediate();
        let starting_address = self.cpu.index_register;
        let draw_bytes = self.memory.read_bytes(starting_address, num_bytes.into());
        let start_x = self.cpu.data_registers[operation.get_register() as usize];
        let start_y = self.cpu.data_registers[operation.get_second_register() as usize];

        let result = self.frame_buffer.draw_sprite(start_x, start_y, draw_bytes);
        self.cpu.data_registers[0x0F] = result.into();
    }
    /// *SKP*:
    ///Skips the next instruction if the key corresponding to the value in Vx is pressed.
    ///Ex9E
    pub fn skip_pressed(&mut self, operation: &Instruction) {
        let key_value = self.cpu.data_registers[operation.get_register() as usize];
        if self.input.check_pressed(key_value) {
            self.cpu.program_counter += 2;
        }
    }
    /// *SKNP*:
    ///Skips the next instruction if the key corresponding to the value in Vx is not pressed.
    ///ExA1
    pub fn skip_not_pressed(&mut self, operation: &Instruction) {
        let key_value = self.cpu.data_registers[operation.get_register() as usize];
        if !self.input.check_pressed(key_value) {
            self.cpu.program_counter += 2;
        }
    }
    /// *LD*:
    ///Loads the value from the delay timer and stores it in Vx
    ///0xFx07 Vx = Delay Timer.
    pub fn load_delay(&mut self, operation: &Instruction) {
        self.cpu.data_registers[operation.get_register() as usize] = self.cpu.delay_timer;
    }
    /// *LD*:
    ///Stores the value of the next keypress in Vx. Execution stops until then.
    ///0xFx0A: Vx = Keypress.
    pub fn load_keypress(&mut self, operation: &Instruction) {
        self.cpu.data_registers[operation.get_register() as usize] = self.input.receive_input();
    }
    /// *LD*:
    ///Sets delay timer to value within specified register.
    ///0xFx15: Delay Timer = Vx.
    pub fn store_delay(&mut self, operation: &Instruction) {
        self.cpu.delay_timer = self.cpu.data_registers[operation.get_register() as usize];
    }
    /// *LD ST*:
    ///Sets sound timer to value within specified register.
    ///0xFx18: Sound Timer = Vx.
    pub fn store_sound(&mut self, operation: &Instruction) {
        self.cpu.sound_timer = self.cpu.data_registers[operation.get_register() as usize];
    }
    /// *ADD I, Vx*:
    ///Adds I with the value in Vx and stores it in I.
    ///0xFx1E: I += Vx.
    pub fn add_index(&mut self, operation: &Instruction) {
        self.cpu.index_register +=
            self.cpu.data_registers[operation.get_register() as usize] as u16;
    }
    /// *LD F, Vx*:
    ///Stores the address of the sprite in Vx into I.
    ///Practically speaking, this just stores Vx * 5 into I
    ///0xFx29: I = Vx * 5.
    pub fn index_sprite(&mut self, operation: &Instruction) {
        self.cpu.index_register =
            self.cpu.data_registers[operation.get_register() as usize] as u16 * 5;
    }
    /// *LD B, Vx*:
    ///Stores the BCD version of Vx in memory at address I, I+1, & I+2.
    ///0xFx33
    pub fn store_bcd(&mut self, operation: &Instruction) {
        let value = self.cpu.data_registers[operation.get_register() as usize];
        let hundreds = value / 100;
        let tens = (value / 10) % 10;
        let ones = value % 10;

        let address = self.cpu.index_register as usize;
        self.memory.ram[address] = hundreds;
        self.memory.ram[address + 1] = tens;
        self.memory.ram[address + 2] = ones;
    }
    /// *LD [I], Vx*:
    ///Stores values in V0 -> Vx registers in consecutive memory locations starting at the address in I.
    ///0xFx55
    pub fn store_registers(&mut self, operation: &Instruction) {
        for i in 0..(operation.get_register() + 1) as usize {
            let value = self.cpu.data_registers[i];
            self.memory.ram[self.cpu.index_register as usize + i] = value;
        }
    }
    /// *LD Vx, [I]
    ///Loads values into V0 -> Vx registers from consecutive memory locations starting at the address in I.
    ///0xFx65
    pub fn load_registers(&mut self, operation: &Instruction) {
        for i in 0..(operation.get_register() + 1) as usize {
            let value = self.memory.ram[self.cpu.index_register as usize + i];
            self.cpu.data_registers[i] = value;
        }
    }

    pub fn load_rom(&mut self, rom_bytes: Vec<u8>) {
        self.memory.store_rom(rom_bytes);
    }
}

impl Display for Chip8Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = format!("{}", self.cpu);
        string.push_str(&format!("{}", self.memory));
        string.push_str(&format!(
            "Targeted Tick Frequency: {} Hz\n",
            self.clock_speed_hz
        ));
        string.push_str(&format!("{}", self.frame_buffer));

        write!(f, "{}", string)
    }
}

impl ThreadedEmulator for Chip8Computer {
    fn new(sender_from_computer: Sender<EmulatorResponse>) -> Self {
       Chip8Computer::new(sender_from_computer)
    }

    fn match_received_command(&self, command: EmulatorCommand) -> std::result::Result<(), String> {
        match command {
            EmulatorCommand::Go => todo!(),
            EmulatorCommand::Step(step_count) => todo!(),
            EmulatorCommand::GetMemory => todo!(),
            EmulatorCommand::GetRegisters => todo!(),
            EmulatorCommand::Pause => todo!(),
        }
    }
}

