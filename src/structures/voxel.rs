
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Voxel(VoxelType);






/// core modules
crate::core_modules!();

/// trait modules
pub(crate) mod filter;
pub(crate) mod level;
pub(crate) mod dimensions;
pub(crate) mod color;

/// this imports
pub use filter::*;
pub use level::*;
pub use dimensions::*;
pub use color::*;



