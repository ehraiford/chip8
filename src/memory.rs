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
        let rom: Vec<u8> = self.data[0x200..].into();
        for (i, chunk) in rom.chunks(16).into_iter().enumerate() {
            print!("0x{:04x}:    ", (i * 16) + 0x200);
            for bytes in chunk.chunks(2) {
                print!("0x{:02x}{:02x}  ", bytes[0], bytes[1]);
            }
            println!();
        }
    }
}
