#![no_std]

mod zig_alloc;
mod voxel_color;

pub use voxel_color::*;

#[no_mangle] 
#[allow(improper_ctypes_definitions)]
pub extern "C" fn voxel_color(x: i32, y: i32, z: i32) -> VoxelColors {
    VoxelColors::new(x as usize, y as usize, z as usize)
}
#[no_mangle] 
pub extern "C" fn voxel_color_capacity(voxel: &VoxelColors) -> i32 {
    voxel.capacity() as i32
}
#[no_mangle]
pub extern "C" fn voxel_color_dim_x(voxel: &VoxelColors) -> i32 {
    voxel.dim_x as i32
}
#[no_mangle]
pub extern "C" fn voxel_color_dim_y(voxel: &VoxelColors) -> i32 {
    voxel.dim_y as i32
}
#[no_mangle]
pub extern "C" fn voxel_color_dim_z(voxel: &VoxelColors) -> i32 {
    voxel.dim_z as i32
}
#[no_mangle] 
pub extern "C" fn voxel_color_r(voxel: &VoxelColors, index: i32) -> u8 {
    voxel.get_color(index as usize, &RGBA::R).unwrap_or(0) as u8
}
#[no_mangle] 
pub extern "C" fn voxel_color_g(voxel: &VoxelColors, index: i32) -> u8 {
    voxel.get_color(index as usize, &RGBA::G).unwrap_or(0) as u8
}
#[no_mangle] 
pub extern "C" fn voxel_color_b(voxel: &VoxelColors, index: i32) -> u8 {
    voxel.get_color(index as usize, &RGBA::B).unwrap_or(0) as u8
}
#[no_mangle] 
pub extern "C" fn voxel_color_a(voxel: &VoxelColors, index: i32) -> u8 {
    voxel.get_color(index as usize, &RGBA::A).unwrap_or(0) as u8
}
#[no_mangle] 
pub extern "C" fn set_voxel_color_r(voxel: &mut VoxelColors, index: i32, value: u8) {
    voxel.set_color(index as usize, &RGBA::R, value);
}
#[no_mangle] 
pub extern "C" fn set_voxel_color_g(voxel: &mut VoxelColors, index: i32, value: u8) {
    voxel.set_color(index as usize, &RGBA::G, value);
}
#[no_mangle] 
pub extern "C" fn set_voxel_color_b(voxel: &mut VoxelColors, index: i32, value: u8) {
    voxel.set_color(index as usize, &RGBA::B, value);
}
#[no_mangle] 
pub extern "C" fn set_voxel_color_a(voxel: &mut VoxelColors, index: i32, value: u8) {
    voxel.set_color(index as usize, &RGBA::A, value);
}
#[no_mangle]
pub extern "C" fn voxel_color_idx(voxel: &VoxelColors, x: i32, y: i32, z: i32) -> i32 {
    voxel.idx(x as usize, y as usize, z as usize).unwrap_or_default() as i32
}




