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
    use crate::image_stack::FloatStack;


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

    let mut stack= FloatStack::create_float_stack(1,1,1);
    stack.set_pixel(0,20.5);
    let pixel = stack.get_pixel(0);
    let row = stack.get_row(0,0);
    println!("Les lignes : {:?}, le pixel : {}",row,pixel);
    let mut img2 =FloatProcessor::create_float_processor(2,2);
    img2.set_row(0,0,vec![255.0,130.30]);
    let max=img2.get_max_value();
    unsafe{
    let a = img2.get_mean();
    println!("voici {} et la moy : {}",max,a);
    }

    let test = (0.0/4.0)+(0.0/4.0)+(255.0/4.0)+(130.0/4.0);
    println!("{}",test);    

    //println!("Histogram {:?}", img.get_histogram());


    

}
