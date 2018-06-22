
use frame::*;
use sdl2::pixels::Color;

pub struct Texture {
    buffer: Frame<Color>,
}

impl Texture {
    pub fn create(buffer: Frame<Color>) -> Self {
        return Texture{
            buffer,
        };
    }

    pub fn sample(&self, x: f32, y: f32) -> Color {
        let ix = (x * (self.buffer.width() as f32).round()) as i32;
        let iy = (y * (self.buffer.height() as f32).round()) as i32;
        let constrained_ix = Self::constrain(ix, 0, self.buffer.width() as i32 - 1);
        let constrained_iy = Self::constrain(iy, 0, self.buffer.height() as i32 - 1);
        return self.buffer.at(constrained_ix as usize, constrained_iy as usize).unwrap();
    }

    // make generic
    fn constrain(x: i32, lower: i32, upper: i32) -> i32 {
        if x > upper {
            upper
        } else if x < lower {
            lower
        } else {
            x
        }
    }
}