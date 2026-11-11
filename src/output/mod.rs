//! Output formats

use std::io::Write;

use crate::image::Image;

pub mod plain_ppm;

pub trait Output {
    fn write(image: &Image, w: &mut impl Write);
}
