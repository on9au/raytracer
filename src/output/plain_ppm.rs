//! Plain Portable Pixmap Format (`.ppm`) `P3`

use crate::{image::Image, output::Output};

pub struct PlainPpm;

impl Output for PlainPpm {
    fn write(image: &Image, w: &mut impl std::io::prelude::Write) {
        writeln!(w, "P3\n{} {}\n255", image.width, image.height);

        image.pixels.iter().try_for_each(|c| {
            // translate [0,1] to [0,255]
            let r = (c.x * 255.999) as i64;
            let g = (c.y * 255.999) as i64;
            let b = (c.z * 255.999) as i64;

            writeln!(w, "{r} {g} {b}")
        });
    }
}
