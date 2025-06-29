use std::{borrow::Borrow, ops::{BitOrAssign, Deref, Shl}, str::FromStr};
use anyhow::Context;

use crate::structures::bitwise_builder::BitwiseBuilder;

impl<T> BitwiseBuilder<T>
where
    T: FromStr + std::fmt::Debug + BitOrAssign + Deref + Copy,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    Self: Default
{
    pub fn new(start_bit: u8, bits: u8) -> Self {
        let start_bit = 63.min(start_bit);
        let bits = 64.min(bits);

        assert!(64 >= start_bit as u16 + bits as u16 && bits > 0);

        let mask = if bits == 64 {
                u64::MAX
            }
            else {
                (2^(bits as u64)) - 1
            }
            .shl(start_bit)
            ;

        Self { start_bit, bits, mask: {
                format!("{}", mask)
                    .as_str()
                    .parse::<T>()
                    .map_err(anyhow::Error::from)
                    .context("can't create BitwiseBuilder")
            }
            .expect("can't create BitwiseBuilder")}
    }

    pub fn combine(builders: &[BitwiseBuilder<T>]) -> Self {
        if builders.is_empty() {
            return Self::default()
        }
        let mut mask = builders[0].mask;
        for builder in builders.iter().skip(1) {
            mask |= builder.mask;
        }
        Self { start_bit: 0, bits: (std::mem::size_of_val(&mask) * 8) as u8, mask }
    }
}
