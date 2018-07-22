
use colors::*;

pub struct Material {
    pub diffuse: FloatColor,
    pub ambient: FloatColor,
    pub specular: FloatColor,
    // This isn't good, because it enforces ideas on Material that should be flexible across
    // different fragment processors, namely that a material is associated with one particular
    // texture.
    pub texture: Option<usize>,
}