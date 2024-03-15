use std::fmt::Display;

pub struct Memory {
    pub data: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: [0; 4096] }
    }

    pub fn _new_from_rom(rom_bytes: Vec<u8>) -> Self {
        let mut memory = Memory::new();
        memory.store_rom(rom_bytes);
        memory
    }

    pub fn store_rom(&mut self, rom_bytes: Vec<u8>) {
        for (i, byte) in rom_bytes.into_iter().enumerate() {
            self.data[i + 0x200] = byte;
        }
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
            .map(|offset| self.read_byte(starting_address.into() + offset))
            .collect()
    }
    pub fn read_instruction(&self, address: u16) -> u16 {
        let bytes = self.read_bytes(address, 2);
        let instruction_value = ((bytes[0] as u16) << 8) + bytes[1] as u16;
        instruction_value
    }

    pub fn print_rom(&self) {
        let rom_view = self.view_memory_section(0x200, self.data.len());
        println!("{rom_view}");
    }

    fn view_memory_section(&self, starting_index: usize, ending_index: usize) -> String {
        let mut return_string: String = "".into();

        let ending_index = std::cmp::min(ending_index, self.data.len()); 
        let bytes: Vec<u8> = self.data[starting_index..ending_index].into();

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
        self.view_memory_section(0x00, self.data.len())
    }

    pub fn get_only_interesting_memory(&self) -> String {
        let mut return_string = "".to_string();
        
        let permissable_empty_bytes = 10;
        let mut num_consecutive_zeros = 0;
        let mut start_of_interesting_data: Option<usize> = None;

        for (i, byte) in self.data.iter().enumerate() {
            if *byte == 0x00 {
                num_consecutive_zeros += 1;

                if start_of_interesting_data.is_some() && num_consecutive_zeros == permissable_empty_bytes {
                    let start_index = start_of_interesting_data.unwrap();
                    let aligned_start_index = start_index - (start_index % 16);

                    let last_interesting_index = i - num_consecutive_zeros;
                    let last_index = last_interesting_index + (last_interesting_index % permissable_empty_bytes); 

                    let memory_chunk_string = self.view_memory_section(aligned_start_index, last_index);
                    return_string.push_str(&memory_chunk_string);
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
}

impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.get_only_interesting_memory())
    }
}