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

    let mut img = FloatProcessor::create_float_processor(10,10);
    img.debug();

    let mut pixel = img.get_pixel_at(0,0);
    println!("Pixel in position x=0, y=0 : {}", pixel);
    println!("Pixel in position x=1, y=1 : {}", img.get_pixel_at(1,1));

    img.set(0,10.0);
    img.add(1.0);
    img.ceil(8.3);
    img.floor(1.2);
    img.multiply(2.0);
    img.divide(1.3);

    pixel = img.get_pixel_at(0,0);
    println!("Pixel in position x=0, y=0 : {}", pixel);
    println!("Pixel in position x=1, y=1 : {}", img.get_pixel_at(1,1));

    //println!("Average {}", img.get_average_value());


    //println!("Histogram {:?}", img.get_histogram());


    

}
