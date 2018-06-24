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
use graphics::colors::*;
use graphics::materials::*;

fn main() {
    let ctx = sdl2::init().unwrap();
    let mut events = ctx.event_pump().unwrap();
    let mut timers = Timers::new();
    let mut canvas = create_sdl_canvas(&ctx, 50, 50);

    let mut rasterizer = Rasterizer::create(50, 50);
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
    let material = Material{
        diffuse: FloatColor::from_rgb(1.0, 1.0, 1.0),
        specular: FloatColor::from_rgb(1.0, 1.0, 1.0),
        ambient: FloatColor::from_rgb(1.0, 1.0, 1.0),
    };
    let lights = vec![
        Light::directional_light(Vector3{x: -1.0, y: -1.0, z: 1.0})
    ];
    let ambient = FloatColor::from_rgb(0.1, 0.1, 0.1);

    'main: loop {
        rasterizer.clear();

        timers.start("render");
        render_mesh(
            &mesh,
            &mut rasterizer,
            &transformation,
            &lights,
            &ambient,
            &texture,
            &material,
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

    println!("{}", timers);
}

fn render_mesh<>(
    mesh: &Mesh,
    rasterizer: & mut Rasterizer,
    world_to_clip_space: &Matrix4<f32>,
    lights: &Vec<Light>,
    ambient: &FloatColor,
    texture: &Texture,
    material: &Material,
) {
    for (w0, w1, w2) in &mesh.vertices {
        let c0 = process_vertex(&w0, world_to_clip_space, lights);
        let c1 = process_vertex(&w1, world_to_clip_space, lights);
        let c2 = process_vertex(&w2, world_to_clip_space, lights);
        rasterizer.triangle(
            (*w0, *w1, *w2),
            (c0, c1, c2),
            lights,
            ambient,
            Some(texture),
            material,
        );
    }
}

fn process_vertex(vertex: &Vertex3, world_to_clip_space: &Matrix4<f32>, lights: &Vec<Light>) -> Vertex4 {
    let transformed_vertex =  Vertex4{
        position: world_to_clip_space * vertex.to_vertex4(1.0).position,
        uv: vertex.uv,
    };
    return transformed_vertex.perspective_adjusted();
}

