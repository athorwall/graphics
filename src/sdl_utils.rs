use frame::Frame;
use sdl2;
use sdl2::*;
use sdl2::{
    event::Event,
    pixels::Color,
    rect::Point,
    render::Canvas,
    video::Window,
};
use std::ptr;
use timing::*;

pub fn create_sdl_canvas(ctx: &Sdl, screen_width: u32, screen_height: u32) -> Canvas<Window> {
    let video_ctx = ctx.video().unwrap();

    let window = match video_ctx.window(
        "window",
        screen_width,
        screen_height)
        .position_centered()
        .opengl()
        .build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    let mut canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(err)   => panic!("failed to create renderer: {}", err)
    };

    return canvas;
}

pub fn render_to_canvas(
    canvas: &mut Canvas<Window>,
    color_buffer: &Frame<u32>,
) {
    let mut texture = canvas.create_texture_streaming(
        sdl2::pixels::PixelFormatEnum::ARGB8888,
        color_buffer.width() as u32,
        color_buffer.height() as u32).unwrap();
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        unsafe {
            ptr::copy_nonoverlapping(
                color_buffer.cells().as_ptr() as *const u8,
                buffer.as_mut_ptr(),
                color_buffer.cells().len() * 4);
        }
    }).unwrap();
    canvas.clear();
    let _ = canvas.copy(&texture, None, None);
    canvas.present();
}

pub fn RGB(r: u8, g: u8, b: u8) -> u32 {
    return (255 << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
}
