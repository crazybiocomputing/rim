pub mod io;
mod image_processor;
mod color_space;
mod image_traits;
mod image_stack;
mod stats;

use std::error::Error;

pub fn really_complicated_code(a: u8, b: u8) -> Result<u8, Box<dyn Error>> {
    Ok(a + b)
}

pub fn test() {
    println!("Test");
}
