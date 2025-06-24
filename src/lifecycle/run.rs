use raylib::{color::Color, math::Vector3, prelude::{RaylibDraw, RaylibDraw3D, RaylibMode3DExt}};
use tracing::info;

use crate::constants::CUBE_SIZE;
use crate::graphics::state::State;

pub fn run(state: &mut State) -> anyhow::Result<()> {
    info!("Running with state: {}", state.data);
    // Simulate some processing

    state.rl.update_camera(&mut state.camera, raylib::consts::CameraMode::CAMERA_CUSTOM);

    while !state.rl.window_should_close() {

        state.camera_position = state.camera_position.update(state);
        state.update_camera();

        let mut d = state.rl.begin_drawing(&state.thread);
        
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);


        let colors = state.color_pipeline.process(&state.voxel_set);
        d.draw_mode3D(state.camera, |mut rl, _| {
            for voxel in &colors {
            }
        });
    }

    Ok(())
}

fn correct(position: f32, max: f32) -> f32 {
    position - max / 2.0
}
