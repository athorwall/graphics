extern crate cgmath;
extern crate graphics;
extern crate sdl2;
extern crate num_traits;

use cgmath::*;
use graphics::camera::Camera;
use graphics::render::*;
use graphics::geometry::*;
use graphics::sdl_utils::*;
use sdl2::{
    event::Event,
};

fn main() {
    let ctx = sdl2::init().unwrap();
    let mut canvas = create_sdl_canvas(&ctx, 640, 480);

    let mut renderer = Renderer::create(640, 480);
    let mut mesh = Mesh::xy_face(1.0);

    let camera = Matrix4::look_at(
        Point3 { x: 0.0, y: 0.0, z: -3.0 },
        Point3 { x: 0.0, y: 0.0, z: 1.0 },
        Vector3{ x: 0.0, y: 1.0, z: 0.0 },
    );
    let perspective = Matrix4::from(perspective(Deg(90.0), 640.0 / 480.0, 0.1, 100.0));
    println!("{:?}", perspective);
    let transformation = perspective * camera.invert().unwrap();

    'main: loop {
        renderer.clear();
        renderer.mesh(&mesh.transformed(transformation));
        mesh.transform(Matrix4::from_angle_y(Deg(6.1)));

        render_to_canvas(&mut canvas, renderer.get_color_buffer());

        let mut events = ctx.event_pump().unwrap();
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'main,
                _               => continue
            }
        }
    }
}
