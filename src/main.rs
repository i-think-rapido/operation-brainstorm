mod index;
mod color;
mod voxel_colors;
mod color_pipeline;
mod camera;
mod state;
mod lifecycle;

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::voxel_colors::VoxelColors;

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

    info!("Operation Branstorm is running...");
    lifecycle::run(&mut state)?;

    info!("Operation Branstorm is shutting down...");
    lifecycle::shutdown(&mut state);

    info!("Operation Branstorm is completed successfully!");

    Ok(())
}


