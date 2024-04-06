use std::fmt::Display;

pub struct Memory {
    pub ram: [u8; 4096],
    pub stack: [u16; 16],
}

impl Memory {
    pub fn new() -> Self {
        let mut ram = [0; 4096];
        let hex_sprites = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80,
            0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0,
            0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90,
            0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0,
            0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
            0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
        ];

        ram[0x00..0x50].copy_from_slice(&hex_sprites);
        Memory {
            ram,
            stack: [0; 16],
        }
    }

    pub fn _new_from_rom(rom_bytes: Vec<u8>) -> Self {
        let mut memory = Memory::new();
        memory.store_rom(rom_bytes);
        memory
    }

    pub fn store_rom(&mut self, rom_bytes: Vec<u8>) {
        for (i, byte) in rom_bytes.into_iter().enumerate() {
            self.ram[i + 0x200] = byte;
        }
    }

    pub fn read_byte<T>(&self, address: T) -> u8
    where
        T: Into<usize>,
    {
        self.ram[address.into()]
    }
    pub fn read_bytes<T>(&self, starting_address: T, num_bytes: T) -> Vec<u8>
    where
        T: Into<usize> + Copy,
    {
        (0..num_bytes.into())
            .map(|offset| self.read_byte(starting_address.into() + offset))
            .collect()
    }
    pub fn read_instruction(&self, address: u16) -> u16 {
        let bytes = self.read_bytes(address, 2);
        let instruction_value = ((bytes[0] as u16) << 8) + bytes[1] as u16;
        instruction_value
    }

    pub fn print_rom(&self) {
        let rom_view = self.view_memory_section(0x200, self.ram.len());
        println!("{rom_view}");
    }

    fn view_memory_section(&self, starting_index: usize, ending_index: usize) -> String {
        let mut return_string: String = "".into();

        let ending_index = std::cmp::min(ending_index, self.ram.len());
        let bytes: Vec<u8> = self.ram[starting_index..ending_index].into();

        for (i, chunk) in bytes.chunks(16).into_iter().enumerate() {
            return_string.push_str(&format!("0x{:04x}:    ", (i * 16) + starting_index));
            for bytes in chunk.chunks(2) {
                return_string.push_str(&format!("0x{:02x}{:02x}  ", bytes[0], bytes[1]));
            }
            return_string.push('\n');
        }

        return_string
    }

    pub fn get_memory_view_string(&self) -> String {
        self.view_memory_section(0x00, self.ram.len())
    }

    pub fn get_only_interesting_memory(&self) -> String {
        let mut return_string = "".to_string();

        let permissable_empty_bytes = 48;
        let mut num_consecutive_zeros = 0;
        let mut start_of_interesting_data: Option<usize> = None;

        for (i, byte) in self.ram.iter().enumerate() {
            if *byte == 0x00 {
                num_consecutive_zeros += 1;

                if start_of_interesting_data.is_some()
                    && num_consecutive_zeros == permissable_empty_bytes
                {
                    let start_index = start_of_interesting_data.unwrap();
                    let aligned_start_index = start_index - (start_index % 16);

                    let last_interesting_index = i - num_consecutive_zeros;
                    let last_index = last_interesting_index + (16 - last_interesting_index % 16);

                    let memory_chunk_string =
                        self.view_memory_section(aligned_start_index, last_index);
                    if !return_string.is_empty() {
                        return_string.push_str(".....MEMORY BREAK.....\n");
                    }
                    return_string.push_str(&memory_chunk_string);
                    num_consecutive_zeros = 0;
                    start_of_interesting_data = None;
                }
            } else {
                num_consecutive_zeros = 0;
                if start_of_interesting_data == None {
                    start_of_interesting_data = Some(i);
                }
            }
        }

        return_string
    }

    pub fn draw_memory_as_sprites(&self) {
        let clear_command = match cfg!(windows) {
            true => "cls".to_string(),
            false => "clear".to_string(),
        };
        let _ = std::process::Command::new(&clear_command).status();

        for (i, chunk) in self.ram.chunks_exact(5).enumerate() {
            if chunk.into_iter().all(|byte| *byte == 0) {
                continue;
            }

            let mut sprite = "".to_string();
            sprite.push_str(&format!("0x{:04x}\n", i * 5));
            for i in 0..5 {
                for j in (0..8).rev() {
                    match (chunk[i] >> j) & 0x01 == 1 {
                        true => sprite.push('\u{2588}'),
                        false => sprite.push(' '),
                    }
                }
                sprite.push('\n');
            }
            println!("{}\n", sprite);
        }
    }

    pub fn get_stack(&self) -> [u16; 16] {
        self.stack
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_only_interesting_memory())
        //write!(f,"{}", self.get_memory_view_string())
    }
}
