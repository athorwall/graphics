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
use textures::*;

pub struct Rasterizer {
    z_buffer: Frame<f32>,
    color_buffer: Frame<Color>,
    screen_width: u32,
    screen_height: u32,
}

impl Rasterizer {
    pub fn create(screen_width: u32, screen_height: u32) -> Self {
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

        return Rasterizer {
            z_buffer,
            color_buffer,
            screen_width,
            screen_height,
        };
    }

    pub fn triangle(
        &mut self,
        v0: Vertex4,
        v1: Vertex4,
        v2: Vertex4,
        texture: Option<&Texture>,
    ) {
        let projected_triangle = Triangle{
            p0: Point2{
                x: ((v0.position.x + 1.0) / 2.0) * self.color_buffer.width() as f32,
                y: ((1.0 - v0.position.y) / 2.0) * self.color_buffer.height() as f32,
            },
            p1: Point2{
                x: ((v1.position.x + 1.0) / 2.0) * self.color_buffer.width() as f32,
                y: ((1.0 - v1.position.y) / 2.0) * self.color_buffer.height() as f32,
            },
            p2: Point2{
                x: ((v2.position.x + 1.0) / 2.0) * self.color_buffer.width() as f32,
                y: ((1.0 - v2.position.y) / 2.0) * self.color_buffer.height() as f32,
            },
        };
        let bounds =
            RectBounds::<u32>::from(RectBounds::bounds_of_triangle(projected_triangle));
        for x in bounds.left..bounds.right {
            for y in bounds.bottom..bounds.top {
                let point = Point2{x: x as f32, y: y as f32};
                if projected_triangle.point_is_inside(point) {
                    let bary = projected_triangle.barycentric_coordinates(point);
                    let inv_z = (1.0 / v0.position.w) * bary.0
                        + (1.0 / v1.position.w) * bary.1
                        + (1.0 / v2.position.w) * bary.2;
                    let z = 1.0 / inv_z;
                    if z < self.z_buffer.at(x as usize, y as usize).unwrap() {
                        let weights = &vec![
                            z * bary.0 / v0.position.w,
                            z * bary.1 / v1.position.w,
                            z * bary.2 / v2.position.w,
                        ];
                        let mut color = mix_colors(
                            &vec![v0.color, v1.color, v2.color],
                            &weights,
                        );
                        match texture {
                            Some(ref t) => {
                                let uvs = mix_uvs(
                                    &vec![v0.uv, v1.uv, v2.uv],
                                    weights,
                                );
                                color = t.sample(uvs.x, uvs.y);
                            },
                            None => {},
                        };
                        self.color_buffer.set(x as usize, y as usize, color);
                        self.z_buffer.set(x as usize, y as usize, z);
                    }
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer.set_all(Color::RGB(0, 0, 0));
        self.z_buffer.set_all(Float::max_value());
    }

    pub fn get_color_buffer(&self) -> &Frame<Color> {
        return &self.color_buffer;
    }
}




