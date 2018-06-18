use cgmath::*;
use sdl2::{
    pixels::Color,
};

pub struct Vertex {
    pub position: Vector3<f32>,
    pub color: Color,
}