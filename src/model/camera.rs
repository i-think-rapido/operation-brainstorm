use raylib::ffi::DEG2RAD;
use raylib::math::Matrix;
use raylib::prelude::{Vector3, KeyboardKey};

use super::state::State;

const CLOCKWISE_STEPS: usize = 8;
const HEIGHT_STEPS: usize = 8;

fn deg2rad(value: f32) -> f32 {
    (value as f64 * DEG2RAD) as f32
}

lazy_static::lazy_static!(
    pub(crate) static ref COORDINATES: Vec<Vec<Vector3>> = {
        let mut out = vec![];

        let z_step = deg2rad(360.0 / CLOCKWISE_STEPS as f32);
        let h_step = deg2rad(300.0 / HEIGHT_STEPS as f32);

        let z = deg2rad(0.0);
        let h = deg2rad(-150.0);

        for i in 0..CLOCKWISE_STEPS {
            let mut inner = vec![];

            for j in 0..HEIGHT_STEPS {
                inner.push(
                    (- Vector3::forward() * 10.0).transform_with (   
                        Matrix::rotate_y(z + z_step * i as f32)
                        * 
                        Matrix::rotate_x(h + h_step * j as f32)
                    )
                );
            }

            out.push(inner);
        }

        out
    };
);

#[derive(Default, Clone, Copy)]
pub struct CameraPosition(pub usize, pub usize);
impl CameraPosition {
    pub fn left_by(&self, amount: usize) -> Self {
        let CameraPosition(z, height) = self;

        let z = (z + CLOCKWISE_STEPS - amount % CLOCKWISE_STEPS) % CLOCKWISE_STEPS;

        CameraPosition(z, *height)
    }
    pub fn right_by(&self, amount: usize) -> Self {
        let CameraPosition(z, height) = self;

        let z = (z + amount) % CLOCKWISE_STEPS;

        CameraPosition(z, *height)
    }
    pub fn up_by(&self, amount: usize) -> Self {
        let CameraPosition(z, height) = self;

        let height = (height + HEIGHT_STEPS - amount % HEIGHT_STEPS) % HEIGHT_STEPS;

        CameraPosition(*z, height)
    }
    pub fn down_by(&self, amount: usize) -> Self {
        let CameraPosition(z, height) = self;

        let height = (height + amount) % HEIGHT_STEPS;

        CameraPosition(*z, height)
    }
    pub fn left(&self) -> Self {
        self.left_by(1)
    }
    pub fn right(&self) -> Self {
        self.right_by(1)
    }
    pub fn up(&self) -> Self {
        self.up_by(1)
    }
    pub fn down(&self) -> Self {
        self.down_by(1)
    }

    pub fn update(&self, state: &State) -> CameraPosition {
        let mut out = self.clone();
        if state.rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
            out = out.left();                
        }
        if state.rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            out = out.right();                
        }
        if state.rl.is_key_pressed(KeyboardKey::KEY_UP) {
            out = out.up();                
        }
        if state.rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
            out = out.down();                
        }
        out
    }
}
