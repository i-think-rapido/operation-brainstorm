use anyhow::Context;
use parking_lot::RwLock;

use std::sync::Arc;

use crate::data_structures::VoxelSet;

#[derive(Default, Clone)]
pub struct ColorPipeline<'a>(Arc<RwLock<Vec<Arc<dyn Fn(VoxelSet) -> VoxelSet + 'a>>>>);
impl<'a> ColorPipeline<'a> {
    pub fn add_step<T>(&self, fun: T) 
        where   T: Fn(VoxelSet) -> VoxelSet + 'a
    {
        self.0.write().push(Arc::new(fun));
    }
    pub fn process(&self, colors: &VoxelSet) -> VoxelSet {
        let mut colors = colors.clone();
        for fun in self.0.read().clone() {
            colors = (*fun)(colors.clone());
        }
        colors
    }
}
