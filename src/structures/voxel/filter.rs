



use crate::data_structures::voxel::{Idx, OffsetType, Voxel, VoxelType, DIMENSION_MASKS, LEVEL_BITS, LEVEL_BIT_START, LEVEL_MASK, MAX_LEVEL_BITS, XYZ_START_BITS, X_INDEX, Y_INDEX, Z_INDEX};
use crate::{get, set, mask};
use std::ops::{Shl, Shr};





/// trait
pub trait Filter {
    fn index(&self) -> VoxelType;
    fn level(&self) -> VoxelType;
    fn depth(&self) -> VoxelType;
    fn values(&self) -> (VoxelType, VoxelType, VoxelType);
    fn offset(&self) -> (OffsetType, OffsetType, OffsetType);
}








impl Filter for Voxel {
    fn index(&self) -> VoxelType {
        let lvl = self.level() as Idx;
        (**self & *LEVEL_MASK) 
            | (**self & DIMENSION_MASKS[lvl][X_INDEX]) 
            | (**self & DIMENSION_MASKS[lvl][Y_INDEX]) 
            | (**self & DIMENSION_MASKS[lvl][Z_INDEX])
    }
    fn level(&self) -> VoxelType {
        get!(**self,    *LEVEL_MASK,         LEVEL_BIT_START)
    }
    #[inline]
    fn depth(&self) -> VoxelType {
        MAX_LEVEL_BITS - self.level()
    }
    
    fn values(&self) -> (VoxelType, VoxelType, VoxelType) {
        let depth = self.depth();
        let lvl = self.level() as Idx;
        (
            get!(**self,    DIMENSION_MASKS[lvl][X_INDEX],         depth),
            get!(**self,    DIMENSION_MASKS[lvl][Y_INDEX],         depth),
            get!(**self,    DIMENSION_MASKS[lvl][Z_INDEX],         depth),
        )
    }

    fn offset(&self) -> (OffsetType, OffsetType, OffsetType) {

        let fun = |dim| {
            let mut out = 0.0_f32;

            let lvl = self.level();
            for l in 0..lvl {
                out += if get!(**self, mask!(XYZ_START_BITS[dim] + LEVEL_BITS - lvl - 1, 1), 1) == 0u64 {
                    -0.5_f32.powf(l as f32)
                }
                else {
                    0.5_f32.powf(l as f32)
                }
            }

            out
        };

        (fun(X_INDEX), fun(Y_INDEX), fun(Z_INDEX))
    }
}



