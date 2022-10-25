pub mod color_space;
pub mod gray_processor;
pub mod grayscale;
pub mod image_processor;
pub mod io;
pub mod pixel;

// ImageProcessor compatible with ImageJ

pub mod byte_processor;
pub mod float_processor;
pub mod image_stack;
pub mod image_traits;
pub mod short_processor;
// pub mod stats;
pub mod operator;

use std::error::Error;

pub fn really_complicated_code(a: u8, b: u8) -> Result<u8, Box<dyn Error>> {
    Ok(a + b)
}

pub fn test() {
    println!("Test");
}
