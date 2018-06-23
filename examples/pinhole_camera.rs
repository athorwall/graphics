extern crate cgmath;
extern crate graphics;
extern crate sdl2;
extern crate num_traits;
extern crate timing;

use cgmath::*;
use graphics::rasterizer::*;
use graphics::geometry::*;
use graphics::sdl_utils::*;
use sdl2::{
    event::Event,
    pixels::Color,
};
use timing::Timers;
use graphics::frame::*;
use graphics::textures::*;
use graphics::light::*;

fn main() {
    let ctx = sdl2::init().unwrap();
    let mut events = ctx.event_pump().unwrap();
    let mut timers = Timers::new();
    let mut canvas = create_sdl_canvas(&ctx, 640, 480);

    let mut rasterizer = Rasterizer::create(640, 480);
    let mut mesh = Mesh::cube(1.5);

    let camera = Matrix4::look_at(
        Point3 { x: 0.0, y: 0.0, z: -2.5 },
        Point3 { x: 0.0, y: 0.0, z: 1.0 },
        Vector3{ x: 0.0, y: 1.0, z: 0.0 },
    );
    let perspective = Matrix4::from(perspective(Deg(90.0), 640.0 / 480.0, 0.1, 100.0));
    let transformation = perspective * camera.invert().unwrap();

    let mut texture_frame = Frame::new(8, 8, Color::RGB(255, 255, 255));
    for x in 0..8 {
        for y in 0..8 {
            if (x + y) % 2 == 0 {
                texture_frame.set(x, y, Color::RGB(0, 0, 255));
            }
        }
    }
    let texture = Texture::create(texture_frame);
    let lights = vec![
        Light::directional_light(Vector3{x: -1.0, y: -1.0, z: 1.0})
    ];

    'main: loop {
        rasterizer.clear();

        timers.start("render");
        render_mesh(
            &mesh,
            &mut rasterizer,
            &transformation,
            &lights,
            &texture,
        );
        timers.stop("render");

        mesh.transform(Matrix4::from_angle_y(Deg(0.3)));

        timers.start("canvas");
        render_to_canvas(&mut canvas, rasterizer.get_color_buffer());
        timers.stop("canvas");

        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'main,
                _               => continue
            }
        }
    }
}

fn render_mesh<>(
    mesh: &Mesh,
    rasterizer: & mut Rasterizer,
    world_to_clip_space: &Matrix4<f32>,
    lights: &Vec<Light>,
    texture: &Texture,
) {
    let processed_mesh = mesh.clone();
    let vertices: Vec<(Vertex4, Vertex4, Vertex4)> = processed_mesh.vertices.iter()
        .map(|(v0, v1, v2)| {
            return (
                process_vertex(&v0, world_to_clip_space, lights),
                process_vertex(&v1, world_to_clip_space, lights),
                process_vertex(&v2, world_to_clip_space, lights),
            );
        })
        .collect();
    for (v0, v1, v2) in &vertices {
        rasterizer.triangle(
            *v0,
            *v1,
            *v2,
            Some(texture),
        );
    }
}

fn process_vertex(vertex: &Vertex3, world_to_clip_space: &Matrix4<f32>, lights: &Vec<Light>) -> Vertex4 {
    let transformed_vertex =  Vertex4{
        position: world_to_clip_space * vertex.to_vertex4(1.0).position,
        color: vertex.color,
        uv: vertex.uv,
    };
    return transformed_vertex.perspective_adjusted();
}
