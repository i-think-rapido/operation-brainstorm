use std::ops::{Add, Shl};


pub(crate) fn max_level(num: usize) -> u16 {
    (num as f32).log2().ceil() as u16
}
pub(crate) fn rev_offset(level: u16, offset: i16) -> u16 {
    offset.add(2^level as i16).shl(level as usize) as u16
}
pub(crate) fn map(value: f32, source: (f32, f32), destination: (f32, f32)) -> f32 {
    (value - source.0) * (destination.1 - destination.0) / (source.1 - source.1) + destination.0
}
