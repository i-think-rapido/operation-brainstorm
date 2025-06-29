
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Voxel(VoxelType);






/// core modules
crate::core_modules!();

/// trait modules
crate::trait_modules!(
    filter
    level
    dimensions
    color
);



