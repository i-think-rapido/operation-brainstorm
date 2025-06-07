use crate::zig_alloc::ZigAllocator;

#[global_allocator]
#[no_mangle]
static GLOBAL: ZigAllocator = ZigAllocator;

use core::panic::PanicInfo;

#[panic_handler]
fn handle_panic(_info: &PanicInfo) -> ! {
    loop {} // or use core::intrinsics::abort() if available
}

use getset::Getters;



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

#[derive(Getters, Debug)]
#[repr(C)]
pub struct VoxelColors {
    #[getset(get)]
    pub dim_x: usize,
    #[getset(get)]
    pub dim_y: usize,
    #[getset(get)]
    pub dim_z: usize,
    
    colors: memory::Values,
}

impl VoxelColors {
    pub fn new(dim_x: usize, dim_y: usize, dim_z: usize) -> Self {
        VoxelColors {
            dim_x,
            dim_y,
            dim_z,

            colors: memory::Values::new(dim_x * dim_y * dim_z * 4), // 4 for RGBA
        }
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.dim_x * self.dim_y * self.dim_z
    }

    pub fn get_color(&self, index: usize, rgba: &RGBA) -> Option<u8> {
        self.colors.get(memory::index(index, rgba))
    }

    pub fn set_color(&mut self, index: usize, rgba: &RGBA, value: u8) {
        self.colors.set(memory::index(index, rgba), value);
    }

    pub fn idx(&self, x: usize, y: usize, z: usize) -> Option<usize> {
        if x >= self.dim_x || y >= self.dim_y || z >= self.dim_z { return None }
        Some(x + y * self.dim_x + z * self.dim_x * self.dim_y)
    }

}

mod memory {

    use crate::zig_alloc::ZigAllocator;

    use super::RGBA;

    use core::alloc::{GlobalAlloc, Layout};

    #[inline(always)]
    pub fn index(index: usize, rgba: &RGBA) -> usize {
        index * 4 + u8::from(rgba) as usize
    }

    #[derive(Debug)]
    #[repr(C)]
    pub struct Values {
        ptr: *mut u8,
        capacity: usize,
    }

    impl Values {
        pub fn new(capacity: usize) -> Self {
            unsafe {
                let ptr = if let Ok(layout) = Layout::from_size_align(capacity, 8)
                {
                    let ptr = ZigAllocator.alloc(layout);
                    if ptr.is_null() {
                        core::ptr::null_mut()
                    }
                    else {
                        ptr
                    }
                } else {
                    core::ptr::null_mut()
                };
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
                if let Ok(layout) = Layout::from_size_align(self.capacity, 8) {
                    ZigAllocator.dealloc(self.ptr, layout);
                }
            }
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
