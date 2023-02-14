#![forbid(unsafe_code)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use rand::Rng;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod colors;
mod window;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const BOX_SIZE: i16 = 64;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,

    is_paused: bool,
}

pub fn main() -> Result<(), Error> {
    println!("Hello, world!");

    env_logger::init();

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello MicroPC")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut world = World::new();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame_mut());
            if let Err(err) = pixels.render() {
                error!("pixels.render() failed: {err}");
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Space) {
                world.pause();
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    error!("pixels.resize_surface() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });

    /*

    'running: loop {
        for event in event_pump.poll_iter() {

        // Create a red-green gradient
        init_context
            .screen
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                let mut rng = rand::thread_rng();
                let x = rng.gen_range(0..window::W);
                let y = rng.gen_range(0..window::H);
                let color = colors::VGA_4BIT[rng.gen_range(0..10)];

                let offset = y * pitch + x * 3;
                buffer[offset] = color.r;
                buffer[offset + 1] = color.g;
                buffer[offset + 2] = color.b;
            })?;

        init_context.canvas.copy(
            &init_context.screen,
            None,
            Some(Rect::new(0, 0, 2560, 1920)),
        )?;

        init_context.canvas.present();

        // canvas.clear();
        // canvas.present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }
    Ok(())
     */
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,

            is_paused: false,
        }
    }

    fn pause(&mut self) {
        self.is_paused = !self.is_paused;
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if self.is_paused {
            return;
        }

        if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        if self.is_paused {
            return;
        }
        let mut rng = rand::thread_rng();

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let inside_the_box = x >= self.box_x
                && x < self.box_x + BOX_SIZE
                && y >= self.box_y
                && y < self.box_y + BOX_SIZE;

            let rgba = if inside_the_box {
                let color = colors::VGA_4BIT[rng.gen_range(0..10)];

                [color[0], color[1], color[2], color[3]]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
