
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum IndexError {
    #[error("out of bounds")]
    OutOfBounds,
    #[error("no dimensions provided")]
    NoDimensionsProvided,
    #[error("wrong index variant")]
    WrongIndexVariant,
    #[error("can't increase index")]
    CantIncreaseIndex,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub enum Index {
    Dimensions(usize, usize, usize),
    Index(usize)
}

impl Index {
    // pub fn capacity(self) -> Result<usize, IndexError> {
    //     match self {
    //         Index::Dimensions(dim_x, dim_y, dim_z) => Ok(dim_x * dim_y * dim_z),
    //         _ => Err(IndexError::NoDimensionsProvided)
    //     }
    // }
    pub fn to_index(self, dimensions: Index) -> Result<Self, IndexError> {
        if let Index::Dimensions(dim_x, dim_y, dim_z) = dimensions {
            Ok(match self {
                index @ Index::Index(idx) =>
                    if idx >= dim_x * dim_y * dim_z {
                        return Err(IndexError::OutOfBounds)
                    }
                    else {
                        index
                    },
                Index::Dimensions(x, y, z) =>
                    if x >= dim_x || y >= dim_y || z >= dim_z {
                        return Err(IndexError::OutOfBounds)
                    }
                    else {
                        Index::Index(
                            x +
                            dim_x * y + 
                            dim_x * dim_y * z
                        )
                    }
            })
        }
        else {
            Err(IndexError::NoDimensionsProvided)
        }
    }

    pub fn to_dimensions(self, dimensions: Index) -> Result<Self, IndexError> {
        if let Index::Dimensions(dim_x, dim_y, dim_z) = dimensions {
            Ok(match self {
                Index::Index(idx) =>
                {
                    let x = idx % dim_x;
                    let y = idx / dim_x % dim_y;
                    let z = idx / (dim_x * dim_y) % dim_z;
                    if idx / (dim_x * dim_y * dim_z) > 1 {
                        return Err(IndexError::OutOfBounds)
                    }
                    Index::Dimensions(x, y, z)
                }
                coords @ Index::Dimensions(x, y, z) =>
                    if x >= dim_x || y >= dim_y || z >= dim_z {
                        return Err(IndexError::OutOfBounds)
                    }
                    else {
                        coords
                    }
            })
        }
        else {
            Err(IndexError::NoDimensionsProvided)
        }
    }

    // pub fn is_index(&self) -> bool {
    //     if let Index::Index(_) = self {
    //         true
    //     }
    //     else {
    //         false
    //     }
    // }
    pub fn is_dimensions(&self) -> bool {
        if let Index::Dimensions(_, _, _) = self {
            true
        }
        else {
            false
        }
    }

    pub fn inc(self, dimensions: Index) -> Result<Self, IndexError> {
        if !dimensions.is_dimensions() {
            return Err(IndexError::NoDimensionsProvided)
        }
        if let Index::Index(idx) = self.to_index(dimensions)? {
            let index = Index::Index(idx + 1);
            let _ = index.to_dimensions(dimensions).map_err(|_| IndexError::CantIncreaseIndex)?;
            Ok(index)
        }
        else {
            Err(IndexError::WrongIndexVariant)
        }
    }
}

pub struct IndexIterator {
    dimensions: Index,
    index: Index,
}
impl IndexIterator {
    pub fn new(dimensions: &Index) -> Result<Self, IndexError> {
        if !dimensions.is_dimensions() {
            return Err(IndexError::WrongIndexVariant)
        }
        Ok(Self {
            dimensions: *dimensions,
            index: Index::Index(0)
        })
    }
}
impl Iterator for IndexIterator {
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.index.to_index(self.dimensions).ok()?.inc(self.dimensions).ok()?;
        self.index = out;
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIMENSIONS: Index = Index::Dimensions(4, 3, 2);

    #[test]
    fn test_dimensions() -> anyhow::Result<()>{
        let idx = Index::Index(23);

        assert_eq!(idx.to_dimensions(DIMENSIONS), Ok(Index::Dimensions(3, 2, 1)));

        Ok(())
    }

    #[test]
    fn test_iterator() -> Result<(), IndexError> {
        let mut sum = 0;

        let mut iter = IndexIterator::new(&DIMENSIONS)?;
        while let Some(idx) = iter.next() {
            if let Index::Index(idx) = idx { sum += idx }
        }

        assert_eq!(300, sum);

        Ok(())
    }
}
