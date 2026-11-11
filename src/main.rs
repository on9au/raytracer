use std::io::stdout;

use raytracer::{
    color::Color,
    image::Image,
    output::{Output, plain_ppm::PlainPpm},
};

fn main() {
    // image

    let image_width: usize = 256;
    let image_height: usize = 256;
    let mut pixels: Vec<Color> = vec![];

    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            pixels.push(Color::new((r, g, b)));
        }
    }

    let image = Image {
        height: image_height,
        width: image_width,
        pixels,
    };

    PlainPpm::write(&image, &mut stdout());
}
