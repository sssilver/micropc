/*
use sdl2::{video::Window, Sdl, render::{Texture, Canvas}, pixels::PixelFormatEnum};

pub const W: usize = 320;
pub const H: usize = 200;

pub struct Context<'a> {
    pub canvas: Canvas<Window>,
    pub sdl_context: Sdl,
    pub window: Window,
    pub screen: Texture<'a>,
}


pub fn create<'a>() -> Result<Context<'a>, String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("EC1841", 1280, 960)
        .resizable()
        .fullscreen_desktop()
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, W as u32, H as u32)
        .map_err(|e| e.to_string())?;

    Ok(Context {
        canvas,
        sdl_context,
        window,
        screen: texture,
    })
}
 */
