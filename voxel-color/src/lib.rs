pub use voxel_memory::VoxelColors;

#[no_mangle] 
#[allow(improper_ctypes_definitions)]
pub extern "C" fn new_voxel_color(x: usize, y: usize, z: usize) -> VoxelColors {
    VoxelColors::new(x, y, z)
}
#[no_mangle] 
pub extern "C" fn voxel_color(voxel: &VoxelColors, x: usize, y: usize, z: usize, rgba: &RGBA) -> u8 {
    voxel.idx(x, y, z)
        .map(|idx| voxel.get_color(idx, rgba))
        .unwrap_or_default()
        .unwrap_or(255)
}
#[no_mangle] 
pub extern "C" fn set_voxel_color(voxel: &mut VoxelColors, x: usize, y: usize, z: usize, rgba: &RGBA, value: u8) {
    if let Some(idx) = voxel.idx(x, y, z) {
        voxel.set_color(idx, rgba, value);
    }
}
#[no_mangle] 
pub extern "C" fn voxel_color_size(voxel: &VoxelColors) -> usize {
    voxel.capacity()
}
#[no_mangle] 
pub extern "C" fn voxel_color_dim_x(voxel: &VoxelColors) -> usize {
    voxel.dim_x
}
#[no_mangle] 
pub extern "C" fn voxel_color_dim_y(voxel: &VoxelColors) -> usize {
    voxel.dim_y
}
#[no_mangle] 
pub extern "C" fn voxel_color_dim_z(voxel: &VoxelColors) -> usize {
    voxel.dim_z
}
#[no_mangle] 
pub extern "C" fn voxel_color_index(voxel: &VoxelColors, x: usize, y: usize, z: usize) -> usize {
    voxel.idx(x, y, z).unwrap_or_default()
}


#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum RGBA { R, G, B, A, }
impl From<&RGBA> for u8 {
    fn from(v: &RGBA) -> u8 {
        match v {
            RGBA::R => 0,
            RGBA::G => 1,
            RGBA::B => 2,
            RGBA::A => 3,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voxel() {
        let arr = [RGBA::R, RGBA::G, RGBA::B, RGBA::A];

        let mut voxel_color = VoxelColors::new(3, 2, 1);

        for ref rgba in arr {
            assert_eq!(voxel_color.get_color(0, rgba), Some(0));
            assert_eq!(voxel_color.get_color(1, rgba), Some(0));
            assert_eq!(voxel_color.get_color(2, rgba), Some(0));
            assert_eq!(voxel_color.get_color(3, rgba), Some(0));
            assert_eq!(voxel_color.get_color(4, rgba), Some(0));
            assert_eq!(voxel_color.get_color(5, rgba), Some(0));
            assert_eq!(voxel_color.get_color(6, rgba), None);
        }

        for ref rgba in arr {
            voxel_color.set_color(0, rgba, 13);
        }
        for ref rgba in arr {
            assert_eq!(voxel_color.get_color(0, rgba), Some(13));
        }

        assert_eq!(voxel_color.capacity(), 6);
    }
}

mod voxel_memory {
    use std::alloc::{alloc, dealloc, Layout};
    use getset::Getters;

    use crate::RGBA;

    #[derive(Getters, Debug)]
    #[repr(C)]
    pub struct VoxelColors {
        #[getset(get)]
        pub dim_x: usize,
        #[getset(get)]
        pub dim_y: usize,
        #[getset(get)]
        pub dim_z: usize,
        
        colors: Values,
    }

    impl VoxelColors {
        pub fn new(dim_x: usize, dim_y: usize, dim_z: usize) -> Self {
            VoxelColors {
                dim_x,
                dim_y,
                dim_z,

                colors: Values::new(dim_x as usize * dim_y * dim_z * 4), // 4 for RGBA
            }
        }

        #[inline(always)]
        pub fn capacity(&self) -> usize {
            self.dim_x * self.dim_y * self.dim_z
        }

        pub fn get_color(&self, index: usize, rgba: &RGBA) -> Option<u8> {
            self.colors.get(self::index(index, rgba))
        }

        pub fn set_color(&mut self, index: usize, rgba: &RGBA, value: u8) {
            self.colors.set(self::index(index, rgba), value);
        }

        pub fn idx(&self, x: usize, y: usize, z: usize) -> Option<usize> {
            if x >= self.dim_x || y >= self.dim_y || z >= self.dim_z { return None }
            Some(x + y * self.dim_x + z * self.dim_x * self.dim_y)
        }

    }

    #[inline(always)]
    fn index(index: usize, rgba: &RGBA) -> usize {
        index * 4 + u8::from(rgba) as usize
    }

    #[derive(Debug)]
    #[repr(C)]
    struct Values {
        ptr: *mut u8,
        capacity: usize,
    }

    impl Values {
        pub fn new(capacity: usize) -> Self {
            unsafe {
                let layout = Layout::array::<u8>(capacity).expect("Failed to create layout for Values");
                let ptr = alloc(layout);
                ptr.write_bytes(0, capacity); // Initialize memory to zero
                Values { ptr, capacity }
            }
        }

        pub fn get(&self, index: usize) -> Option<u8> {
            if index >= self.capacity {
                return None;
            }
            unsafe { Some(*self.ptr.add(index)) }
        }
        pub fn set(&mut self, index: usize, value: u8) {
            if index < self.capacity {
                unsafe {
                    *self.ptr.add(index) = value;
                }
          }
        }
    }

    impl Drop for Values {
        fn drop(&mut self) {
            unsafe {
                let layout = Layout::array::<u8>(self.capacity).expect("Failed to create layout for Values");
                dealloc(self.ptr, layout);
            }
        }
    }
}