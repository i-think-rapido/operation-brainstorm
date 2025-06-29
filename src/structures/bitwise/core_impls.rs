use std::ops::{Deref, DerefMut};

use crate::structures::bitwise::Bitwise;

// Ensure Bitwise is a tuple struct and its field is public


impl<'a, 'b, T>  Deref for Bitwise<'a, 'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

