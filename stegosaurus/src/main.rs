extern crate image;

use image::{GenericImageView, GenericImage};
use image::jpeg::JPEGEncoder;
use std::fs::File;
/*
Unobtrusive uniquely identifying watermark that is tolerant to cropping, resizing, compression/reencoding, and potentially other transformations (such as shear) and not easily recognizable by a human eye.
    - Modify pixel values slightly in unique pattern across whole image (checkerboard for example)
    - Adjust pixel triplets (r, g, b) such that they are minimally changed but have a relationship thats unlikely to occur randomly
        - For example, r, g, b are all prime numbers
*/
fn main() {
    println!("Hello, world!");
    let img = image::open(String::from("/home/malcolm/projects/stegosaurus/stegosaurus/kitten.jpg"));
    if let Ok(mut di) = img {
        println!("Read an image");
        let (width, height) = di.dimensions();
        for w in 0..width {
            for h in 0..height {
                /*
                let mut px = di.get_pixel(w, h);
                let (red, green, blue, alpha)  = (px.0[0], px.0[1], px.0[2], px.0[3]);
                let mut red_new = red;
                if false  {
                    red_new = (red + 1) % 255;
                    println!("Old red: {:}, new red: {:}", red, red_new);
                }

                let new_px = image::Rgba([red_new, green, blue, alpha]);
                di.put_pixel(w, h, new_px);
                */
            }
        }

        // di.save(String::from("/home/malcolm/projects/stegosaurus/stegosaurus/testimg1-unmodified.jpg"));

        // TODO: Doesn't work - need to see if encoding with 100 quality prevents output appearing dark
        let mut writer = File::create("/home/malcolm/projects/stegosaurus/stegosaurus/out-kitten.jpg").unwrap();
        let mut encoder = JPEGEncoder::new_with_quality(&mut writer, 100);
        encoder.encode_image(&di);
    }
}
