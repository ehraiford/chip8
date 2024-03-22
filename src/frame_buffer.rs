use std::{fmt::Display, process::Command, time::SystemTime};

pub struct FrameBuffer {
    pub buffer: [u64; 32],
    clear_command: String,
    last_update: SystemTime,
}

impl FrameBuffer {
    pub fn new() -> Self {
        let clear_command = match cfg!(windows) {
            true => "cls".to_string(),
            false => "clear".to_string(),
        };

        FrameBuffer {
            buffer: [0; 32],
            clear_command,
            last_update: SystemTime::now(),
        }
    }

    pub fn get_frame_buffer(&self) -> [u64; 32] {
        self.buffer
    }

    ///XORs the bytes onto screen starting at the the given coordinates.
    ///Returns whether or not any bits are erased because of this.
    pub fn draw_sprite(&mut self, start_x: u8, start_y: u8, bytes: Vec<u8>) -> bool {
        let mut result = false;

        for (i, byte) in bytes.into_iter().enumerate() {
            let mut byte_u64 = (byte as u64) << 56;
            byte_u64 = byte_u64.rotate_right(start_x as u32);

            let y_position = start_y as usize + i;
            if y_position >= 32 {
                break;
            }

            if byte_u64 & self.buffer[y_position] != 0 {
                result = true;
            }

            self.buffer[y_position] ^= byte_u64;
        }

        result
    }

    pub fn clear(&mut self) {
        for mut line in self.buffer {
            line = 0;
        }
    }

    pub fn get_buffer_as_string(&self) -> String {
        let mut string_buffer = String::new();
        for line in self.buffer {
            for i in (0..64).rev() {
                let bit = (line >> i) as u8 & 0x01;
                if bit == 1 {
                    string_buffer.push('\u{2588}');
                } else {
                    string_buffer.push(' ');
                }
            }
            string_buffer.push('\n');
        }
        string_buffer
    }

    pub fn print_buffer_to_terminal(&self) {
        let _ = Command::new(&self.clear_command).status();
        println!("{}", self.get_buffer_as_string());
    }

    pub fn print_loop(&self) {}
}

impl Display for FrameBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_buffer_as_string())
    }
}
