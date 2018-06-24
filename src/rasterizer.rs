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
        world_vertices: (Vertex3, Vertex3, Vertex3),
        clip_vertices: (Vertex4, Vertex4, Vertex4),
        lights: &Vec<Light>,
        ambient: &FloatColor,
        texture: Option<&Texture>,
        material: &Material,
    ) {
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
        let bounds =
            RectBounds::<u32>::from(RectBounds::bounds_of_triangle(projected_triangle));
        for x in bounds.left..bounds.right {
            for y in bounds.bottom..bounds.top {
                let point = Point2{x: x as f32, y: y as f32};
                if projected_triangle.point_is_inside(point) {
                    let bary = projected_triangle.barycentric_coordinates(point);
                    let inv_z = (1.0 / clip_vertices.0.position.w) * bary.0
                        + (1.0 / clip_vertices.1.position.w) * bary.1
                        + (1.0 / clip_vertices.2.position.w) * bary.2;
                    let z = 1.0 / inv_z;
                    if z < self.z_buffer.at(x as usize, y as usize).unwrap() {
                        let weights = &vec![
                            z * bary.0 / clip_vertices.0.position.w,
                            z * bary.1 / clip_vertices.1.position.w,
                            z * bary.2 / clip_vertices.2.position.w,
                        ];
                        let normal = (world_vertices.0.normal * weights[0])
                            + (world_vertices.1.normal * weights[1])
                            + (world_vertices.2.normal * weights[2]);
                        let uvs = mix_uvs(
                            &vec![
                                clip_vertices.0.uv,
                                clip_vertices.1.uv,
                                clip_vertices.2.uv,
                            ],
                            weights,
                        );
                        let color = Self::process_fragment(
                            Vector3{x: 0.0, y: 0.0, z: 0.0},
                            normal,
                            uvs,
                            lights,
                            ambient,
                            texture,
                            material,
                        );
                        self.color_buffer.set(x as usize, y as usize, color.as_sdl_color());
                        self.z_buffer.set(x as usize, y as usize, z);
                    }
                }
            }
        }
    }

    fn process_fragment(
        world_coordinates: Vector3<f32>,
        world_normals: Vector3<f32>,
        uvs: Vector2<f32>,
        lights: &Vec<Light>,
        ambient: &FloatColor,
        texture: Option<&Texture>,
        material: &Material,
    ) -> FloatColor {
        let texture_color = match texture {
            Some(ref t) => {
                FloatColor::from_sdl_color(&t.sample(uvs.x, uvs.y))
            },
            None => FloatColor::from_rgb(1.0, 1.0, 1.0),
        };
        let color_from_lights = lights.iter()
            .map(|light| {
                Self::process_fragment_light(
                    world_coordinates,
                    world_normals,
                    light,
                    ambient,
                    material,
                )
            })
            .sum();
        let ambient_color = FloatColor::multiply_colors(ambient, &material.ambient);
        let total_light_color = ambient_color + color_from_lights;

        FloatColor::multiply_colors(&texture_color, &total_light_color)
    }

    fn process_fragment_light(
        world_coordinates: Vector3<f32>,
        world_normals: Vector3<f32>,
        light: &Light,
        ambient: &FloatColor,
        material: &Material,
    ) -> FloatColor {
        let intensity = match light.light_type {
            LightType::Directional(ref directional_light) => {
                let normalized_direction = directional_light.direction / directional_light.direction.magnitude();
                let intensity = normalized_direction.dot(world_normals);
                if intensity < 0.0 { 0.0 } else { intensity }
            }
            _ => 1.0,
        };
        FloatColor::multiply_colors(&material.diffuse, &light.color) * intensity
    }

    pub fn clear(&mut self) {
        self.color_buffer.set_all(Color::RGB(0, 0, 0));
        self.z_buffer.set_all(Float::max_value());
    }

    pub fn get_color_buffer(&self) -> &Frame<Color> {
        return &self.color_buffer;
    }
}




