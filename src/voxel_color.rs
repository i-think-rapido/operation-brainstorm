use anyhow::bail;
use tracing_subscriber::field::debug;

use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Index {
    dimensions: Arc<Coordinate>,
    value: Coordinate,
}
impl Index {
    pub fn from_coordinates(x: usize, y: usize, z: usize, dimensions: &Arc<Coordinate>) -> anyhow::Result<Self> {
        if x >= dimensions.x { bail!("x coordinate is too big") }
        if y >= dimensions.x { bail!("y coordinate is too big") }
        if z >= dimensions.x { bail!("z coordinate is too big") }
        Ok(Self { dimensions: dimensions.clone(), value: Coordinate { x, y, z }})
    }
    pub fn from_usize(idx: usize, dimensions: &Arc<Coordinate>) -> anyhow::Result<Self> {
        if dimensions.x * dimensions.y * dimensions.z <= idx { bail!("index is too big") }
        let x = idx % dimensions.x;
        let y = (idx / dimensions.x) % dimensions.y;
        let z = (idx / dimensions.y / dimensions.x) % dimensions.z;
        Ok(Self { dimensions: dimensions.clone(), value: Coordinate { x, y, z }})
    }
}
impl From<Index> for Option<usize> {
    fn from(index: Index) -> Self {
        if  index.dimensions.x <= index.value.x ||
            index.dimensions.y <= index.value.y ||
            index.dimensions.z <= index.value.z
        {
            None
        }
        else {
            Some(   index.value.z * index.dimensions.y * index.dimensions.x 
                +   index.value.y * index.dimensions.x 
                +   index.value.x
            )
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}
impl Color {
    pub fn into_raylib(&self, alpha: u8) -> raylib::color::Color {
        if alpha == 0 {
            NONE_COLOR.clone()
        }
        else {
            raylib::color::Color::new(self.r, self.g, self.b, alpha)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorValue(Color, u8);
impl ColorValue {
    pub fn to_none(&self) -> ColorValue {
        let mut out = self.clone();
        out.1 = 0;
        out
    }
    pub fn to_alpha(&self, alpha: u8) -> ColorValue {
        let mut out = self.clone();
        out.1 = alpha;
        out
    }
    pub fn color(&self) -> &Color {
        &self.0
    }
    pub fn alpha(&self) -> u8 {
        self.1
    }
}
impl From<raylib::color::Color> for ColorValue {
    fn from(value: raylib::color::Color) -> Self {
        ColorValue(Color { r: value.r, g: value.g, b: value.b }, value.a )
    }
}
impl From<ColorValue> for raylib::color::Color {
    fn from(value: ColorValue) -> Self {
        if value.1 == 0 {
            NONE_COLOR.clone()
        }
        else {
            raylib::color::Color{ r: value.0.r, g: value.0.g, b: value.0.b, a: value.1 }
        }
    }
}
pub type Colors = HashMap<Index, ColorValue>;

const DUMMY_COLOR: raylib::color::Color = raylib::color::Color::PINK;
lazy_static::lazy_static!(
    static ref NONE_COLOR: raylib::color::Color = DUMMY_COLOR.alpha(0.0);
);

#[derive(Debug, Clone)]
pub struct VoxelColors {
    dimensions: Arc<Coordinate>,
    colors: Colors,
}
impl VoxelColors {
    pub fn new(dim_x: usize, dim_y: usize, dim_z: usize) -> Self {
        VoxelColors { dimensions: Arc::new(Coordinate{ x: dim_x, y: dim_y, z: dim_z}), colors: Colors::default() }
    }
    pub fn set(&mut self, idx: &Index, color: ColorValue) {
        self.colors.insert(idx.clone(), color);
    }
    pub fn get(&mut self, idx: &Index) -> Option<ColorValue> {
        self.colors.get(idx).cloned()
    }
    pub fn to_none(&mut self, idx: &Index) {
        if let Some(color) = self.colors.get_mut(idx) {
            *color = color.to_none();
        }
    }
    pub fn to_alpha(&mut self, idx: &Index, alpha: u8) {
        if let Some(color) = self.colors.get_mut(idx) {
            *color = color.to_alpha(alpha);
        }
    }
    pub fn capacity(&self) -> usize {
        self.dimensions.x * self.dimensions.y * self.dimensions.z
    }
    pub fn dimensions(&self) -> Arc<Coordinate> {
        self.dimensions.clone()
    }
}

