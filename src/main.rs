mod voxel_color;


use raylib::prelude::*;
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::voxel_color::VoxelColors;

const CUBE_SIZE: f32 = 0.05;
const FRAME_RATE: u32 = 60;

fn main() -> anyhow::Result<()> {

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Operation Branstorm is starting...");
    let mut state = prepare_state(VoxelColors::new_from_singe_picture(&"./data/brain.png", 6, 3)?);
    // let mut state = prepare_state(VoxelColors::new_example());

    info!("Operation Branstorm is running...");
    run(&mut state);

    info!("Operation Branstorm is shutting down...");
    shutdown(&mut state);

    info!("Operation Branstorm is completed successfully!");

    Ok(())
}

pub struct State {
    pub data: String,
    pub rl: raylib::RaylibHandle,
    pub thread: raylib::RaylibThread,
    pub camera: raylib::camera::Camera3D,

    pub colors: VoxelColors,
}

pub fn prepare_state(colors: VoxelColors) -> State {
    info!("Preparing state...");

    let (mut rl, thread) = raylib::init()
        .fullscreen()
        .size(2560, 1600)
        .resizable()
        .title("Hello, World")
        .build()
        ;

        
    rl.set_target_fps(FRAME_RATE);
    rl.disable_cursor();

    State {
        data: String::from("Initial data"),
        rl,
        thread,
        camera: raylib::camera::Camera3D::perspective(
            Vector3::new(10.0, 10.0, 10.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            45.0
        ),

        colors,
    }
}

pub fn run(state: &mut State) {
    info!("Running with state: {}", state.data);
    // Simulate some processing

    while !state.rl.window_should_close() {
        state.rl.update_camera(&mut state.camera, raylib::consts::CameraMode::CAMERA_FREE);

        let mut d = state.rl.begin_drawing(&state.thread);
         
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

        d.draw_mode3D(state.camera, |mut rl, cam| {

            state.colors.process(|idx, mut color| {
                if let Some((x,y,z)) = state.colors.dimensions(idx) {
                    if color.r < 51 { return }
                    if color.r > 150 { return }
                    //color.a = 100;
                    rl.draw_cube(
                        Vector3::new(
                             correct(CUBE_SIZE * x as f32,  CUBE_SIZE * state.colors.dim_x as f32), 
                            -correct(CUBE_SIZE * y as f32,  CUBE_SIZE * state.colors.dim_y as f32),
                            -correct(CUBE_SIZE * z as f32, -CUBE_SIZE * state.colors.dim_z as f32),
                        ), 
                        CUBE_SIZE, 
                        CUBE_SIZE, 
                        CUBE_SIZE, 
                        color,
                    );            
                }
            });

            // rl.draw_cube_wires(Vector3::new(0.0, 1.0, 0.0), 2.0, 2.0, 2.0, Color::BLACK);
        });
    }
}

fn correct(position: f32, max: f32) -> f32 {
    position - max / 2.0
}

pub fn shutdown(state: &mut State) {
    info!("Shutting down with state: {}", state.data);
    // Simulate cleanup
    state.data.clear();
    info!("State cleared.");
}
