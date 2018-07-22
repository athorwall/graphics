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
        &|coords, normals, uvs| {
            process_fragment(coords, normals, uvs, lights, ambient, &texture, material)
        },
    );
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
