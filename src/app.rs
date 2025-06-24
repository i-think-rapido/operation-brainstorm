
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

// use crate::data_structures::Voxel;


pub struct App;

impl App {

    pub fn run() -> anyhow::Result<()> {

        use tracing::info;

        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(tracing_subscriber::EnvFilter::from_default_env())
            .init();

        info!("Operation Branstorm is starting...");
        // let mut state = lifecycle::prepare_state(Voxel::from_multiple_pictures(&"./data/CT*.png")?)?;
        // let mut state = lifecycle::prepare_state(Voxel::from_single_picture(&"./data/brain.png", 6, 3)?)?;
        // let mut state = prepare_state(VoxelColors::new_example());

        //let pl = &state.color_pipeline;

        // let portion = |pl: &ColorPipeline<'_>, num: usize| {
        //     pl.clone().add_step(move |mut vs| {
        //         if let Index::Dimensions(x, y, z) = index.to_dimensions(vc.dimensions())? {
        //             if z > num || z < num  {
        //                 let rgba = vc.get_mut(index).context("can't retrieve rgba")?;
        //                 rgba.inactivate();
        //             }
        //         }

        //         vs
        //     });
        // };

        // let inactivate = |pl: &ColorPipeline<'_>, min: u8, max: u8| {
        //     pl.clone().add_step(move |index, mut vc| {
        
        //         let rgba = vc.get_mut(index).context("can't retrieve rgba")?;
                
        //         if rgba.grey() > max || rgba.grey() < min { return Ok(vc) }
                
        //         rgba.inactivate();

        //         Ok(vc)
        //     });
        // };
        // let set_alpha = |pl: &ColorPipeline<'_>, alpha_value: u8| {
        //     pl.clone().add_step(move |index, mut vc| {

        //         let rgba = vc.get_mut(index).context("can't retrieve rgba")?;
        //         if rgba.is_active() { rgba.set_a(alpha_value); }

        //         Ok(vc)
        //     });
        // };
        // let colorize = |pl: &ColorPipeline<'_>, color: RGBA, min: u8, max: u8| {
        //     pl.clone().add_step(move |index, mut vc| {
        
        //         let rgba = vc.get_mut(index).context("can't retrieve rgba")?;
                
        //         if rgba.grey() > max || rgba.grey() < min { return Ok(vc) }
                
        //         *rgba = RGBA::from(color);

        //         Ok(vc)
        //     });
        // };

        //portion(pl, 20);

    //    inactivate(pl, 0, 30);

        //let mut transparent =  RGBA::from(Color::PINK);
        //transparent.set_a(0);
        //colorize(transparent, 125, 255);
        //colorize(transparent, 0, 90);

    //    colorize(RGBA::from(Color::BLUE), 95, 215);


        info!("Operation Branstorm is running...");
    //    lifecycle::run(&mut state)?;

        info!("Operation Branstorm is shutting down...");
    //    lifecycle::shutdown(&mut state);

        info!("Operation Branstorm is completed successfully!");

        Ok(())
    }



}