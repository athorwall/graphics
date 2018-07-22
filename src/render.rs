use cgmath::*;
use math::*;
use geometry::*;
use rasterizer::*;
use light::*;
use colors::*;
use materials::*;
use textures::*;
use std::collections::HashMap;
use sdl_utils::*;
use sdl2::{
    render::Canvas,
    video::Window,
};
use camera::*;

// Right now, Renderer takes ownership of rasterizer, canvas, and textures.
// Not sure if that should be the case.
pub struct Renderer {
    // TODO: no pub, also should Renderer be responsible for this stuff?
    pub rasterizer: Rasterizer,
    pub canvas: Canvas<Window>,

    // TODO: replace with some kind of vertex-shader equivalent
    pub model_view: Matrix4<f32>,
    pub world_to_view_matrix: Matrix4<f32>,
    pub projection: Matrix4<f32>,

    pub lighting: Lighting,
    // TODO: HashMap<usize, Texture>
    pub textures: HashMap<usize, Texture>,
    pub material: Material,
}

impl Renderer {

    pub fn new(rasterizer: Rasterizer, canvas: Canvas<Window>) -> Self {
        Renderer{
            rasterizer,
            canvas,
            model_view: Matrix4::identity(),
            world_to_view_matrix: Matrix4::identity(),
            projection: Matrix4::from(perspective(Deg(70.0), 1000.0 / 800.0, 0.1, 100.0)),
            lighting: Lighting{
                lights: vec![
                    Light::point_light(Vector3{x: 1.0, y: 1.0, z: 1.0}),
                ],
                ambient: FloatColor::from_rgb(0.3, 0.3, 0.3),
            },
            textures: HashMap::new(),
            material: Material{
                diffuse: FloatColor::from_rgb(1.0, 1.0, 1.0),
                specular: FloatColor::from_rgb(1.0, 1.0, 1.0),
                ambient: FloatColor::from_rgb(1.0, 1.0, 1.0),
                texture: None,
            },
        }
    }

    pub fn set_from_camera(&mut self, camera: &Camera) {
        self.world_to_view_matrix = camera.eye().invert().unwrap();
        self.projection = camera.projection();
    }

    pub fn set_texture(&mut self, index: usize, texture: Texture) {
        self.textures.insert(index, texture);
    }

    // todo: perform lighting calculations in camera space
    pub fn mesh(&mut self, mesh: &Mesh) {
        for tri in &mesh.vertices {
            self.triangle(tri.0, tri.1, tri.2);
        }
    }

    // Improvements: shouldn't have to compute inverses every time
    pub fn triangle(&mut self, v0: Vertex3, v1: Vertex3, v2: Vertex3) {
        // First we need to transform our vertices to clip space, for clipping. Then,
        // we'll need to transform the resulting vertices back to camera and world space,
        // because our rasterizer needs all three to work (maybe it shouldn't?).
        let world_to_clip_space = self.projection * self.world_to_view_matrix;
        let clip0 = v0.to_vertex4(1.0).transformed(world_to_clip_space);
        let clip1 = v1.to_vertex4(1.0).transformed(world_to_clip_space);
        let clip2 = v2.to_vertex4(1.0).transformed(world_to_clip_space);
        let tris = clip_triangle(
            clip0,
            clip1,
            clip2,
            self.projection.invert().unwrap(),
        );
        for tri in tris {
            self.render_clip_triangle(tri.0, tri.1, tri.2);
        }
    }

    fn render_clip_triangle(&mut self, v0: Vertex4, v1: Vertex4, v2: Vertex4) {
        let clip_to_camera_space = self.projection.invert().unwrap();
        let camera_to_world_space = self.world_to_view_matrix.invert().unwrap();
        let camera0 = v0.transformed(clip_to_camera_space);
        let camera1 = v1.transformed(clip_to_camera_space);
        let camera2 = v2.transformed(clip_to_camera_space);
        let world0 = camera0.transformed(camera_to_world_space);
        let world1 = camera1.transformed(camera_to_world_space);
        let world2 = camera2.transformed(camera_to_world_space);
        let perspective_adjusted0 = v0.perspective_adjusted();
        let perspective_adjusted1 = v1.perspective_adjusted();
        let perspective_adjusted2 = v2.perspective_adjusted();

        let lights = &self.lighting.lights;
        let ambient = &self.lighting.ambient;
        let textures = &self.textures;
        let material = &self.material;

        self.rasterizer.triangle(
            (world0, world1, world2),
            (camera0, camera1, camera2),
            (perspective_adjusted0, perspective_adjusted1, perspective_adjusted2),
            &|coords, normals, uvs| {
                process_fragment(
                    coords,
                    normals,
                    uvs,
                    lights,
                    ambient,
                    textures,
                    material,
                )
            },
        );
    }

    pub fn present(&mut self) {
        render_to_canvas(&mut self.canvas, self.rasterizer.get_color_buffer());
        self.rasterizer.clear();
    }
}

pub struct RenderFragmentContext {
}

fn process_fragment(
    world_coordinates: &Vector4<f32>,
    world_normals: &Vector4<f32>,
    uvs: &Vector2<f32>,
    lights: &Vec<Light>,
    ambient: &FloatColor,
    textures: &HashMap<usize, Texture>,
    material: &Material,
) -> FloatColor {
    let texture = material.texture
        .and_then(|index| textures.get(&index));
    let texture_color = match texture {
        Some(ref t) => {
            FloatColor::from_sdl_color(&t.sample(uvs.x, uvs.y, TextureFilterMode::Bilinear))
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
