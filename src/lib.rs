pub mod io;
pub mod color_space;

use std::error::Error;

pub fn really_complicated_code(a: u8, b: u8) -> Result<u8, Box<dyn Error>> {
    Ok(a + b)
}

pub fn test() {
    println!("Test");
}
