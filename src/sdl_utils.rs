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

pub fn render_to_canvas(canvas: &mut Canvas<Window>, color_buffer: &Frame<Color>) {
    let mut texture = canvas.create_texture_streaming(
        sdl2::pixels::PixelFormatEnum::ARGB8888,
        color_buffer.width() as u32,
        color_buffer.height() as u32).unwrap();
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in 0..color_buffer.height() {
            for x in 0..color_buffer.width() {
                let pixel = color_buffer.at(x, y).unwrap();
                let offset = y * pitch + x * 4;
                buffer[offset] = pixel.b;
                buffer[offset + 1] = pixel.g;
                buffer[offset + 2] = pixel.r;
                buffer[offset + 3] = pixel.a;
            }
        }
    }).unwrap();
    canvas.clear();
    let _ = canvas.copy(&texture, None, None);
    canvas.present();
}
