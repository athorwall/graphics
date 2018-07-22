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
use graphics::render::*;

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
            &process_fragment,
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
            &process_fragment,
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

fn process_fragment(
    world_coordinates: &Vector4<f32>,
    world_normals: &Vector4<f32>,
    uvs: &Vector2<f32>,
    lights: &Vec<Light>,
    ambient: &FloatColor,
    texture: &Option<&Texture>,
    material: &Material,
) -> FloatColor {
    let texture_color = match texture {
        Some(ref t) => {
            FloatColor::from_sdl_color(&t.sample(uvs.x, uvs.y, TextureFilterMode::NearestNeighbor))
        },
        None => FloatColor::from_rgb(1.0, 1.0, 1.0),
    };
    let color_from_lights = lights.iter()
        .map(|light| {
            process_fragment_light(
                &world_coordinates,
                &world_normals,
                light,
                material,
            )
        })
        .sum();
    let ambient_color = FloatColor::multiply_colors(ambient, &material.ambient);
    let total_light_color = ambient_color + color_from_lights;

    let net_color = FloatColor::multiply_colors(&texture_color, &total_light_color);
    net_color.clamped()
}

fn process_fragment_light(
    world_coordinates: &Vector4<f32>,
    world_normals: &Vector4<f32>,
    light: &Light,
    material: &Material,
) -> FloatColor {
    let intensity = match light.light_type {
        LightType::Directional(ref directional_light) => {
            let intensity = directional_light.direction.dot(world_normals.truncate());
            if intensity < 0.0 { 0.0 } else { intensity }
        }
        LightType::Point(ref point_light) => {
            let ray = point_light.position - world_coordinates.truncate();
            let distance = ray.magnitude();
            let normalized_ray = ray / distance;
            let intensity = normalized_ray.dot(world_normals.truncate()) / (distance * distance);
            if intensity < 0.0 { 0.0 } else { intensity }
        }
    };
    FloatColor::multiply_colors(&material.diffuse, &light.color) * intensity
}

