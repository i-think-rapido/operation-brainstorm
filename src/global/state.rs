use raylib::math::Vector3;
use tracing::info;

use crate::{grpahics::camera::CameraPosition, color_pipeline::ColorPipeline};
//use crate::{constants::{CUBE_SIZE, FRAME_RATE}, data_structures::VoxelSet};

pub struct State<'a> {
    pub data: String,
    pub rl: raylib::RaylibHandle,
    pub thread: raylib::RaylibThread,
    pub camera: raylib::camera::Camera3D,
    pub camera_position: CameraPosition,

    pub voxel_set: VoxelSet,
    pub color_pipeline: ColorPipeline<'a>,
}
impl<'a> State<'a> {
    pub fn update_camera(&mut self) {
        self.camera.position = super::camera::COORDINATES
            [self.camera_position.0]
            [self.camera_position.1]
            ;
    }
}

pub fn prepare_state<'a>(voxel_set: VoxelSet) -> anyhow::Result<State<'a>> {
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

    Ok(State {
        data: String::from("Initial data"),
        rl,
        thread,
        camera: raylib::camera::Camera3D::perspective(
            Vector3::new(10.0, 10.0, 10.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            45.0
        ),
        camera_position: 
            CameraPosition::default()
                .down_by(3)
                .right_by(3)
                ,

        voxel_set,
        color_pipeline: ColorPipeline::default()
    })
}

