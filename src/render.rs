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
    z_buffer: Frame<f32>,
    color_buffer: Frame<Color>,
    screen_width: u32,
    screen_height: u32,
}

impl Renderer {
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

        return Renderer {
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

    pub fn mesh(
        &mut self,
        mesh: &Mesh,
    ) {
        for (v0, v1, v2) in &mesh.triangles {
            self.triangle(
                mesh.vertices[*v0],
                mesh.vertices[*v1],
                mesh.vertices[*v2],
            );
        }
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

    pub fn get_color_buffer(&self) -> &Frame<Color> {
        return &self.color_buffer;
    }
}




