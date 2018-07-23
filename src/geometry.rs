use cgmath::*;
use math::*;
use std::f32::consts::PI;

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
            normal: self.normal.extend(1.0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex4 {
    pub position: Vector4<f32>,
    pub uv: Vector2<f32>,
    // could be Vector3 but makes clipping easier this way
    pub normal: Vector4<f32>,
}

impl Vertex4 {
    pub fn transformed(&self, transformation: Matrix4<f32>) -> Self {
        let normal_matrix = transformation.invert().unwrap().transpose();
        return Vertex4 {
            position: transformation * self.position,
            uv: self.uv,
            normal: normal_matrix * self.normal,
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

/*
pub fn clip_triangle(triangle: &(&Vertex4, &Vertex4, &Vertex4)) -> Vec<(Vertex4, Vertex4, Vertex4)> {
    let mut new_triangle = triangle.clone();
    let points = vec![
        triangle.0.position,
        triangle.1.position,
        triangle.2.position,
    ];
    let clipped_points = clip_in_box(&points);
}
*/

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
                (1, 3, 2),
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
                (0, 1, 2),
                (1, 3, 2),
                (1, 5, 3),
                (5, 7, 3),
                (4, 0, 2),
                (4, 2, 6),
                (4, 6, 5),
                (5, 6, 7),
                (0, 4, 1),
                (1, 4, 5),
                (2, 3, 6),
                (3, 7, 6),
            ],
        );
        cube.compute_normals();
        return cube;
    }

    pub fn sphere(size: f32, smoothness: usize) -> Self {
        let segments = smoothness * 4;
        let segment_angle = Rad(2.0 * PI / (segments as f32));
        let mut vertices = vec![];
        for i in 0..smoothness {
            if i == 0 {
                vertices.push((
                    Vector3{x: 0.0, y: size, z: 0.0},
                    Self::sphere_vertex_position(size, segment_angle, segment_angle),
                    Self::sphere_vertex_position(size, segment_angle, Rad(0.0)),
                ));
            } else {
                vertices.push((
                    Self::sphere_vertex_position(size, segment_angle * i as f32, Rad(0.0)),
                    Self::sphere_vertex_position(size, segment_angle * i as f32, segment_angle),
                    Self::sphere_vertex_position(size, segment_angle * (i + 1) as f32, segment_angle),
                ));
                vertices.push((
                    Self::sphere_vertex_position(size, segment_angle * (i + 1) as f32, segment_angle),
                    Self::sphere_vertex_position(size, segment_angle * (i + 1) as f32, Rad(0.0)),
                    Self::sphere_vertex_position(size, segment_angle * i as f32, Rad(0.0)),
                ));
            }
        }
        let computed_vertices = vertices.iter()
            .map(|(v0, v1, v2)| (
                Vertex3{
                    position: *v0,
                    uv: Vector2{x: 0.0, y: 0.0},
                    normal: *v0 / v0.magnitude(),
                },
                Vertex3{
                    position: *v1,
                    uv: Vector2{x: 0.0, y: 0.0},
                    normal: *v1 / v1.magnitude(),
                },
                Vertex3{
                    position: *v2,
                    uv: Vector2{x: 0.0, y: 0.0},
                    normal: *v2 / v2.magnitude(),
                },
            ))
            .collect();
        let mut mesh = Mesh{vertices: computed_vertices};
        for i in 1..segments {
            let mut next_mesh = mesh.clone();
            next_mesh.transform(Matrix4::from_angle_y(segment_angle * i as f32));
            mesh.vertices.append(&mut next_mesh.vertices);
        }
        let mut next_mesh = mesh.clone();
        next_mesh.transform(Matrix4::from_angle_x(Rad(PI)));
        mesh.vertices.append(&mut next_mesh.vertices);
        mesh.compute_normals();
        mesh
    }

    fn sphere_vertex_position(
        size: f32,
        polar_angle: Rad<f32>,
        azimuthal_angle: Rad<f32>,
    ) -> Vector3<f32> {
        Vector3{
            x: size * polar_angle.sin() * azimuthal_angle.cos(),
            y: size * polar_angle.cos(),
            z: size * polar_angle.sin() * azimuthal_angle.sin(),
        }
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