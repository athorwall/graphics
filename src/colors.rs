use sdl2::pixels::Color;
use std::ops::*;
use cgmath::*;
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

    pub fn mix_colors(colors: &Vec<Self>, weights: &Vec<f32>) -> Self {
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;
        return colors.iter().zip(weights).map(|(c, w)| *c * *w).sum();
    }

    fn component_as_u8(component: f32) -> u8 {
        (component * 255.0) as u8
    }

    fn component_as_f32(component: u8) -> f32 {
        (component as f32) / 255.0
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
