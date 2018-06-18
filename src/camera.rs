use cgmath::{
    Point3,
    Vector2,
    Vector3,
    Vector4,
    Matrix4,
    Matrix,
    SquareMatrix,
};
use math::RectBounds;

const INCH_TO_MM: f32 = 25.4;

pub struct Camera {
    // Millimeters
    pub focal_length: f32,

    // Inches
    pub film_aperture_width: f32,
    pub film_aperture_height: f32,

    pub near_clipping_plane: f32,
    pub far_clipping_plane: f32,

    /* TODO */
    pub fit_resolution_gate: u32,

    pub camera_to_world_matrix: Matrix4<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        return Camera {
            focal_length: 35.0,
            film_aperture_width: 0.825,
            film_aperture_height: 0.446,
            near_clipping_plane: 0.1,
            far_clipping_plane: 100.0,
            fit_resolution_gate: 0,
            camera_to_world_matrix: Matrix4::identity(),
        }
    }

    pub fn aperture_aspect_ratio(&self) -> f32 {
        return self.film_aperture_width / self.film_aperture_height;
    }

    pub fn canvas_bounds(&self) -> RectBounds<f32> {
        let top = ((self.film_aperture_height * INCH_TO_MM / 2.0) / self.focal_length)
            * self.near_clipping_plane;
        let right = top * (self.film_aperture_width / self.film_aperture_height);
        return RectBounds{
            top,
            bottom: -top,
            right,
            left: -right,
        }
    }

    pub fn compute_pixel_coordinates(
        &self,
        world_coordinates: Vector3<f32>
    ) -> Option<Vector3<f32>> {
        println!("World coordinates: {:?}", world_coordinates);
        let world_to_camera = self.camera_to_world_matrix.invert().unwrap();
        let camera_coordinates = world_to_camera * world_coordinates.extend(1.0);
        println!("Camera coordinates: {:?}", camera_coordinates);
        let screen_coordinates = Vector3 {
            x: camera_coordinates.x / -camera_coordinates.z * self.near_clipping_plane,
            y: camera_coordinates.y / -camera_coordinates.z * self.near_clipping_plane,
            z: -camera_coordinates.z,
        };
        println!("Screen coordinates: {:?}", screen_coordinates);
        let bounds = self.canvas_bounds();
        let ndc_coordinates = Vector3 {
            x: (screen_coordinates.x + bounds.right) / (2.0 * bounds.right),
            y: (screen_coordinates.y + bounds.top) / (2.0 * bounds.top),
            z: screen_coordinates.z,
        };
        if ndc_coordinates.x < 0.0
            || ndc_coordinates.x > 1.0
            || ndc_coordinates.y < 0.0
            || ndc_coordinates.y > 1.0 {
            return None;
        } else {
            return Some(ndc_coordinates);
        }
    }
}
