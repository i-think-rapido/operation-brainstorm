use crate::structures::{bitwise::Bitwise, bitwise_builder::BitwiseBuilder};


impl<'t, 'b, T> Bitwise<'t, 'b, T> {
    pub fn new(val: &'t T, builder: &'b BitwiseBuilder<T>) -> Self {
        Self(val, builder)
    }
}
