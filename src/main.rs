mod voxel_color;

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::voxel_color::VoxelColors;

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
    let mut state = lifecycle::prepare_state(VoxelColors::new_from_singe_picture(&"./data/brain.png", 6, 3)?);
    // let mut state = prepare_state(VoxelColors::new_example());

    info!("Operation Branstorm is running...");
    lifecycle::run(&mut state);

    info!("Operation Branstorm is shutting down...");
    lifecycle::shutdown(&mut state);

    info!("Operation Branstorm is completed successfully!");

    Ok(())
}

mod color_pipeline {
    use crate::voxel_color::{Index, VoxelColors};
    use std::sync::Arc;

    pub struct ColorPipeline<'a>(Vec<Arc<dyn Fn(VoxelColors) -> VoxelColors + 'a>>);
    impl<'a> Default for ColorPipeline<'a> {
        fn default() -> Self {
            let mut out = ColorPipeline(vec![]);

            out.add_step(|mut vc| {
                for idx in 0..vc.capacity() {
                    if let Some(value) = vc.get(&Index::from_usize(idx, &vc.dimensions()).expect("")) {
                        if value.alpha() < 51 {
                            value.to_none();
                        }
                    }
                }
                vc
            });

            out
        }
    }
    impl<'a> ColorPipeline<'a> {
        pub fn add_step<T>(&mut self, fun: T) 
            where   T: Fn(VoxelColors) -> VoxelColors + 'a
        {
            self.0.push(Arc::new(fun));
        }
        pub fn process(&self, colors: VoxelColors) -> VoxelColors{
            let mut out = colors;
            for fun in self.0.iter() {
                out = (*fun)(out);
            }
            out
        }
    }
}

mod lifecycle {

    use tracing::info;
    use crate::color_pipeline::ColorPipeline;
    use crate::constants::*;
    use crate::voxel_color::VoxelColors;
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

    pub fn prepare_state<'a>(colors: VoxelColors) -> State<'a> {
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
            camera_position: 
                CameraPosition::default()
                    .down_by(3)
                    .right_by(3)
                    ,

            colors,
            color_pipeline: ColorPipeline::default()
        }
    }

    pub fn run(state: &mut State) {
        info!("Running with state: {}", state.data);
        // Simulate some processing

        state.rl.update_camera(&mut state.camera, raylib::consts::CameraMode::CAMERA_CUSTOM);

        while !state.rl.window_should_close() {

            state.camera_position = state.camera_position.update(state);
            state.update_camera();

            let mut d = state.rl.begin_drawing(&state.thread);
            
            d.clear_background(Color::WHITE);
            d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

            d.draw_mode3D(state.camera, |mut rl, cam| {

                state.color_pipeline
                    .process(state.colors.clone())
                    .process(|idx, mut color| 
                {
                    if let Some((x,y,z)) = state.colors.dimensions(idx) {
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

            for z in 0..CLOCKWISE_STEPS {
                let mut inner = vec![];

                for height in 0..HEIGHT_STEPS {
                    inner.push(
                        (- Vector3::forward() * 10.0).transform_with (   
                            Matrix::rotate_z(z as f32 / 2.0 + z      as f32 * deg2rad(180.0) / CLOCKWISE_STEPS as f32)
                          * Matrix::rotate_x(height as f32 * deg2rad(180.0) / HEIGHT_STEPS as f32)
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