
use frame::*;
use sdl2::pixels::Color;
use cgmath::BaseNum;
use colors::*;

pub struct Texture {
    buffer: Frame<Color>,
}

impl Texture {
    pub fn create(buffer: Frame<Color>) -> Self {
        return Texture{
            buffer,
        };
    }

    pub fn sample(&self, x: f32, y: f32, filter_mode: TextureFilterMode) -> Color {
        let tx = Self::constrain(x * self.buffer.width() as f32, 0.0, self.buffer.width() as f32);
        let ty = Self::constrain(y * self.buffer.height() as f32, 0.0, self.buffer.height() as f32);
        match filter_mode {
            TextureFilterMode::NearestNeighbor => {
                self.sample_safely(tx as i32, ty as i32)
            },
            TextureFilterMode::Bilinear => {
                let txr = tx.round();
                let tyr = ty.round();
                let dx = (tx - txr) + 0.5;
                let dy = (ty - tyr) + 0.5;
                let c00 = self.sample_safely((txr - 1.0) as i32, (tyr - 1.0) as i32);
                let c01 = self.sample_safely(txr as i32, (tyr - 1.0) as i32);
                let c10 = self.sample_safely((txr - 1.0) as i32, tyr as i32);
                let c11 = self.sample_safely(txr as i32, tyr as i32);
                let combined = FloatColor::from_sdl_color(&c00) * (1.0 - dx) * (1.0 - dy)
                    + FloatColor::from_sdl_color(&c01) * dx * (1.0 - dy)
                    + FloatColor::from_sdl_color(&c10) * (1.0 - dx) * dy
                    + FloatColor::from_sdl_color(&c11) * dx * dy;
                combined.as_sdl_color()
            },
        }
    }

    fn sample_safely(&self, x: i32, y: i32) -> Color {
        let sx = Self::constrain(x, 0, self.buffer.width() as i32 - 1);
        let sy = Self::constrain(y, 0, self.buffer.height() as i32 - 1);
        return self.buffer.at(sx as usize, sy as usize).unwrap();
    }

    fn constrain<T>(x: T, lower: T, upper: T) -> T where T: BaseNum {
        if x > upper {
            upper
        } else if x < lower {
            lower
        } else {
            x
        }
    }
}

pub enum TextureFilterMode {
    NearestNeighbor,
    Bilinear,
}
