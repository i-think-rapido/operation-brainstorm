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
