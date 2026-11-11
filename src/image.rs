//! Image

use crate::color::Color;

pub struct Image {
    pub height: usize,
    pub width: usize,
    pub pixels: Vec<Color>,
}
