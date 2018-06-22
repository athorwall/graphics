use cgmath::*;
use sdl2::{
    pixels::Color,
};

#[derive(Clone, Copy, Debug)]
pub struct Vertex3 {
    pub position: Vector3<f32>,
    pub color: Color,
    pub uv: Vector2<f32>,
}

impl Vertex3 {
    pub fn transformed(&self, transformation: Matrix4<f32>) -> Self {
        return Vertex3 {
            position: (transformation * self.position.extend(1.0)).truncate(),
            color: self.color,
            uv: self.uv,
        };
    }

    pub fn to_vertex4(&self, w: f32) -> Vertex4 {
        return Vertex4{
            position: self.position.extend(w),
            color: self.color,
            uv: self.uv,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex4 {
    pub position: Vector4<f32>,
    pub color: Color,
    pub uv: Vector2<f32>,
}

impl Vertex4 {
    pub fn transformed(&self, transformation: Matrix4<f32>) -> Self {
        return Vertex4 {
            position: transformation * self.position,
            color: self.color,
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
    pub vertices: Vec<Vertex3>,
    pub triangles: Vec<(usize, usize, usize)>,
}

impl Mesh {
    pub fn xy_face(size: f32) -> Self {
        return Mesh{
            vertices: vec![
                Vertex3 {
                    position: Vector3{x: -size / 2.0, y: -size / 2.0, z: 0.0},
                    color: Color::RGB(255, 255, 255),
                    uv: Vector2{x: 0.0, y: 0.0},
                },
                Vertex3 {
                    position: Vector3{x: size / 2.0, y: -size / 2.0, z: 0.0},
                    color: Color::RGB(255, 255, 255),
                    uv: Vector2{x: 1.0, y: 0.0},
                },
                Vertex3 {
                    position: Vector3{x: -size / 2.0, y: size / 2.0, z: 0.0},
                    color: Color::RGB(255, 255, 255),
                    uv: Vector2{x: 0.0, y: 1.0},
                },
                Vertex3 {
                    position: Vector3{x: size / 2.0, y: size / 2.0, z: 0.0},
                    color: Color::RGB(255, 255, 255),
                    uv: Vector2{x: 1.0, y: 1.0},
                },
            ],
            triangles: vec![
                (0, 1, 2),
                (1, 2, 3),
            ],
        };
    }

    pub fn cube(size: f32) -> Self {
        return Mesh{
            vertices: vec![
                Vertex3 {
                    position: Vector3{x: -size / 2.0, y: -size / 2.0, z: size / 2.0},
                    color: Color::RGB(255, 255, 255),
                    uv: Vector2{x: 0.0, y: 0.0},
                },
                Vertex3 {
                    position: Vector3{x: size / 2.0, y: -size / 2.0, z: size / 2.0},
                    color: Color::RGB(255, 255, 255),
                    uv: Vector2{x: 1.0, y: 0.0},
                },
                Vertex3 {
                    position: Vector3{x: -size / 2.0, y: size / 2.0, z: size / 2.0},
                    color: Color::RGB(255, 255, 255),
                    uv: Vector2{x: 0.0, y: 1.0},
                },
                Vertex3 {
                    position: Vector3{x: size / 2.0, y: size / 2.0, z: size / 2.0},
                    color: Color::RGB(255, 255, 255),
                    uv: Vector2{x: 1.0, y: 1.0},
                },
                Vertex3 {
                    position: Vector3{x: -size / 2.0, y: -size / 2.0, z: -size / 2.0},
                    color: Color::RGB(255, 255, 255),
                    uv: Vector2{x: 1.0, y: 0.0},
                },
                Vertex3 {
                    position: Vector3{x: size / 2.0, y: -size / 2.0, z: -size / 2.0},
                    color: Color::RGB(255, 255, 255),
                    uv: Vector2{x: 0.0, y: 0.0},
                },
                Vertex3 {
                    position: Vector3{x: -size / 2.0, y: size / 2.0, z: -size / 2.0},
                    color: Color::RGB(255, 255, 255),
                    uv: Vector2{x: 1.0, y: 1.0},
                },
                Vertex3 {
                    position: Vector3{x: size / 2.0, y: size / 2.0, z: -size / 2.0},
                    color: Color::RGB(255, 255, 255),
                    uv: Vector2{x: 0.0, y: 1.0},
                },
            ],
            triangles: vec![
                (0, 1, 2),
                (1, 2, 3),
                (1, 5, 3),
                (5, 3, 7),
                (4, 0, 2),
                (4, 2, 6),
                (4, 5, 6),
                (5, 6, 7),
                (0, 1, 4),
                (1, 4, 5),
                (2, 3, 6),
                (3, 6, 7),
            ],
        };
    }

    pub fn combine(mesh1: &Mesh, mesh2: &Mesh) -> Self {
        return Self::combine_many(&vec![mesh1, mesh2]);
    }

    pub fn combine_many(meshes: &Vec<&Mesh>) -> Self {
        if meshes.len() == 0 {
            return Mesh{
                vertices: vec![],
                triangles: vec![],
            }
        } else {
            let mut vertices = vec![];
            let mut triangles = vec![];
            for mesh in meshes {
                let shifted_triangles: Vec<(usize, usize, usize)> = mesh.triangles.iter()
                    .map(|(u1, u2, u3)|
                        (u1 + vertices.len(), u2 + vertices.len(), u3 + vertices.len()))
                    .collect();
                triangles.extend(&shifted_triangles);
                vertices.extend(&mesh.vertices);
            }
            return Mesh {
                vertices,
                triangles,
            };
        }
    }

    pub fn transform(&mut self, transformation: Matrix4<f32>) {
        for vertex in &mut self.vertices {
            let homogenous_coordinates = transformation * vertex.position.extend(1.0);
            vertex.position = Vector3{
                x: homogenous_coordinates.x / homogenous_coordinates.w,
                y: homogenous_coordinates.y / homogenous_coordinates.w,
                z: homogenous_coordinates.z / homogenous_coordinates.w,
            };
        }
    }

    pub fn transformed(&self, transformation: Matrix4<f32>) -> Self {
        let mut mesh = self.clone();
        for vertex in &mut mesh.vertices {
            let homogenous_coordinates = transformation * vertex.position.extend(1.0);
            vertex.position = Vector3{
                x: homogenous_coordinates.x / homogenous_coordinates.w,
                y: homogenous_coordinates.y / homogenous_coordinates.w,
                z: homogenous_coordinates.z / homogenous_coordinates.w,
            };
        }
        return mesh;
    }
}