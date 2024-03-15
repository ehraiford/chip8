pub struct Instruction {
    pub value: u16,
}

impl Instruction {
    ///Returns bits 15-12 of an operation's value.
    ///This value normally is the opcode part of an instruction.
    ///
    ///Value is in the 4 right bits of the return value.
    pub fn get_opcode(&self) -> u8 {
        (self.value >> 12) as u8
    }

    ///Returns bits 7-0 of an operation's value.
    ///This value is where instructions using an immediate value encode the immedate.
    pub fn get_immediate(&self) -> u8 {
        (self.value & 0x00FF) as u8
    }

    ///Returns bits 11-0 of an operation's value.
    ///This value is where instructions using a large immediate like for a memory address encode it.
    pub fn get_address_immediate(&self) -> u16 {
        self.value & 0x0FFF
    }

    ///Returns bits 11-8 of an operation's value.
    ///This value is where instructions operating on a register encode the register number.
    ///
    ///Value is in the 4 right bits of the return value.
    pub fn get_register(&self) -> u8 {
        ((self.value & 0x0F00) >> 8) as u8
    }

    ///Returns bits 7-4 of an operation's value.
    ///This value is where instructions using two registers encodes the second register.
    ///
    ///Value is in the 4 right bits of the return value.
    pub fn get_second_register(&self) -> u8 {
        ((self.value & 0x00F0) >> 4) as u8
    }

    ///Returns bits 3-0 of an operation's value.
    ///This value is where an 4-bit immediate is stored if the instruction is also storing 2 registers
    ///
    ///Value is in the 4 right bits of the return value.
    pub fn get_small_immediate(&self) -> u8 {
        (self.value & 0x000F) as u8
    }

    pub fn new(value: u16) -> Self {
        Instruction { value }
    }
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        Instruction { value }
    }
}
