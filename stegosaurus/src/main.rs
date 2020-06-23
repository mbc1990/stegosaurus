extern crate image;

use image::{GenericImageView, GenericImage};
use image::jpeg::JPEGEncoder;
use std::fs::File;
use image::png::PNGEncoder;
/*
Unobtrusive uniquely identifying watermark that is tolerant to cropping, resizing, compression/reencoding, and potentially other transformations (such as shear) and not easily recognizable by a human eye.
    - Modify pixel values slightly in unique pattern across whole image (checkerboard for example)
    - Adjust pixel triplets (r, g, b) such that they are minimally changed but have a relationship thats unlikely to occur randomly
        - For example, r, g, b are all prime numbers
*/
fn main() {
    let img = image::open(String::from("/home/malcolm/projects/stegosaurus/stegosaurus/whitesquare.jpeg")).unwrap();
    let mut writer = File::create("/home/malcolm/projects/stegosaurus/stegosaurus/whitesquare_reencoded-release22.jpeg").unwrap();
    let mut encoder = JPEGEncoder::new_with_quality(&mut writer, 100);
    encoder.encode_image(&img);
}
