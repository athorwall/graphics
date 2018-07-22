use cgmath::*;
use sdl2::keyboard::*;

pub struct Camera {
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,
    eye: Matrix4<f32>,
}

impl Camera {

    pub fn create(fov: f32, aspect: f32, near: f32, far: f32, eye: Matrix4<f32>) -> Self {
        Camera{fov, aspect, near, far, eye}
    }

    pub fn eye(&self) -> Matrix4<f32> {
        self.eye
    }

    pub fn projection(&self) -> Matrix4<f32> {
        Matrix4::from(perspective(Deg(self.fov), self.aspect, self.near, self.far))
    }

    pub fn move_forward(&mut self, distance: f32) {
        self.eye = self.eye * Matrix4::from_translation(Vector3{x: 0.0, y: 0.0, z: -distance});
    }

    pub fn control_with_keyboard(
        &mut self,
        move_speed: f32,
        turn_speed: f32,
        keyboard: KeyboardState
    ) {
        if keyboard.is_scancode_pressed(Scancode::Up) {
            self.move_forward(move_speed);
        }
        if keyboard.is_scancode_pressed(Scancode::Down) {
            self.move_forward(-move_speed);
        }
        if keyboard.is_scancode_pressed(Scancode::Left) {
            self.eye = self.eye * Matrix4::from_angle_y(Rad(turn_speed));
        }
        if keyboard.is_scancode_pressed(Scancode::Right) {
            self.eye = self.eye * Matrix4::from_angle_y(Rad(-turn_speed));
        }
    }

}
