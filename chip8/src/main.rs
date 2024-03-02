use rand::Rng;

fn main() {
    let mut emulator_instance = Chip8Computer::new();
    emulator_instance.map_operation_to_function(&Operation::new(0x0000))
}

struct Chip8Computer {
    cpu: Cpu,
    memory: Memory,
    frame_buffer: FrameBuffer,
    input: Input,
}

impl Chip8Computer {
    pub fn new() -> Self {
        Chip8Computer {
            cpu: Cpu::new(),
            memory: Memory::new(),
            frame_buffer: FrameBuffer::new(),
            input: Input::new(),
        }
    }

    fn map_operation_to_function(&mut self, operation: &Operation) {
        match operation.get_opcode() {
            0x0 => match operation.value {
                0x00E0 => {
                    self.clear_frame_buffer();
                }
                0x00EE => {
                    self.return_subroutine();
                }
                _ => {
                    panic!("sys addr is not implemented in this emulator.");
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
                0x0 => {}
                0x1 => {}

                _ => {
                    panic!("This isn't implemented.")
                }
            },

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
        self.cpu.program_counter = self.cpu.get_top_of_stack(&self.memory);
    }
    ///*JP*:
    ///Jumps to the specified address.
    fn jump(&mut self, operation: &Operation) {
        self.cpu.program_counter = operation.get_address_immediate();
    }
    ///*CALL*:
    ///Calls a subroutine at the specified address
    ///0x2nnn: Puts current PC on the stack and sets PC to nnn.
    fn call(&mut self, operation: &Operation) {
        self.cpu
            .push_to_stack(self.cpu.program_counter, &mut self.memory);
        self.cpu.program_counter = operation.get_address_immediate();
    }
    ///*SE*:
    ///Skips the next instruction if the value in the specified register equals the specified value
    ///0x3xkk: Skips next instruction if Vx == kk.
    fn skip_equal_immediate(&mut self, operation: &Operation) {
        let immediate_value = operation.get_immediate();
        let register_value = self.cpu.data_registers[operation.get_register() as usize];
        if immediate_value == register_value {
            self.cpu.program_counter += 2;
        }
    }
    ///*SNE*:
    ///Skips the next instruction if the value in the specified register does not equal the specified value
    ///0x4xkk: Skips next instruction if Vx != kk.
    fn skip_not_equal_immediate(&mut self, operation: &Operation) {
        let immediate_value = operation.get_immediate();
        let register_value = self.cpu.data_registers[operation.get_register() as usize];

        if immediate_value != register_value {
            self.cpu.program_counter += 2;
        }
    }
    ///*SE*:
    ///Skips the next instruction if the values in the two specified registers are not equal
    ///0x5xy0: Skips next instruction if Vx == Vy.
    fn skip_equal_register(&mut self, operation: &Operation) {
        let register_one_value = self.cpu.data_registers[operation.get_register() as usize];
        let register_two_value = self.cpu.data_registers[operation.get_second_register() as usize];

        if register_one_value == register_two_value {
            self.cpu.program_counter += 2;
        }
    }
    ///*LD&:
    ///Loads the given value into the specified register.
    ///0x6xkk: Vx = kk.
    fn load_immediate(&mut self, operation: &Operation) {
        self.cpu.data_registers[operation.get_register() as usize] = operation.get_immediate();
    }
    ///*ADD*:
    ///Adds the given value to the specified register
    ///0x7xkk Vx = Vx + kk.
    fn add_immediate(&mut self, operation: &Operation) {
        self.cpu.data_registers[operation.get_register() as usize] += operation.get_immediate();
    }
    /// *LD*:
    ///Moves the value within the second specified register into the first.
    ///0x8xy0: Vx = Vy.
    fn move_register(&mut self, operation: &Operation) {
        let moved_value = self.cpu.data_registers[operation.get_second_register() as usize];
        self.cpu.data_registers[operation.get_register() as usize] = moved_value;
    }
    /// *OR*:
    ///Bitwise ors Vx and Vy and stores the result in Vx
    ///0x8xy1: Vx |= Vy.
    fn or_register(&mut self, operation: &Operation) {
        let or_value = self.cpu.data_registers[operation.get_second_register() as usize];
        self.cpu.data_registers[operation.get_register() as usize] |= or_value;
    }
    /// *AND*:
    ///Bitwise ands Vx and Vy and stores the result in Vx
    ///0x8xy2: Vx &= Vy.
    fn and_register(&mut self, operation: &Operation) {
        let and_value = self.cpu.data_registers[operation.get_second_register() as usize];
        self.cpu.data_registers[operation.get_register() as usize] &= and_value;
    }
    /// *XOR*:
    ///Bitwise xors Vx and Vy and stores the result in Vx
    ///0x8xy3: Vx ^= Vy.
    fn xor_register(&mut self, operation: &Operation) {
        let xor_value = self.cpu.data_registers[operation.get_second_register() as usize];
        self.cpu.data_registers[operation.get_register() as usize] ^= xor_value;
    }
    /// *ADD*:
    ///Adds the contents of two registers together and stores the result in the first
    ///Sets VF to whether or not there is a carry.
    ///0x8xy4: Vx += Vy.
    fn add_register(&mut self, operation: &Operation) {
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
    fn subtract_register(&mut self, operation: &Operation) {
        let first_value = self.cpu.data_registers[operation.get_register() as usize];
        let second_value = self.cpu.data_registers[operation.get_second_register() as usize];
        let result = first_value.overflowing_sub(second_value);
        self.cpu.data_registers[operation.get_register() as usize] = result.0;
        self.cpu.data_registers[0xF] = !result.1 as u8;
    }
    /// *SHR*:
    ///Shifts the value in a register right by one and stores the result in another. VF is set to the bit that was consumed.
    ///0x8xy6: Vx = Vy >> 1.
    fn shift_right(&mut self, operation: &Operation) {
        let second_register_value = operation.get_second_register() as u8;
        let last_bit = second_register_value & 0x01;
        self.cpu.data_registers[0xF] = last_bit as u8;
        self.cpu.data_registers[operation.get_register() as usize] = second_register_value >> 1;
    }
    /// *SUBN*:
    ///Subtracts the content of register 1 from register 2 and stores the result in the first.
    ///0x8xy7 Vx = Vy - Vx.
    fn subtract_register_not(&mut self, operation: &Operation) {
        let first_value = self.cpu.data_registers[operation.get_register() as usize];
        let second_value = self.cpu.data_registers[operation.get_second_register() as usize];
        let result = second_value.overflowing_sub(first_value);
        self.cpu.data_registers[operation.get_register() as usize] = result.0;
        self.cpu.data_registers[0xF] = !result.1 as u8;
    }
    /// *SHL*:
    ///Shifts the value in a register left by one and stores the result in another. VF is set to the bit that was consumed.
    ///0x8xy8: Vx = Vy << 1.
    fn shift_left(&mut self, operation: &Operation) {
        let second_register_value = operation.get_second_register() as u8;
        let first_bit = second_register_value & 0x80;
        self.cpu.data_registers[0xF] = first_bit as u8;
        self.cpu.data_registers[operation.get_register() as usize] = second_register_value << 1;
    }
    /// *SNE*:
    ///Skips the next instruction if the values in the two given registers are not equal
    ///0x9xy0: Skips next instruction if Vx != Vy.
    fn skip_not_equal_register(&mut self, operation: &Operation) {
        let first_value = self.cpu.data_registers[operation.get_register() as usize];
        let second_value = self.cpu.data_registers[operation.get_second_register() as usize];

        if second_value != first_value {
            self.cpu.program_counter += 2;
        }
    }
    ///*LD I*:
    ///Loads given large immediate value into I Register
    ///0xAnnn: I = nnn.
    fn load_address(&mut self, operation: &Operation) {
        self.cpu.index_register = operation.get_address_immediate()
    }
    ///*JP I*:
    ///Sets PC to nnn + V0
    ///0xBnnn: PC = nnn + V0.
    fn jump_register(&mut self, operation: &Operation) {
        let jump_address = self.cpu.data_registers[0] as u16 + operation.get_address_immediate();
        self.cpu.program_counter = jump_address;
    }
    /// *RND*:
    ///Generates a random 8-bit value, ANDs it with an immediate, and stores the result in Vx.
    ///0xCxkk: Vx = 0x?? + kk.
    fn random(&mut self, operation: &Operation) {
        let mut rng = rand::thread_rng();
        let random_number: u8 = rng.gen_range(0..255);
        let result = random_number & operation.get_immediate();
        self.cpu.data_registers[operation.get_register() as usize] = result;
    }
    /// *DRW*:
    ///Reads n bytes from memory starting at the address in Register I and displays them starting at (Vx, Vy).
    ///Sprites are XORed onto the screen with existing pixels. VF is set to whether any pixels are erased because of this.
    ///0xyn
    fn draw(&mut self, operation: &Operation) {
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
    fn skip_pressed(&mut self, operation: &Operation) {
        let key_value = self.cpu.data_registers[operation.get_register() as usize];
        if self.input.check_pressed(key_value) { 
            self.cpu.program_counter += 2;
        }
    }
    /// *SKNP*:
    ///Skips the next instruction if the key corresponding to the value in Vx is not pressed.
    ///ExA1
    fn skip_not_pressed(&mut self, operation: &Operation) {
        let key_value = self.cpu.data_registers[operation.get_register() as usize];
        if !self.input.check_pressed(key_value) { 
            self.cpu.program_counter += 2;
        }
    }
    /// *LD*:
    ///Loads the value from the delay timer and stores it in Vx
    ///0xFx07 Vx = Delay Timer.
    fn load_delay(&mut self, operation: &Operation) {
        self.cpu.data_registers[operation.get_register() as usize] = self.cpu.delay_timer;
    }
    /// *LD*:
    ///Stores the value of the next keypress in Vx. Execution stops until then.
    ///0xFx0A: Vx = Keypress.
    fn load_keypress(&mut self, operation: &Operation) {
        self.cpu.data_registers[operation.get_register() as usize] = self.input.receive_input();
    }
    /// *LD*:
    ///Sets delay timer to value within specified register.
    ///0xFx15: Delay Timer = Vx.
    fn store_delay(&mut self, operation: &Operation) {
        self.cpu.delay_timer = self.cpu.data_registers[operation.get_register() as usize];
    }
    /// *LD ST*:
    ///Sets sound timer to value within specified register.
    ///0xFx18: Sound Timer = Vx.
    fn store_sound(&mut self, operation: &Operation) {
        self.cpu.sound_timer = self.cpu.data_registers[operation.get_register() as usize];
    }
    /// *ADD I, Vx*:
    ///Adds I with the value in Vx and stores it in I.
    ///0xFx1E: I += Vx.
    fn add_index(&mut self, operation: &Operation) {
        self.cpu.index_register += self.cpu.data_registers[operation.get_register() as usize] as u16;
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

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

struct FrameBuffer {
    buffer: [u32; 64],
}

impl FrameBuffer {
    fn new() -> Self {
        FrameBuffer {
            buffer: [0; 64],
        }
    }

    fn get_frame_buffer(&self) -> [u32; 64] {
        self.buffer
    }

    ///XORs the bytes onto screen starting at the the given coordinates.
    ///Returns whether or not any bits are erased because of this.
    fn draw_sprite(&mut self, start_x: u8, start_y: u8, bytes: Vec<u8>) -> bool {

        let mut xor_line = 0;
        for (i, byte) in bytes.into_iter().enumerate() {
            let byte_as_32 = (byte as u32) << 24;
            let right_shift = start_x + (i as u8*8);
            xor_line |= (byte_as_32 >> right_shift) % 32;
        }

        let result = (xor_line & self.buffer[start_y as usize]) == 0;
        self.buffer[start_y as usize] ^= xor_line;
        
        result
    }

    fn clear(&mut self) {
        for mut line in self.buffer {
            line = 0;
        }
    }
}

struct Memory {
    data: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: [0; 4096] }
    }
    pub fn read_byte<T>(&self, address: T) -> u8
    where
        T: Into<usize>,
    {
        self.data[address.into()]
    }
    pub fn read_bytes<T>(&self, starting_address: T, num_bytes: T) -> Vec<u8>
    where
        T: Into<usize> + Copy,
    {
        (0..num_bytes.into())
            .map(|offset| self.data[starting_address.into() + offset])
            .collect()
    }
}

struct Operation {
    value: u16,
}

impl Operation {
    ///Returns bits 15-12 of an operation's value.
    ///This value normally is the opcode part of an instruction.
    ///
    ///Value is in the 4 right bits of the return value.
    fn get_opcode(&self) -> u8 {
        (self.value >> 12) as u8
    }

    ///Returns bits 7-0 of an operation's value.
    ///This value is where instructions using an immediate value encode the immedate.
    fn get_immediate(&self) -> u8 {
        (self.value & 0x00FF) as u8
    }

    ///Returns bits 11-0 of an operation's value.
    ///This value is where instructions using a large immediate like for a memory address encode it.
    fn get_address_immediate(&self) -> u16 {
        self.value & 0x0FFF
    }

    ///Returns bits 11-8 of an operation's value.
    ///This value is where instructions operating on a register encode the register number.
    ///
    ///Value is in the 4 right bits of the return value.
    fn get_register(&self) -> u8 {
        ((self.value & 0x0F00) >> 8) as u8
    }

    ///Returns bits 7-4 of an operation's value.
    ///This value is where instructions using two registers encodes the second register.
    ///
    ///Value is in the 4 right bits of the return value.
    fn get_second_register(&self) -> u8 {
        ((self.value & 0x00F0) >> 4) as u8
    }

    ///Returns bits 3-0 of an operation's value.
    ///This value is where an 4-bit immediate is stored if the instruction is also storing 2 registers
    ///
    ///Value is in the 4 right bits of the return value.
    fn get_small_immediate(&self) -> u8 {
        (self.value & 0x000F) as u8
    }

    fn new(value: u16) -> Self {
        Operation { value }
    }
}

struct Input {

}

impl Input {
    pub fn new() -> Self {
        return Input {
            
        }
    } 
    pub fn check_pressed(&self, key: u8) -> bool {
        todo!()
    }
    pub fn receive_input(&self) -> u8 {
        todo!()
    }
}