
pub use super::color::*;
use super::index::{Index, IndexError, IndexIterator};
use parking_lot::Mutex;
use raylib::prelude::Image;

#[derive(Clone)]
pub struct VoxelColors {
    dimensions: Index,
    colors: Vec<RGBA>
}

impl VoxelColors {

    pub fn new(dimensions: Index) -> Result<Self, IndexError> {
        match dimensions {
            Index::Index(_) => Err(IndexError::NoDimensionsProvided),
            d @ Index::Dimensions(dim_x, dim_y, dim_z) => {
                if dim_x == 0 || dim_y == 0 || dim_z == 0 {
                    return Err(IndexError::NoDimensionsProvided)
                }
                Ok(Self {
                    dimensions: d,
                    colors: vec![],
                })
            }
        }
    }

    pub fn dimensions(&self) -> Index {
        self.dimensions
    }

    pub fn iter(&self) -> Result<IndexIterator, IndexError> {
        IndexIterator::new(&self.dimensions)
    }

    pub fn push(&mut self, color: RGBA) {
        self.colors.push(color)
    }

    pub fn get(&self, idx: Index) -> Option<RGBA> {
        if let Index::Index(idx) = idx.to_index(self.dimensions).ok()? {
            self.colors.get(idx).cloned()
        }
        else {
            None
        }
    }
    pub fn get_mut(&mut self, idx: Index) -> Option<&mut RGBA> {
        if let Index::Index(idx) = idx.to_index(self.dimensions).ok()? {
            self.colors.get_mut(idx)
        }
        else {
            None
        }
    }
    // pub fn set(&mut self, idx: Index, color: RGBA) {
    //     if let Some(c) = self.get_mut(idx) {
    //         *c = color;
    //     }
    // }

    pub fn process<F>(&self, fun: F) -> anyhow::Result<()>
        where F: FnMut(&Index, Option<RGBA>) -> anyhow::Result<()>
    {
        let mut iter = self.iter()?;
        let fun = Mutex::new(fun);
        while let Some(index) = iter.next() {
            (*fun.lock())(&index, self.get(index))?
        }
        Ok(())
    }
}

// data loading ...
impl VoxelColors {

    pub fn from_single_picture(filename: &dyn AsRef<str>, cols: usize, rows: usize) -> anyhow::Result<VoxelColors> {

        let mut image = Image::load_image(filename.as_ref())?;

        let (dim_x, dim_y, dim_z) = (image.width as usize / cols, image.height as usize / rows, cols * rows);

        let dimensions = Index::Dimensions(dim_x, dim_y, dim_z);
        let mut out = VoxelColors::new(dimensions)?;

        let mut iter = out.iter()?;
        while let Some(idx) = iter.next() {
            if let Index::Dimensions(x, y, z) = idx.to_dimensions(dimensions)? {
                out.push(image.get_color((
                        x + (z % cols) * dim_x
                    ) as i32, (
                        y + (z / cols) * dim_y
                    ) as i32
                ).into());
            }
        }

        Ok(out)
    }

}