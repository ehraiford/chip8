pub struct Memory {
    pub data: [u8; 4096],
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