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
use cgmath::*;
use math::*;
use num_traits::Float;
use geometry::*;
use std::borrow::*;

pub struct Renderer {
    ctx: Sdl,
    video_ctx: VideoSubsystem,
    canvas: Canvas<Window>,
    z_buffer: Frame<f32>,
    color_buffer: Frame<Color>,
    screen_width: u32,
    screen_height: u32,
}

impl Renderer {
    pub fn create(screen_width: u32, screen_height: u32) -> Self {
        let ctx = sdl2::init().unwrap();
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

        let z_buffer = Frame::new(
            screen_width as usize,
            screen_height as usize,
            Float::max_value()
        );

        let color_buffer = Frame::new(
            screen_width as usize,
            screen_height as usize,
            Color::RGB(0, 0, 0),
        );

        return Renderer {
            ctx,
            video_ctx,
            canvas,
            z_buffer,
            color_buffer,
            screen_width,
            screen_height,
        };
    }

    pub fn triangle(
        &mut self,
        v0: Vertex,
        v1: Vertex,
        v2: Vertex,
    ) {
        let projected_triangle = Triangle{
            p0: Point2{
                x: v0.position.x * self.color_buffer.width() as f32,
                y: (1.0 - v0.position.y) * self.color_buffer.height() as f32,
            },
            p1: Point2{
                x: v1.position.x * self.color_buffer.width() as f32,
                y: (1.0 - v1.position.y) * self.color_buffer.height() as f32,
            },
            p2: Point2{
                x: v2.position.x * self.color_buffer.width() as f32,
                y: (1.0 - v2.position.y) * self.color_buffer.height() as f32,
            },
        };
        let bounds =
            RectBounds::<u32>::from(RectBounds::bounds_of_triangle(projected_triangle));
        for x in bounds.left..bounds.right {
            for y in bounds.bottom..bounds.top {
                let point = Point2{x: x as f32, y: y as f32};
                if projected_triangle.point_is_inside(point) {
                    let bary = projected_triangle.barycentric_coordinates(point);
                    let inv_z = (1.0 / v0.position.z) * bary.0
                        + (1.0 / v1.position.z) * bary.1
                        + (1.0 / v2.position.z) * bary.2;
                    let z = 1.0 / inv_z;
                    if z < self.z_buffer.at(x as usize, y as usize).unwrap() {
                        let color = mix_colors(
                            &vec![v0.color, v1.color, v2.color],
                            &vec![bary.0, bary.1, bary.2]
                        );
                        self.color_buffer.set(x as usize, y as usize, color);
                        self.z_buffer.set(x as usize, y as usize, z);
                    }
                }
            }
        }
    }

    pub fn triangles(
        vertices: &Vec<Vector3<f32>>,
        triangles: &Vec<(usize, usize, usize)>,
    ) {
        /*
        for triangle in triangles {
            draw_triangle(
                [vertices[triangle.0], vertices[triangle.1], vertices[triangle.2]],

            )
        }
        */
    }

    pub fn present(&mut self) {
        let mut texture = self.canvas.create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::ARGB8888,
            self.color_buffer.width() as u32,
            self.color_buffer.height() as u32).unwrap();
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..self.color_buffer.height() {
                for x in 0..self.color_buffer.width() {
                    let pixel = self.color_buffer.at(x, y).unwrap();
                    let offset = y * pitch + x * 4;
                    buffer[offset] = pixel.b;
                    buffer[offset + 1] = pixel.g;
                    buffer[offset + 2] = pixel.r;
                    buffer[offset + 3] = pixel.a;
                }
            }
        }).unwrap();
        self.canvas.clear();
        let _ = self.canvas.copy(&texture, None, None);
        self.canvas.present();
    }

    pub fn clear(&mut self) {
        self.color_buffer = Frame::new(
            self.screen_width as usize,
            self.screen_height as usize,
            Color::RGB(0, 0, 0),
        );
        self.z_buffer = Frame::new(
            self.screen_width as usize,
            self.screen_height as usize,
            Float::max_value(),
        );
    }

    // I don't think this is good.
    pub fn get_events(&mut self) -> EventPump {
        return self.ctx.event_pump().unwrap();
    }
}




