use frame::Frame;
use sdl2::{
    pixels::Color,
};
use cgmath::*;
use math::*;
use num_traits::Float;
use geometry::*;
use std;
use textures::*;
use colors::*;
use materials::*;
use light::*;

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

    // TODO: consolidate most of these fields...
    pub fn triangle(
        &mut self,
        world_vertices: (Vertex4, Vertex4, Vertex4),
        camera_vertices: (Vertex4, Vertex4, Vertex4),
        clip_vertices: (Vertex4, Vertex4, Vertex4),
        fragment_processor: &Fn(
            &Vector4<f32>,
            &Vector4<f32>,
            &Vector2<f32>,
        ) -> FloatColor,
    ) {
        // Return if triangle is facing away from camera.
        let edge1 = (clip_vertices.1.position - clip_vertices.0.position).truncate();
        let edge2 = (clip_vertices.2.position - clip_vertices.0.position).truncate();
        if edge1.cross(edge2).dot(Vector3{x: 0.0, y: 0.0, z: 1.0}) < 0.0 {
            return;
        }

        // Compute coordinates of triangle in screen space.
        let projected_triangle = Triangle{
            p0: Point2{
                x: ((clip_vertices.0.position.x + 1.0) / 2.0) * self.color_buffer.width() as f32,
                y: ((1.0 - clip_vertices.0.position.y) / 2.0) * self.color_buffer.height() as f32,
            },
            p1: Point2{
                x: ((clip_vertices.1.position.x + 1.0) / 2.0) * self.color_buffer.width() as f32,
                y: ((1.0 - clip_vertices.1.position.y) / 2.0) * self.color_buffer.height() as f32,
            },
            p2: Point2{
                x: ((clip_vertices.2.position.x + 1.0) / 2.0) * self.color_buffer.width() as f32,
                y: ((1.0 - clip_vertices.2.position.y) / 2.0) * self.color_buffer.height() as f32,
            },
        };
        let optional_bounds =
            RectBounds::<i32>::from(RectBounds::bounds_of_triangle(projected_triangle))
                .overlap(RectBounds{
                    left: 0,
                    bottom: 0,
                    right: (self.screen_width - 1) as i32,
                    top: (self.screen_height - 1) as i32,
                });
        // ugh because I don't want to match and indent
        let bounds = optional_bounds.unwrap_or(RectBounds{top: 0, bottom: 0, left: 0, right: 0});
        for y in bounds.bottom..bounds.top + 1 {
            let optional_bounds = projected_triangle.bounds_at_height(y as f32);
            match optional_bounds {
                Some(line_bounds) => {
                    let x_start = std::cmp::max(line_bounds.0 as i32, 0);
                    let x_end = std::cmp::min(line_bounds.1 as i32, (self.screen_width - 1) as i32);
                    for x in x_start..x_end + 1 {
                        let point = Point2{x: x as f32, y: y as f32};
                        let bary = projected_triangle.barycentric_coordinates(point);
                        // should maybe be -w?
                        let adjusted_bary = (
                            bary.0 / -camera_vertices.0.position.z,
                            bary.1 / -camera_vertices.1.position.z,
                            bary.2 / -camera_vertices.2.position.z,
                        );
                        let inv_z = adjusted_bary.0 + adjusted_bary.1 + adjusted_bary.2;
                        let z = 1.0 / inv_z;
                        if z <= 0.0 {
                            continue;
                        }
                        if z < self.z_buffer.at(x as usize, y as usize).unwrap() {
                            let w0 = z * adjusted_bary.0;
                            let w1 = z * adjusted_bary.1;
                            let w2 = z * adjusted_bary.2;
                            let normal = (world_vertices.0.normal * w0)
                                + (world_vertices.1.normal * w1)
                                + (world_vertices.2.normal * w2);
                            let uvs = clip_vertices.0.uv * w0
                                + clip_vertices.1.uv * w1
                                + clip_vertices.2.uv * w2;
                            let world = (world_vertices.0.position * w0)
                                + (world_vertices.1.position * w1)
                                + (world_vertices.2.position * w2);
                            let color = fragment_processor(
                                &world,
                                &normal,
                                &uvs,
                            );
                            self.color_buffer.set(x as usize, y as usize, color.as_sdl_color());
                            self.z_buffer.set(x as usize, y as usize, z);
                        }
                    }
                },
                None => {},
            };
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




