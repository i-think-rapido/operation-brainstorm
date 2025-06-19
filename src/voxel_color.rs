use getset::Getters;
use raylib::{color::Color, texture::Image};



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
    
    colors: Vec<Color>,
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

    pub fn new_from_singe_picture(path: &dyn AsRef<str>, cols: usize, rows: usize) -> anyhow::Result<Self> {
        let mut img = Image::load_image(path.as_ref())?;
        let dim_x = img.width() as usize / cols;
        let dim_y = img.height() as usize / rows;
        let dim_z = cols * rows; // Single layer for 2D image

        let mut colors = Self::new(dim_x, dim_y, dim_z);

        for i in 0..img.width() as usize {
            for j in 0..img.height() as usize {
                let x = i % dim_x;
                let y = j % dim_y;
                let z = i / dim_x * j / dim_y;
                if let Some(index) = colors.idx(x, y, z) {
                    colors.colors[index] = img.get_color(i as i32, j as i32);
                }

            }
        }

        Ok(colors)
    }
    pub fn new_example() -> Self {
        let mut out = Self::new(5, 4, 3);

        let rand = |idx: i32| {
            match idx % 3 {
                0 => Color::RED,
                1 => Color::GREEN,
                2 => Color::BLUE,
                _ => unreachable!()
            }
        };

        for i in 0..out.capacity()
        {
            out.colors[i] = rand(i as i32);
        }

        out
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
    pub fn dimensions(&self, index: usize) -> Option<(usize, usize, usize)> {
        if index >= self.capacity() { return None }
        let z = index / (self.dim_x * self.dim_y);
        let y = (index % (self.dim_x * self.dim_y)) / self.dim_x;
        let x = index % self.dim_x;
        Some((x, y, z))
    }

    pub fn process(&self, mut func: impl FnMut(usize, Color)) {
        for (index, color) in self.colors.iter().enumerate() {
            func(index, color.clone());
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
