use raylib::{color::Color, math::Vector3, prelude::{RaylibDraw, RaylibDraw3D, RaylibMode3DExt}};
use tracing::info;

use crate::constants::CUBE_SIZE;
use crate::model::{index::Index, state::State};

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


        let vc = state.color_pipeline.process(state.colors.clone())?;
        d.draw_mode3D(state.camera, |mut rl, _| {
            let _ = vc.process(|index, rgba| {
                if let (
                    Ok(Index::Dimensions(x, y, z)),
                    Index::Dimensions(dim_x, dim_y, dim_z),
                    Some(rgba)
                )
                = (
                    index.to_dimensions(vc.dimensions()),
                    vc.dimensions(),
                    rgba,
                ) 
                {
                    if rgba.is_active() {
                        rl.draw_cube(
                            Vector3::new(
                                    correct(CUBE_SIZE * x as f32,  CUBE_SIZE * dim_x as f32), 
                                -correct(CUBE_SIZE * y as f32,  CUBE_SIZE * dim_y as f32),
                                -correct(CUBE_SIZE * z as f32, -CUBE_SIZE * dim_z as f32),
                            ), 
                            CUBE_SIZE, 
                            CUBE_SIZE, 
                            CUBE_SIZE, 
                            raylib::prelude::Color::from(rgba),
                        );
                    }
                }
                Ok(())
            });
        });
    }

    Ok(())
}

fn correct(position: f32, max: f32) -> f32 {
    position - max / 2.0
}
