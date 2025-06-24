use std::ops::{Deref, DerefMut};

use crate::data_structures::voxel::{Filter, Voxel, VoxelType};


/// deref
impl Deref for Voxel {
    type Target = VoxelType;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Voxel {
    fn deref_mut(&mut self) -> &mut VoxelType {
        &mut self.0
    }
}

/// ord
impl PartialOrd for Voxel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let fun: Box<dyn Fn(&Voxel) -> VoxelType> = Box::new(|voxel| {
            voxel.index()
        });
        fun(self).partial_cmp(&fun(other))
    }
}
impl Ord for Voxel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let fun: Box<dyn Fn(&Voxel) -> VoxelType> = Box::new(|voxel| {
            voxel.index()
        });
        fun(self).cmp(&fun(other))
    }
}

/// hash
impl std::hash::Hash for Voxel 
where   Self: Filter
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index().hash(state);
    }
} 
