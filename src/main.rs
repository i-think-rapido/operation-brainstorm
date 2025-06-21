mod index;
mod color;
mod voxel_colors;

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

mod color_pipeline {

    use crate::voxel_colors::VoxelColors;
    use std::sync::Arc;

    pub struct ColorPipeline<'a>(Vec<Arc<dyn Fn(VoxelColors) -> anyhow::Result<VoxelColors> + 'a>>);
    impl<'a> ColorPipeline<'a> {
        pub fn new() -> anyhow::Result<Self> {
            let mut out = ColorPipeline(vec![]);

            out.add_step(|mut vc| {

                let mut iter = vc.iter()?;
                while let Some(index) = iter.next() {
                    if let Some(value) = vc.get_mut(index) {
                        if value.grey() < 51 {
                            value.inactivate();
                        }
                    }
                }
                Ok(vc)
            });

            Ok(out)
        }

        pub fn add_step<T>(&mut self, fun: T) 
            where   T: Fn(VoxelColors) -> anyhow::Result<VoxelColors> + 'a
        {
            self.0.push(Arc::new(fun));
        }
        pub fn process(&self, colors: VoxelColors) -> anyhow::Result<VoxelColors> {
            let mut out = colors;
            for fun in self.0.iter() {
                out = (*fun)(out)?;
            }
            Ok(out)
        }
    }
}

mod lifecycle {

    use tracing::info;
    use crate::color_pipeline::ColorPipeline;
    use crate::constants::*;
    use crate::index::Index;
    use crate::voxel_colors::VoxelColors;
    use crate::camera::*;
    use raylib::prelude::*;

    pub struct State<'a> {
        pub data: String,
        pub rl: raylib::RaylibHandle,
        pub thread: raylib::RaylibThread,
        pub camera: raylib::camera::Camera3D,
        pub camera_position: CameraPosition,

        pub colors: VoxelColors,
        pub color_pipeline: ColorPipeline<'a>,
    }
    impl<'a> State<'a> {
        pub fn update_camera(&mut self) {
            self.camera.position = crate::camera::COORDINATES
                [self.camera_position.0]
                [self.camera_position.1]
                ;
        }
    }

    pub fn prepare_state<'a>(colors: VoxelColors) -> anyhow::Result<State<'a>> {
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
                Vector3::new(0.0, 0.0, {
                    let mut cam_correction = 0.0;
                    if let Index::Dimensions(_, _, z) = colors.dimensions() {
                        cam_correction = -CUBE_SIZE * z as f32; // 2.0;
                    }
                    cam_correction
                }),
                Vector3::new(0.0, 1.0, 0.0),
                45.0
            ),
            camera_position: 
                CameraPosition::default()
                    .down_by(3)
                    .right_by(3)
                    ,

            colors,
            color_pipeline: ColorPipeline::new()?
        })
    }

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

    pub fn shutdown(state: &mut State) {
        info!("Shutting down with state: {}", state.data);
        // Simulate cleanup
        state.data.clear();
        info!("State cleared.");
    }


    fn correct(position: f32, max: f32) -> f32 {
        position - max / 2.0
    }

}

mod camera {
    use raylib::ffi::DEG2RAD;
    use raylib::math::Matrix;
    use raylib::prelude::{Vector3, KeyboardKey};

    use crate::lifecycle::State;

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
}

