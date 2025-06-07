
mod voxel_color;

pub use voxel_color::*;

#[no_mangle] 
#[allow(improper_ctypes_definitions)]
pub extern "C" fn voxel_color(x: usize, y: usize, z: usize) -> VoxelColors {
    VoxelColors::new(x, y, z)
}
#[no_mangle] 
pub extern "C" fn voxel_color_capacity(voxel: &VoxelColors) -> usize {
    voxel.capacity()
}
#[no_mangle] 
pub extern "C" fn voxel_color_r(voxel: &VoxelColors, index: usize) -> u8 {
    voxel.get_color(index, &RGBA::R).unwrap_or(0) as u8
}
#[no_mangle] 
pub extern "C" fn voxel_color_g(voxel: &VoxelColors, index: usize) -> u8 {
    voxel.get_color(index, &RGBA::G).unwrap_or(0) as u8
}
#[no_mangle] 
pub extern "C" fn voxel_color_b(voxel: &VoxelColors, index: usize) -> u8 {
    voxel.get_color(index, &RGBA::B).unwrap_or(0) as u8
}
#[no_mangle] 
pub extern "C" fn voxel_color_a(voxel: &VoxelColors, index: usize) -> u8 {
    voxel.get_color(index, &RGBA::A).unwrap_or(0) as u8
}
#[no_mangle] 
pub extern "C" fn set_voxel_color_r(voxel: &mut VoxelColors, index: usize, value: u8) {
    voxel.set_color(index, &RGBA::R, value);
}
#[no_mangle] 
pub extern "C" fn set_voxel_color_g(voxel: &mut VoxelColors, index: usize, value: u8) {
    voxel.set_color(index, &RGBA::G, value);
}
#[no_mangle] 
pub extern "C" fn set_voxel_color_b(voxel: &mut VoxelColors, index: usize, value: u8) {
    voxel.set_color(index, &RGBA::B, value);
}
#[no_mangle] 
pub extern "C" fn set_voxel_color_a(voxel: &mut VoxelColors, index: usize, value: u8) {
    voxel.set_color(index, &RGBA::A, value);
}




