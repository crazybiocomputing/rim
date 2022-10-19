#![allow(non_snake_case)]
#![allow(unused)]

mod image_processor;
mod color_space;
mod image_traits;
mod image_stack;
mod stats;
mod operator;


fn main() {
    use image_processor::*;
    use color_space::ColorSpace;
    use image_traits::Access;
    use image_stack::ImageStack;
    use stats::Stats;
    use operator::Operator;

    let mut img = ByteProcessor::create_byte_processor(10,10);
    img.debug();

    println!("Pixel in position x=0, y=0 : {}", img.get_pixel_at(0,0));
    println!("Pixel in position x=1, y=1 : {}", img.get_pixel_at(1,1));

    img.set(0,10);
    println!("Pixel in position x=0, y=0 : {}", img.get_pixel_at(0,0));
    println!("Pixel in position x=1, y=1 : {}", img.get_pixel_at(1,1));

    img.add(1);
    println!("Pixel in position x=0, y=0 : {}", img.get_pixel_at(0,0));
    println!("Pixel in position x=1, y=1 : {}", img.get_pixel_at(1,1));
    
    img.ceil(8);
    println!("Pixel in position x=0, y=0 : {}", img.get_pixel_at(0,0));
    println!("Pixel in position x=1, y=1 : {}", img.get_pixel_at(1,1));
    
    img.floor(2);
    println!("Pixel in position x=0, y=0 : {}", img.get_pixel_at(0,0));
    println!("Pixel in position x=1, y=1 : {}", img.get_pixel_at(1,1));
    
    img.multiply(2);
    println!("Pixel in position x=0, y=0 : {}", img.get_pixel_at(0,0));
    println!("Pixel in position x=1, y=1 : {}", img.get_pixel_at(1,1));

    img.divide(3);
    println!("Pixel in position x=0, y=0 : {}", img.get_pixel_at(0,0));
    println!("Pixel in position x=1, y=1 : {}", img.get_pixel_at(1,1));

    //println!("Average {}", img.get_average_value());


    //println!("Histogram {:?}", img.get_histogram());


    

}
