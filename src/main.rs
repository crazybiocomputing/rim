#![allow(non_snake_case)]
#![allow(unused)]

//mod float_processor;

mod image_processor;
mod color_space;
mod image_traits;
mod image_stack;


fn main() {
    use image_processor::ImageProcessor;
    use color_space::ColorSpace;
    use image_traits::Access;
    //use image_stacks::ImageStack;


    let mut img = ImageProcessor::<(u8,u8,u8)>::create_color_processor(10,100);
    img.debug();

    let mut pixel = img.get_pixel_at(0,0);
    println!("Pixel in position x=1, y=1 : {},{},{}", pixel.0,pixel.1,pixel.2);

    img.set(0,(10,20,50));

    pixel = img.get_pixel_at(0,0);
    println!("Pixel in position x=1, y=1 : {},{},{}", pixel.0,pixel.1,pixel.2);

}
