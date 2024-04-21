use pixels::{Error, Pixels, SurfaceTexture};
use std::{sync::mpsc::{self, Receiver}, thread};
use computer::EmulatorResponse;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct ProgramDisplay {
    event_loop: EventLoop<()>,
    pixels: Pixels,
    width: usize,
    height: usize,
    receiver_from_emulator: Receiver<EmulatorResponse>
}

 impl ProgramDisplay {

    fn new(width: usize, height: usize, window_name: String, receiver_from_emulator: Receiver<EmulatorResponse>) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(window_name)
            .with_inner_size(winit::dpi::LogicalSize::new(640, 320))
            .build(&event_loop)
            .unwrap();

        let surface_texture = {
            let window_size = window.inner_size();
            SurfaceTexture::new(window_size.width, window_size.height, &window)
        };
        let mut pixels = Pixels::new(width as u32, height as u32, surface_texture).unwrap();

        ProgramDisplay {
            event_loop: EventLoop::new(),
            pixels,
            width,
            height,
            receiver_from_emulator
        }

    }
    
    fn initialize(&self) {
        let mut current_image = Self::start_image(self.width, self.height).to_rgb_vec();

        // Run the event loop
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                Event::RedrawRequested(_) => {
                    if let Ok(new_image) = self.receiver_from_emulator.try_recv() {
                        current_image = new_image
                    }

                    for (i, pixel) in pixels.get_frame().chunks_exact_mut(4).enumerate() {
                        pixel.copy_from_slice(&current_image[i]);
                    }
                    if pixels
                        .render()
                        .map_err(|e| eprintln!("pixels.render() failed: {}", e))
                        .is_err()
                    {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                _ => {}
            }
        });
    }

    /// Takes in a width and height for a frame_buffer and generates a Vec<impl ToRGB> for a sample image.
    ///
    /// The default implementation just creates a checkered pattern.
    fn start_image(width: usize, height: usize) -> Vec<impl ToRGB> {
        let mut buffer = Vec::<u8>::new();
        for i in 0..(width * height) {
            if ((i % width) % 8 < 4 && (i / width) % 8 < 4)
                || ((i % width) % 8 > 3 && (i / width) % 8 > 3)
            {
                buffer.push(255);
            } else {
                buffer.push(0);
            }
        }
        buffer
    }
}


impl ToRGB for u8 {
    fn to_rgb(&self) -> [u8; 4] {
        match self {
            0 => [16, 21, 158, 255],
            _ => [0, 0, 0, 255],
        }
    }
}

pub trait ToRGB {
    fn to_rgb(&self) -> [u8; 4];
}
pub trait ToRGBVec {
    fn to_rgb_vec(&self) -> Vec<[u8; 4]>;
}
impl<T: ToRGB> ToRGBVec for Vec<T> {
    fn to_rgb_vec(&self) -> Vec<[u8; 4]> {
        self.into_iter().map(|item| item.to_rgb()).collect()
    }
}
