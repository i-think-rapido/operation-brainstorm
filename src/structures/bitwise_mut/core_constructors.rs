use crate::structures::{bitwise_builder::BitwiseBuilder, bitwise_mut::BitwiseMut};


impl<'t, 'b, T> BitwiseMut<'t, 'b, T> {
    pub fn new(val: &'t mut T, builder: &'b BitwiseBuilder<T>) -> Self {
        Self(val, builder)
    }
}
