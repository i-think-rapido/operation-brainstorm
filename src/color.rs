
use std::{fmt::Debug, ops::Shl, u32};

use raylib::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(packed)]
pub struct ColorValues {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Copy)]
#[repr(packed)]
pub union RGBA {
    pub value: u32,
    pub color: ColorValues,
}

impl PartialEq for RGBA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.value == other.value }
    }
}
impl Eq for RGBA {}
impl Debug for RGBA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RGBA {{ a: {}, r: {}, g: {}, b: {} }}", self.a(), self.r(), self.g(), self.b())
    }
}

// raylib::color::RGBA
impl From<&Color> for RGBA {
    fn from(value: &Color) -> Self {
        Self { color: ColorValues { a: value.a, b: value.b, g: value.g, r: value.r } }
    }
}
impl From<Color> for RGBA {
    fn from(value: Color) -> Self {
        Self { color: ColorValues { a: value.a, b: value.b, g: value.g, r: value.r } }
    }
}
impl From<RGBA> for Color {
    fn from(value: RGBA) -> Self {
        unsafe { Self::new(value.color.r, value.color.g, value.color.b, value.color.a) }
    }
}

// u32
impl From<&RGBA> for u32 {
    fn from(value: &RGBA) -> Self {
        unsafe { value.value }
    }
}
impl From<RGBA> for u32 {
    fn from(value: RGBA) -> Self {
        unsafe { value.value }
    }
}
impl From<u32> for RGBA {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

// default
impl Default for RGBA {
    fn default() -> Self {
        Color::PINK.into()
    }
}

// getter setter
impl RGBA {
    #[inline]
    pub fn r(&self) -> u8 {
        unsafe { self.color.r }
    }
    #[inline]
    pub fn g(&self) -> u8 {
        unsafe { self.color.g }
    }
    #[inline]
    pub fn b(&self) -> u8 {
        unsafe { self.color.b }
    }
    #[inline]
    pub fn a(&self) -> u8 {
        unsafe { self.color.a }
    }

    #[inline]
    pub fn set_r(&mut self, value: u8) {
        self.color.r = value
    }
    #[inline]
    pub fn set_g(&mut self, value: u8) {
        self.color.g = value
    }
    #[inline]
    pub fn set_b(&mut self, value: u8) {
        self.color.b = value
    }
    #[inline]
    pub fn set_a(&mut self, value: u8) {
        self.color.a = value
    }
}

impl RGBA {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { color: ColorValues { a, b, g, r } }
    }
}

lazy_static::lazy_static!(
    static ref ALPHA_MASK: u32 = 255u32.shl(24);
    static ref RGB_MASK: u32 = { let mask: u32 = 255u32.shl(24); !mask };
);
impl RGBA {
    pub fn to_rgb(&self) -> u32 {
        unsafe { self.value & *RGB_MASK }
    }
    pub fn from_rgb(rgb: u32) -> Self {
        Self { value: u32::MAX & *ALPHA_MASK | rgb & *RGB_MASK }
    }
    pub fn split_rgb_a(&self) -> (u32, u32) {
        (self.to_rgb(), self.a() as u32)
    }
}

// grey
impl RGBA {
    pub fn new_grey(value: u8) -> Self {
        Self { color: ColorValues { a: 255, b: value, g: value, r: value } }
    }
    #[inline]
    pub fn grey(&self) -> u8 {
        ( ( self.r() as u16 + self.g() as u16 + self.b() as u16 ) / 3 ) as u8
    }
    #[inline]
    pub fn is_grey(&self) -> bool {
        self.r() == self.g() &&  self.g() == self.b()
    }
    #[inline]
    pub fn to_grey(&mut self) {
        self.set_grey(self.grey());
    }
    #[inline]
    pub fn set_grey(&mut self, value: u8) {
        *self = Self::new_grey(value);
    }
}

// active
impl RGBA {
    #[inline]
    pub fn inactivate(&mut self) -> u8 {
        let out = unsafe { self.color.a };
        self.color.a = 0;
        out
    }
    #[inline]
    pub fn is_active(&self) -> bool {
        self.a() != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        let ray_pink = Color::PINK;
        let pink: RGBA = RGBA::default();

        assert_eq!(ray_pink.r, pink.r());
        assert_eq!(ray_pink.g, pink.g());
        assert_eq!(ray_pink.b, pink.b());
        assert_eq!(ray_pink.a, pink.a());

        assert_eq!(ray_pink, pink.into());
    }

    #[test]
    fn test_transparent() {
        let ray_white = Color::WHITE;
        let mut white: RGBA = ray_white.into();

        assert_eq!(ray_white, white.into());

        assert!(white.is_active());
        white.inactivate();
        assert!(!white.is_active());
        white.set_a(33);
        assert!(white.is_active());
        white.set_a(255);
        assert!(white.is_active());
        white.set_a(1);
        assert!(white.is_active());
        white.set_a(0);
        assert!(!white.is_active());
    }

    #[test]
    fn test_grey() {
        let grey = RGBA::new_grey(33);
        assert!(grey.is_grey());

        let mut not_gray = RGBA::new(32, 33, 36, 255);
        not_gray.to_grey();
        assert_eq!(not_gray.r(), 33);
        assert_eq!(not_gray.g(), 33);
        assert_eq!(not_gray.b(), 33);
        assert_eq!(not_gray.a(), 255);
    }

    #[test]
    fn test_rgb() {
        let pink: RGBA = Color::PINK.into();
        assert_eq!(pink, RGBA::from_rgb(pink.to_rgb()));
    }
}
