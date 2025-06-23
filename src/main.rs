mod index;
mod color;
mod voxel_colors;
mod color_pipeline;
mod camera;
mod state;
mod lifecycle;

use raylib::color::Color;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{color::RGBA, voxel_colors::VoxelColors};

mod constants {
    pub const CUBE_SIZE: f32 = 0.05;
    pub const FRAME_RATE: u32 = 60;
}

fn main() -> anyhow::Result<()> {

    use tracing::info;

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Operation Branstorm is starting...");
    let mut state = lifecycle::prepare_state(VoxelColors::from_single_picture(&"./data/brain.png", 6, 3)?)?;
    // let mut state = prepare_state(VoxelColors::new_example());

    let pl = &mut state.color_pipeline;

    pl.add_step(|mut vc| {

            let mut iter = vc.iter()?;
            while let Some(index) = iter.next() {
                if let Some(rgba) = vc.get_mut(index) {
                    if rgba.is_active() { rgba.set_a(160); }
                }
            }

            Ok(vc)
    });

    let mut colorize = |color: raylib::color::Color, min: u8, max: u8| {
        pl.add_step(move |mut vc| {
            let mut iter = vc.iter()?;
            while let Some(index) = iter.next() {
                if let Some(rgba) = vc.get_mut(index) {
                    if rgba.grey() > max || rgba.grey() < min { continue; }
                    *rgba = RGBA::from(color);
                }
            };

            Ok(vc)
        });
    };

    colorize(Color::BLUE.clone(), 95, 215);

    info!("Operation Branstorm is running...");
    lifecycle::run(&mut state)?;

    info!("Operation Branstorm is shutting down...");
    lifecycle::shutdown(&mut state);

    info!("Operation Branstorm is completed successfully!");

    Ok(())
}


