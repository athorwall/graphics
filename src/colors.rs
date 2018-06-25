use sdl2::pixels::Color;
use std::ops::*;
use std::iter::Sum;

#[derive(Copy, Clone, Debug)]
pub struct FloatColor {
    pub a: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl FloatColor {
    pub fn new(a: f32, r: f32, g: f32, b: f32) -> Self {
        FloatColor{a, r, g, b}
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(1.0, r, g, b)
    }

    pub fn from_argb(a: f32, r: f32, g: f32, b: f32) -> Self {
        Self::new(a, r, g, b)
    }

    pub fn from_rgb_u8s(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgb(
            Self::component_as_f32(r),
            Self::component_as_f32(g),
            Self::component_as_f32(b),
        )
    }

    pub fn from_argb_u8s(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self::from_argb(
            Self::component_as_f32(a),
            Self::component_as_f32(r),
            Self::component_as_f32(g),
            Self::component_as_f32(b),
        )
    }

    pub fn as_argb_u8s(&self) -> (u8, u8, u8, u8) {
        (
            Self::component_as_u8(self.a),
            Self::component_as_u8(self.r),
            Self::component_as_u8(self.g),
            Self::component_as_u8(self.b),
        )
    }

    pub fn as_rgb_u8s(&self) -> (u8, u8, u8) {
        (
            Self::component_as_u8(self.r),
            Self::component_as_u8(self.g),
            Self::component_as_u8(self.b),
        )
    }

    pub fn from_sdl_color(color: &Color) -> Self {
        Self::from_argb_u8s(
            color.a,
            color.r,
            color.g,
            color.b,
        )
    }

    pub fn as_sdl_color(&self) -> Color {
        Color::RGBA(
            Self::component_as_u8(self.r),
            Self::component_as_u8(self.g),
            Self::component_as_u8(self.b),
            Self::component_as_u8(self.a),
        )
    }

    pub fn mix_colors(colors: &Vec<Self>, weights: &Vec<f32>) -> Self {
        colors.iter().zip(weights).map(|(c, w)| *c * *w).sum()
    }

    pub fn multiply_many_colors(colors: &Vec<FloatColor>) -> Self {
        colors.iter().fold(FloatColor::from_rgb(1.0, 1.0, 1.0), |c1, c2| Self::multiply_colors(&c1, c2))
    }

    pub fn multiply_colors(color1: &FloatColor, color2: &FloatColor) -> Self {
        Self::from_argb(
            color1.a * color2.a,
            color1.r * color2.r,
            color1.g * color2.g,
            color1.b * color2.b,
        )
    }

    pub fn clamped(&self) -> Self {
        Self::from_argb(
            Self::clamp_component(self.a),
            Self::clamp_component(self.r),
            Self::clamp_component(self.g),
            Self::clamp_component(self.b),
        )
    }

    fn component_as_u8(component: f32) -> u8 {
        (component * 255.0) as u8
    }

    fn component_as_f32(component: u8) -> f32 {
        (component as f32) / 255.0
    }

    fn clamp_component(component: f32) -> f32 {
        if component > 1.0 {
            1.0
        } else if component < 0.0 {
            0.0
        } else {
            component
        }
    }
}

impl Mul<f32> for FloatColor {
    type Output = FloatColor;

    fn mul(self, rhs: f32) -> <Self as Mul<f32>>::Output {
        return FloatColor{
            a: self.a * rhs,
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Add<FloatColor> for FloatColor {
    type Output = FloatColor;

    fn add(self, rhs: FloatColor) -> <Self as Add<FloatColor>>::Output {
        return FloatColor{
            a: self.a + rhs.a,
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sum for FloatColor {
    fn sum<I: Iterator<Item=FloatColor>>(iter: I) -> Self {
        iter.fold(FloatColor::from_argb(0.0, 0.0, 0.0, 0.0), |a, b| a + b)
    }
}
