extern crate image;
extern crate argparse;

use image::*;
use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {



    let img = read_image_in("pixelrick.jpg").unwrap();

    let img1 = img.resize(100, 100, FilterType::Lanczos3);

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img1.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img1.color());
}


fn read_image_in(filename: &str) -> ImageResult<DynamicImage> {
    return image::open(filename);
}