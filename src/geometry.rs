use cgmath::*;
use sdl2::{
    pixels::Color,
};

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub color: Color,
}

impl Vertex {
    pub fn transformed(&self, transformation: Matrix4<f32>) -> Self {
        return Vertex{
            position: (transformation * self.position.extend(1.0)).truncate(),
            color: self.color,
        };
    }
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<(usize, usize, usize)>,
}

impl Mesh {
    pub fn xy_face(size: f32) -> Self {
        return Mesh{
            vertices: vec![
                Vertex{
                    position: Vector3{x: -size / 2.0, y: -size / 2.0, z: 0.0},
                    color: Color::RGB(255, 255, 255),
                },
                Vertex{
                    position: Vector3{x: size / 2.0, y: -size / 2.0, z: 0.0},
                    color: Color::RGB(255, 255, 255),
                },
                Vertex{
                    position: Vector3{x: -size / 2.0, y: size / 2.0, z: 0.0},
                    color: Color::RGB(255, 255, 255),
                },
                Vertex{
                    position: Vector3{x: size / 2.0, y: size / 2.0, z: 0.0},
                    color: Color::RGB(255, 255, 255),
                },
            ],
            triangles: vec![
                (0, 1, 2),
                (1, 2, 3),
            ],
        };
    }

    pub fn cube(size: f32) -> Self {
        let mut face1 = Self::xy_face(size);
        face1.transform(Matrix4::from_translation(Vector3{x: 0.0, y: 0.0, z: size / 2.0}));
        let mut face2 = Self::xy_face(size);
        face2.transform(Matrix4::from_translation(Vector3{x: 0.0, y: 0.0, z: -size / 2.0}));
        let mut face3 = Self::xy_face(size);
        face3.transform(Matrix4::from_angle_y(Deg(90.0)));
        face3.transform(Matrix4::from_translation(Vector3{x: size / 2.0, y: 0.0, z: 0.0}));
        let mut face4 = Self::xy_face(size);
        face4.transform(Matrix4::from_angle_y(Deg(90.0)));
        face4.transform(Matrix4::from_translation(Vector3{x: -size / 2.0, y: 0.0, z: 0.0}));
        let mut face5 = Self::xy_face(size);
        face5.transform(Matrix4::from_angle_x(Deg(90.0)));
        face5.transform(Matrix4::from_translation(Vector3{x: 0.0, y: size / 2.0, z: 0.0}));
        let mut face6 = Self::xy_face(size);
        face6.transform(Matrix4::from_angle_x(Deg(90.0)));
        face6.transform(Matrix4::from_translation(Vector3{x: 0.0, y: -size / 2.0, z: 0.0}));
        return Self::combine_many(&vec![
            &face1,
            &face2,
            &face3,
            &face4,
            &face5,
            &face6,
        ]);
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