mod voxel_color;


use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use voxel_color::VoxelColors;


fn main() {

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Operation Branstorm is starting...");
    let mut state = prepare_state();

    info!("Operation Branstorm is running...");
    run(&mut state);

    info!("Operation Branstorm is shutting down...");
    shutdown(&mut state);

    info!("Operation Branstorm is completed successfully!");
}

pub struct State {
    pub data: String,
}

pub fn prepare_state() -> State {
    info!("Preparing state...");
    State {
        data: String::from("Initial data"),
    }
}

pub fn run(state: &mut State) {
    info!("Running with state: {}", state.data);
    // Simulate some processing
    state.data.push_str(" -> Processed");
}

pub fn shutdown(state: &mut State) {
    info!("Shutting down with state: {}", state.data);
    // Simulate cleanup
    state.data.clear();
    info!("State cleared.");
}