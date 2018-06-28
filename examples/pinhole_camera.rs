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
    let mut canvas = create_sdl_canvas(&ctx, 1000, 800);

    let mut rasterizer = Rasterizer::create(1000, 800);
    let mut mesh = Mesh::xy_face(2.5).transformed(Matrix4::from_angle_x(Deg(-90.0)));
    let mut mesh2 = Mesh::cube(0.5).transformed(Matrix4::from_translation(Vector3{x: 0.0, y: 0.25, z: 0.0}));
    //let camera = Matrix4::from_translation(Vector3{x: 0.0, y: 1.0, z: 2.0});

    let mut texture_frame = Frame::new(128, 128, Color::RGB(255, 255, 255));
    for x in 0..128 {
        for y in 0..128 {
            if ((x / 8) + (y / 8)) % 2 == 0 {
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
        Light::point_light(Vector3{x: 1.0, y: 1.0, z: 1.0}),
        Light::point_light(Vector3{x: -1.0, y: 1.0, z: 1.0}),
    ];
    let ambient = FloatColor::from_rgb(0.1, 0.1, 0.1);

    let mut camera_pos = Vector3{x: 0.0, y: 1.0, z: 0.0};

    'main: loop {
        rasterizer.clear();

        timers.start("render");
        let camera = Matrix4::from_translation(camera_pos);
        println!("Camera matrix: {:?}", camera);
        let perspective = Matrix4::from(perspective(Deg(90.0), 1000.0 / 800.0, 0.1, 100.0));
        let transformation = perspective * camera.invert().unwrap();
        render_mesh(
            &mesh,
            &mut rasterizer,
            &transformation,
            &lights,
            &ambient,
            Some(&texture),
            &material,
        );
        render_mesh(
            &mesh2,
            &mut rasterizer,
            &transformation,
            &lights,
            &ambient,
            None,
            &material,
        );
        timers.stop("render");

        mesh.transform(Matrix4::from_angle_y(Deg(0.3)));
        mesh2.transform(Matrix4::from_angle_y(Deg(0.3)));

        timers.start("canvas");
        render_to_canvas(&mut canvas, rasterizer.get_color_buffer());
        timers.stop("canvas");

        {
            events.pump_events();
            let keyboard_state = events.keyboard_state();
            if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Up) {
                camera_pos += Vector3{x: 0.0, y: 0.0, z: -0.02};
            }
            if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Down) {
                camera_pos += Vector3{x: 0.0, y: 0.0, z: 0.02};
            }
        }

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
    texture: Option<&Texture>,
    material: &Material,
) {
    for (w0, w1, w2) in &mesh.vertices {
        let c0 = process_vertex(&w0, world_to_clip_space);
        let c1 = process_vertex(&w1, world_to_clip_space);
        let c2 = process_vertex(&w2, world_to_clip_space);
        match (c0, c1, c2) {
            (Some(c0), Some(c1), Some(c2)) => {
                rasterizer.triangle(
                    (*w0, *w1, *w2),
                    (c0, c1, c2),
                    lights,
                    ambient,
                    texture,
                    material,
                );
            }
            _ => {}
        }
    }
}

// HACK we don't have proper clipping so ignore any vertex that's not visible
fn process_vertex(vertex: &Vertex3, world_to_clip_space: &Matrix4<f32>) -> Option<Vertex4> {
    let transformed_vertex =  Vertex4{
        position: world_to_clip_space * vertex.to_vertex4(1.0).position,
        uv: vertex.uv,
    };
    if transformed_vertex.position.x <= -transformed_vertex.position.w
        || transformed_vertex.position.x >= transformed_vertex.position.w
        || transformed_vertex.position.y <= -transformed_vertex.position.w
        || transformed_vertex.position.y >= transformed_vertex.position.w
        || transformed_vertex.position.z <= -transformed_vertex.position.w
        || transformed_vertex.position.z >= transformed_vertex.position.w {
        return None;
    } else {
        return Some(transformed_vertex.perspective_adjusted());
    }
}

