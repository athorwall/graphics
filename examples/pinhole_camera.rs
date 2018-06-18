extern crate cgmath;
extern crate graphics;
extern crate sdl2;
extern crate num_traits;

use cgmath::*;
use graphics::camera::Camera;
use graphics::render::*;
use graphics::geometry::*;
use sdl2::{
    pixels::Color,
    event::Event,
};

fn main() {
    let mut camera = Camera::new();
    camera.camera_to_world_matrix = Matrix4::look_at(
        Point3 { x: 0.0, y: 0.0, z: -6.0 },
        Point3 { x: 0.0, y: 0.0, z: 1.0 },
        Vector3{ x: 0.0, y: 1.0, z: 0.0 });
    let mut renderer = Renderer::create(640, 480);
    let mut x = 0.2;
    'main: loop {
        renderer.clear();
        renderer.triangle(
            Vertex {
                position: Vector3 { x, y: 0.2, z: 2.0 },
                color: Color::RGB(255, 0, 0)
            },
            Vertex {
                position: Vector3 { x: 0.2, y: 0.5, z: 2.0 },
                color: Color::RGB(0, 255, 0)
            },
            Vertex {
                position: Vector3 { x: 0.5, y: 0.2, z: 2.0 },
                color: Color::RGB(0, 0, 255)
            },
        );
        renderer.present();
        x += 0.001;

        let mut events = renderer.get_events();
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'main,
                _               => continue
            }
        }
    }
}
