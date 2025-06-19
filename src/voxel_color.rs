use getset::Getters;



#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RGBA { R, G, B, A, }

#[derive(Getters, Debug)]
pub struct VoxelColors {
    #[getset(get)]
    pub dim_x: usize,
    #[getset(get)]
    pub dim_y: usize,
    #[getset(get)]
    pub dim_z: usize,
    
    colors: Vec<Value>,
}

impl VoxelColors {
    pub fn new(dim_x: usize, dim_y: usize, dim_z: usize) -> Self {
        let mut  vec = Vec::with_capacity(dim_x * dim_y * dim_z);
        unsafe {
            vec.set_len(dim_x * dim_y * dim_z);
        }        
        VoxelColors {
            dim_x,
            dim_y,
            dim_z,

            colors: vec, // 4 for RGBA
        }
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.dim_x * self.dim_y * self.dim_z
    }

    pub fn get_color(&self, index: usize, rgba: &RGBA) -> Option<u8> {
        self.colors.get(index).map(|v| match rgba {
            RGBA::R => v.r,
            RGBA::G => v.g,
            RGBA::B => v.b,
            RGBA::A => v.a,
        })
    }

    pub fn set_color(&mut self, index: usize, rgba: &RGBA, value: u8) {
        if let Some(color) = self.colors.get_mut(index) {
            match rgba {
                RGBA::R => color.r = value,
                RGBA::G => color.g = value,
                RGBA::B => color.b = value,
                RGBA::A => color.a = value,
            }
        }
    }

    pub fn idx(&self, x: usize, y: usize, z: usize) -> Option<usize> {
        if x >= self.dim_x || y >= self.dim_y || z >= self.dim_z { return None }
        Some(x + y * self.dim_x + z * self.dim_x * self.dim_y)
    }

}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct Value {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
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
