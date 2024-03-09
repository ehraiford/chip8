pub struct FrameBuffer {
    pub buffer: [u32; 64],
}

impl FrameBuffer {
    pub fn new() -> Self {
        FrameBuffer { buffer: [0; 64] }
    }

    pub fn get_frame_buffer(&self) -> [u32; 64] {
        self.buffer
    }

    ///XORs the bytes onto screen starting at the the given coordinates.
    ///Returns whether or not any bits are erased because of this.
    pub fn draw_sprite(&mut self, start_x: u8, start_y: u8, bytes: Vec<u8>) -> bool {
        let mut xor_line = 0;
        for (i, byte) in bytes.into_iter().enumerate() {
            let byte_as_32 = (byte as u32) << 24;
            let right_shift = start_x + (i as u8 * 8);
            xor_line |= (byte_as_32 >> (right_shift % 32)) % 32;
        }

        let result = (xor_line & self.buffer[start_y as usize]) == 0;
        self.buffer[start_y as usize] ^= xor_line;

        result
    }

    pub fn clear(&mut self) {
        for mut line in self.buffer {
            line = 0;
        }
    }
}




pub struct GUI {
   // canvas: Canvas<Window>
}
impl GUI {
    // pub fn new() -> Self {
    //     let sdl_context = sdl2::init().unwrap();
    //     let video_subsystem = sdl_context.video().unwrap();
    //     let window = video_subsystem
    //         .window("Chip-8 Emulator", 640, 320)
    //         .position_centered()
    //         .build()
    //         .unwrap();
        
    //     Self {
    //         canvas: window.into_canvas().build().unwrap()
    //     }
    // }

    // pub fn present(&mut self) {
    //     // Clear the screen with a black color
    //     self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    //     self.canvas.clear();
    //     self.canvas.present();
    // }
}