
use std::ops::Shl;
use crate::mask;

/// types

pub(crate) type VoxelType = u64;



pub(crate) type Alpha = u64;
pub(crate) type ColorIndex = u64;

pub(crate) type Level = u64;
pub(crate) type StartBit = u64;
pub(crate) type Bits = u64;
pub(crate) type Mask = u64;
pub(crate) type Idx = usize;
pub(crate) type Dimension = u64;
pub(crate) type OffsetType = f32;




/// constants

pub(crate) const ALPHA_START_BIT: StartBit = 8;
pub(crate) const MAX_LEVEL_BITS: Level = 15;
pub(crate) const LEVELS_START_BIT: StartBit = 14;
pub(crate) const LEVEL_BIT_START: StartBit = 59;
pub(crate) const LEVEL_BITS: Bits = 4; 

pub(crate) const X_INDEX: Idx = 0;
pub(crate) const Y_INDEX: Idx = 1;
pub(crate) const Z_INDEX: Idx = 2;
pub(crate) const DIMENSIONS: Idx = 3;

pub(crate) const XYZ_START_BITS: [StartBit; DIMENSIONS] = [
    LEVELS_START_BIT + MAX_LEVEL_BITS * X_INDEX as StartBit,
    LEVELS_START_BIT + MAX_LEVEL_BITS * Y_INDEX as StartBit,
    LEVELS_START_BIT + MAX_LEVEL_BITS * Z_INDEX as StartBit,
];



/// lazy statics
lazy_static::lazy_static!(

    pub(crate) static ref DIMENSION_MASKS: [[Mask; DIMENSIONS ]; MAX_LEVEL_BITS as Idx] = {
        let mut out = [ [0; DIMENSIONS]; MAX_LEVEL_BITS as Idx];

        for lvl in 0..=MAX_LEVEL_BITS {
            for (dim, _) in XYZ_START_BITS.iter().enumerate().take(DIMENSIONS) {
                let position = XYZ_START_BITS[dim] + lvl;
                let bits = MAX_LEVEL_BITS - lvl;
                out[lvl as Idx][dim] = mask!(position, bits);
            }
        }

        out
    };
    pub(crate) static ref LEVEL_MASK: Mask = mask!(LEVEL_BIT_START, LEVEL_BITS);

);
