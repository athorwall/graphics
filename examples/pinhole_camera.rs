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
use graphics::math::*;

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
    ];
    let ambient = FloatColor::from_rgb(0.3, 0.3, 0.3);

    let mut camera = Matrix4::from_translation(Vector3{x: 0.0, y: 1.0, z: 2.0});

    'main: loop {
        rasterizer.clear();

        timers.start("render");
        let perspective = Matrix4::from(perspective(Deg(70.0), 1000.0 / 800.0, 0.1, 100.0));
        render_mesh(
            &mesh,
            &mut rasterizer,
            &camera.invert().unwrap(),
            &perspective,
            &lights,
            &ambient,
            Some(&texture),
            &material,
        );
        render_mesh(
            &mesh2,
            &mut rasterizer,
            &camera.invert().unwrap(),
            &perspective,
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
                camera = camera * Matrix4::from_translation(Vector3{x: 0.0, y: 0.0, z: -0.02});
            }
            if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Down) {
                camera = camera * Matrix4::from_translation(Vector3{x: 0.0, y: 0.0, z: 0.02});
            }
            if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Left) {
                camera = camera * Matrix4::from_angle_y(cgmath::Rad(0.01));
            }
            if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Right) {
                camera = camera * Matrix4::from_angle_y(cgmath::Rad(-0.01));
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

// todo: perform lighting calculations in camera space
fn render_mesh(
    mesh: &Mesh,
    rasterizer: & mut Rasterizer,
    world_to_camera_space: &Matrix4<f32>,
    camera_to_clip_space: &Matrix4<f32>,
    lights: &Vec<Light>,
    ambient: &FloatColor,
    texture: Option<&Texture>,
    material: &Material,
) {
    let world_to_clip_space = *camera_to_clip_space * *world_to_camera_space;
    for (w0, w1, w2) in &mesh.vertices {
        let clip0 = w0.to_vertex4(1.0).transformed(world_to_clip_space);
        let clip1 = w1.to_vertex4(1.0).transformed(world_to_clip_space);
        let clip2 = w2.to_vertex4(1.0).transformed(world_to_clip_space);
        let tris = clip_triangle(
            clip0,
            clip1,
            clip2,
            camera_to_clip_space.invert().unwrap(),
        );
        for tri in tris {
            render_triangle(
                tri,
                rasterizer,
                world_to_camera_space,
                camera_to_clip_space,
                lights,
                ambient,
                texture,
                material,
            );
        }
    }
}

// Improvements: shouldn't have to compute inverses every time
fn render_triangle(
    clip_triangle: (Vertex4, Vertex4, Vertex4),
    rasterizer: & mut Rasterizer,
    world_to_camera_space: &Matrix4<f32>,
    camera_to_clip_space: &Matrix4<f32>,
    lights: &Vec<Light>,
    ambient: &FloatColor,
    texture: Option<&Texture>,
    material: &Material,
) {
    let clip_to_camera_space = camera_to_clip_space.invert().unwrap();
    let camera_to_world_space = world_to_camera_space.invert().unwrap();
    let camera0 = clip_triangle.0.transformed(clip_to_camera_space);
    let camera1 = clip_triangle.1.transformed(clip_to_camera_space);
    let camera2 = clip_triangle.2.transformed(clip_to_camera_space);
    let world0 = camera0.transformed(camera_to_world_space);
    let world1 = camera1.transformed(camera_to_world_space);
    let world2 = camera2.transformed(camera_to_world_space);
    let perspective_adjusted0 = clip_triangle.0.perspective_adjusted();
    let perspective_adjusted1 = clip_triangle.1.perspective_adjusted();
    let perspective_adjusted2 = clip_triangle.2.perspective_adjusted();
    rasterizer.triangle(
        (world0, world1, world2),
        (camera0, camera1, camera2),
        (perspective_adjusted0, perspective_adjusted1, perspective_adjusted2),
        lights,
        ambient,
        texture,
        material,
    );
}
