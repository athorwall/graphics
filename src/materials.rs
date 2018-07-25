
use colors::*;

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub diffuse: FloatColor,
    pub ambient: FloatColor,
    pub specular: FloatColor,
    // This isn't good, because it enforces ideas on Material that should be flexible across
    // different fragment processors, namely that a material is associated with one particular
    // texture.
    pub texture: Option<usize>,
}

impl Material {
    pub fn new() -> Self {
        Material{
            diffuse: FloatColor::from_rgb(1.0, 1.0, 1.0),
            ambient: FloatColor::from_rgb(1.0, 1.0, 1.0),
            specular: FloatColor::from_rgb(1.0, 1.0, 1.0),
            texture: None,
        }
    }
}