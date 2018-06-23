use cgmath::*;

pub struct Light {
    pub light_type: LightType,
}

impl Light {
    pub fn point_light(pos: Vector3<f32>) -> Self {
        Light{
            light_type: LightType::Point(PointLight{
                position: pos,
            })
        }
    }

    pub fn directional_light(dir: Vector3<f32>) -> Self {
        Light{
            light_type: LightType::Directional(DirectionalLight{
                direction: dir,
            })
        }
    }
}

pub enum LightType {
    Point(PointLight),
    Directional(DirectionalLight),
}

pub struct PointLight {
    pub position: Vector3<f32>,
}

pub struct DirectionalLight {
    pub direction: Vector3<f32>,
}

