#![allow(non_snake_case)]
#![allow(unused)]

//mod float_processor;

mod image_processor;
mod color_space;
mod image_traits;


fn main() {
    use image_processor::ImageProcessor;
    use color_space::ColorSpace;
    use crate::image_traits::Access;


    let mut img_byte = ImageProcessor::<u8>::create_byte_processor(10,100);
    img_byte.debug();

    println!("Pixel in position 100 : {}", img_byte.get_pixel(100));
    println!("Pixel in position x=5, y=50 : {}", img_byte.get_pixel_at(5,50));

}
