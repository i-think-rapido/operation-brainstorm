use anyhow::{bail, Context};
use raylib::color::Color;
use glob::glob;

use crate::data_structures::{voxel::{core_functions::{map, max_level, rev_offset}, Dimension, Level, Voxel}, voxel_set::VoxelSet};


impl Voxel {

    pub fn new((x, y, z): (Dimension, Dimension, Dimension), lvl: Level, ) {

    }

    pub fn from_single_picture(filename: &dyn AsRef<str>, cols: usize, rows: usize) -> anyhow::Result<VoxelSet> {

        let mut image = raylib::prelude::Image::load_image(filename.as_ref())?;

        let (dim_x, dim_y, dim_z) = (image.width as usize / cols, image.height as usize / rows, cols * rows);

        let max_level = max_level(dim_x.max(dim_y).max(dim_z));

        let ratio: usize = ((dim_y as f32 / dim_x as f32) * crate::constants::DIM_X as f32) as usize; 

        let mut out = VoxelSet::new();

        for x in 0..dim_x {
            for y in 0..dim_y {
                for z in 0..dim_z {
                   let x = x as f32;
                   let y = y as f32;
                   let z = z as f32;
                   let cols = cols as f32;
                   let dim_x = dim_x as f32;
                   let dim_y = dim_y as f32;
                   let dim_z = dim_z as f32;
                   let color: Color = image.get_color(
                           (map(x, (0.0, dim_x), (0.0, crate::constants::DIM_X as f32)) + (z % cols) * dim_x) as i32
                       , 
                           (map(y, (0.0, dim_y), (0.0, ratio as f32)) + (z / cols) * dim_y) as i32
                       
                   );
                   out.insert(Voxel::new(
                       (
                           rev_offset(max_level, map(x, (0.0, dim_x), (-dim_x / 2.0, dim_x / 2.0)) as i16), 
                           rev_offset(max_level, map(y, (0.0, dim_y), (-dim_y / 2.0, dim_y / 2.0)) as i16),
                           rev_offset(max_level, map(z, (0.0, dim_z), (-dim_z / 2.0, dim_z / 2.0)) as i16),
                       ), max_level, 2^6 - 1, (color.r / 4) as u16));
                }
            }
        }

        Ok(out)
    }

    pub fn from_multiple_pictures(glob_pattern: &dyn AsRef<str>) -> anyhow::Result<VoxelSet> {
        let mut vec = vec![];
        for entry in glob(glob_pattern.as_ref())? {
            match entry {
                Ok(path) => vec.push(Self::from_single_picture(&path.as_path().to_str().context("can't convert filename to string")?, 1, 1)?),
                Err(_) => bail!("file not found"),
            }
        }
        let mut out = VoxelSet::new();

        let dim_z = vec.len();
        if !vec.is_empty() {
            if let Some(voxel) = vec[0].iter().next() {
                for (z, set) in vec.iter().enumerate() {
                    set.iter().map(|voxel| {
                        *voxel
                    }).for_each(|voxel| { out.insert(voxel); });
                }
            }
        }

        Ok(out)
    }

}
