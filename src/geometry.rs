use cgmath::*;
use sdl2::{
    pixels::Color,
};
use colors::*;

#[derive(Clone, Copy, Debug)]
pub struct Vertex3 {
    pub position: Vector3<f32>,
    pub uv: Vector2<f32>,
    pub normal: Vector3<f32>,
}

impl Vertex3 {
    pub fn transform_with_correction(&mut self, transformation: Matrix4<f32>) {
        let homogenous_coordinates = transformation * self.position.extend(1.0);
        let normal_matrix = transformation.invert().unwrap().transpose();
        self.position = Vector3{
            x: homogenous_coordinates.x / homogenous_coordinates.w,
            y: homogenous_coordinates.y / homogenous_coordinates.w,
            z: homogenous_coordinates.z / homogenous_coordinates.w,
        };
        self.normal = (normal_matrix * self.normal.extend(1.0)).truncate();
    }

    pub fn transformed(&self, transformation: Matrix4<f32>) -> Self {
        let normal_matrix = transformation.invert().unwrap().transpose();
        return Vertex3 {
            position: (transformation * self.position.extend(1.0)).truncate(),
            uv: self.uv,
            normal: (normal_matrix * self.normal.extend(1.0)).truncate(),
        };
    }

    pub fn transformed_with_correction(&self, transformation: Matrix4<f32>) -> Self {
        let mut transformed = self.clone();
        transformed.transform_with_correction(transformation);
        return transformed;
    }

    pub fn to_vertex4(&self, w: f32) -> Vertex4 {
        return Vertex4{
            position: self.position.extend(w),
            uv: self.uv,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex4 {
    pub position: Vector4<f32>,
    pub uv: Vector2<f32>,
}

impl Vertex4 {
    pub fn transformed(&self, transformation: Matrix4<f32>) -> Self {
        return Vertex4 {
            position: transformation * self.position,
            uv: self.uv,
        };
    }

    pub fn perspective_adjusted(&self) -> Self {
        let mut adjusted_vertex = self.clone();
        adjusted_vertex.position.x /= adjusted_vertex.position.w;
        adjusted_vertex.position.y /= adjusted_vertex.position.w;
        adjusted_vertex.position.z /= adjusted_vertex.position.w;
        return adjusted_vertex;
    }
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<(Vertex3, Vertex3, Vertex3)>,
}

impl Mesh {
    pub fn from_triangles(vertices: &Vec<Vertex3>, triangles: &Vec<(usize, usize, usize)>) -> Self {
        let vertices = triangles.iter().map(|tri| {
            (vertices[tri.0], vertices[tri.1], vertices[tri.2])
        }).collect();
        return Mesh{
            vertices,
        };
    }

    pub fn xy_face(size: f32) -> Self {
        return Self::from_triangles(
            &vec![
                Vertex3 {
                    position: Vector3{x: -size / 2.0, y: -size / 2.0, z: 0.0},
                    uv: Vector2{x: 0.0, y: 0.0},
                    normal: Vector3{x: 0.0, y: 0.0, z: 1.0},
                },
                Vertex3 {
                    position: Vector3{x: size / 2.0, y: -size / 2.0, z: 0.0},
                    uv: Vector2{x: 1.0, y: 0.0},
                    normal: Vector3{x: 0.0, y: 0.0, z: 1.0},
                },
                Vertex3 {
                    position: Vector3{x: -size / 2.0, y: size / 2.0, z: 0.0},
                    uv: Vector2{x: 0.0, y: 1.0},
                    normal: Vector3{x: 0.0, y: 0.0, z: 1.0},
                },
                Vertex3 {
                    position: Vector3{x: size / 2.0, y: size / 2.0, z: 0.0},
                    uv: Vector2{x: 1.0, y: 1.0},
                    normal: Vector3{x: 0.0, y: 0.0, z: 1.0},
                },
            ],
            &vec![
                (0, 1, 2),
                (1, 2, 3),
            ],
        );
    }

    pub fn cube(size: f32) -> Self {
        let mut cube = Self::from_triangles(
            &vec![
                Vertex3 {
                    position: Vector3{x: -size / 2.0, y: -size / 2.0, z: size / 2.0},
                    uv: Vector2{x: 0.0, y: 0.0},
                    normal: Vector3{x: 0.0, y: 0.0, z: 1.0},
                },
                Vertex3 {
                    position: Vector3{x: size / 2.0, y: -size / 2.0, z: size / 2.0},
                    uv: Vector2{x: 1.0, y: 0.0},
                    normal: Vector3{x: 0.0, y: 0.0, z: 1.0},
                },
                Vertex3 {
                    position: Vector3{x: -size / 2.0, y: size / 2.0, z: size / 2.0},
                    uv: Vector2{x: 0.0, y: 1.0},
                    normal: Vector3{x: 0.0, y: 0.0, z: 1.0},
                },
                Vertex3 {
                    position: Vector3{x: size / 2.0, y: size / 2.0, z: size / 2.0},
                    uv: Vector2{x: 1.0, y: 1.0},
                    normal: Vector3{x: 0.0, y: 0.0, z: 1.0},
                },
                Vertex3 {
                    position: Vector3{x: -size / 2.0, y: -size / 2.0, z: -size / 2.0},
                    uv: Vector2{x: 1.0, y: 0.0},
                    normal: Vector3{x: 0.0, y: 0.0, z: 1.0},
                },
                Vertex3 {
                    position: Vector3{x: size / 2.0, y: -size / 2.0, z: -size / 2.0},
                    uv: Vector2{x: 0.0, y: 0.0},
                    normal: Vector3{x: 0.0, y: 0.0, z: 1.0},
                },
                Vertex3 {
                    position: Vector3{x: -size / 2.0, y: size / 2.0, z: -size / 2.0},
                    uv: Vector2{x: 1.0, y: 1.0},
                    normal: Vector3{x: 0.0, y: 0.0, z: 1.0},
                },
                Vertex3 {
                    position: Vector3{x: size / 2.0, y: size / 2.0, z: -size / 2.0},
                    uv: Vector2{x: 0.0, y: 1.0},
                    normal: Vector3{x: 0.0, y: 0.0, z: 1.0},
                },
            ],
            &vec![
                (0, 2, 1),
                (1, 2, 3),
                (1, 3, 5),
                (5, 3, 7),
                (4, 2, 0),
                (4, 6, 2),
                (4, 5, 6),
                (5, 7, 6),
                (0, 1, 4),
                (1, 5, 4),
                (2, 6, 3),
                (3, 6, 7),
            ],
        );
        cube.compute_normals();
        return cube;
    }

    pub fn transform(&mut self, transformation: Matrix4<f32>) {
        for triangle in &mut self.vertices {
            triangle.0.transform_with_correction(transformation);
            triangle.1.transform_with_correction(transformation);
            triangle.2.transform_with_correction(transformation);
        }
    }

    pub fn transformed(&self, transformation: Matrix4<f32>) -> Self {
        let mut mesh = self.clone();
        for (v0, v1, v2) in &mut mesh.vertices {
            v0.transform_with_correction(transformation);
            v1.transform_with_correction(transformation);
            v2.transform_with_correction(transformation);
        }
        return mesh;
    }

    pub fn compute_normals(&mut self) {
        for (v0, v1, v2) in &mut self.vertices {
            let normal = (v1.position - v0.position).cross(v2.position - v0.position);
            let normalized_normal = normal / normal.magnitude();
            v0.normal = normalized_normal;
            v1.normal = normalized_normal;
            v2.normal = normalized_normal;
        }
    }
}