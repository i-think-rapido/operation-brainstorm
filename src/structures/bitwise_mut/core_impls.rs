use std::ops::{Deref, DerefMut};

use crate::structures::bitwise_mut::BitwiseMut;

impl<'a, 'b, T>  Deref for BitwiseMut<'a, 'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, 'b, T> DerefMut for BitwiseMut<'a, 'b, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}
