use crate::structures::{bitwise::Bitwise, bitwise_builder::BitwiseBuilder, bitwise_mut::BitwiseMut};


pub trait Handler<T> {
    fn handle<'a, 'b>(&'b self, val: &'a T) -> Bitwise<'a, 'b, T>;
    fn handle_mut<'a, 'b>(&'b self, val: &'a mut T) -> BitwiseMut<'a, 'b, T>;
}




impl <T> Handler<T> for BitwiseBuilder<T> {
    fn handle<'a, 'b>(&'b self, val: &'a T) -> Bitwise<'a, 'b, T> {
        Bitwise::new(val, self)
    }

    fn handle_mut<'a, 'b>(&'b self, val: &'a mut T) -> BitwiseMut<'a, 'b, T> {
        BitwiseMut::new(val, self)
    }
}
