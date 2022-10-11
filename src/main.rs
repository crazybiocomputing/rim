#![allow(non_snake_case)]
#![allow(unused)]

//mod float_processor;

mod image_processor;
mod color_space;

fn main() {
    use image_processor::ImageProcessor;
    use color_space::ColorSpace;

    let img_byte = ImageProcessor::<u8>::create_byte_processor(10,100);
    img_byte.debug();

    let img_float = ImageProcessor::<f32>::create_float_processor(10,100);
    img_float.debug();

    let img_color = ImageProcessor::<u8>::create_color_processor(10,100);
    img_color.debug();

}
