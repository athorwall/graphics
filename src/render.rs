use cgmath::*;
use math::*;
use geometry::*;
use rasterizer::*;
use light::*;
use colors::*;
use materials::*;
use textures::*;

pub struct RenderFragmentContext {
}

// todo: perform lighting calculations in camera space
pub fn render_mesh(
    mesh: &Mesh,
    rasterizer: & mut Rasterizer,
    world_to_camera_space: &Matrix4<f32>,
    camera_to_clip_space: &Matrix4<f32>,
    lights: &Vec<Light>,
    ambient: &FloatColor,
    texture: Option<&Texture>,
    material: &Material,
    fragment_processor: &Fn(
        &Vector4<f32>,
        &Vector4<f32>,
        &Vector2<f32>,
        &Vec<Light>,
        &FloatColor,
        &Option<&Texture>,
        &Material,
    ) -> FloatColor,
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
                fragment_processor,
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
    fragment_processor: &Fn(
        &Vector4<f32>,
        &Vector4<f32>,
        &Vector2<f32>,
        &Vec<Light>,
        &FloatColor,
        &Option<&Texture>,
        &Material,
    ) -> FloatColor,
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
        &|coords, normals, uvs| {
            fragment_processor(coords, normals, uvs, lights, ambient, &texture, material)
        },
    );
}
