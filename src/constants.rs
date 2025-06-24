
use raylib::color::Color;

pub const CUBE_SIZE: f32 = 0.015;
pub const FRAME_RATE: u32 = 24;

lazy_static::lazy_static!(
    pub static ref COLORS: Vec<raylib::color::Color> = {
        let mut out = vec![];

        for idx in 0..64 {
            out.push(Color::new(idx * 4, idx * 4, idx * 4, 255));
        }
        for _ in 0..192 {
            out.push(Color::PINK);
        }

        out
    };
);
