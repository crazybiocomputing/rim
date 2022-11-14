#![allow(non_snake_case)]
#![allow(unused)]
/*

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

    let test = (0.0/4.0)+(0.0/4.0)+(255.0/4.0)+(130.0/4.0);
    println!("{}",test);

    let bon :u32 = 4;
    let ban = bon as f32;
    println!("bon : {} et ban : {}",bon,ban);

    //img2.get_histogram();
    //println!("Histogram {:?}", img.get_histogram());


    let color = ColorProcessor::create_color_processor(10,20);
    let pixel = color.get(0);
    println!("rgb {:?}",pixel);
    println!("r {}, g {} et b {}",pixel.0,pixel.1,pixel.2);

}
*/

fn main() {

    /*
        use rim::color_space::*;
        use rim::grayscale::Gray8;
        use rim::image_processor::*;
        use rim::io::image_reader::*;
        use rim::statistics::Statistics;

        // Byte Processor
        let filename = "blobs.raw".to_string();
        let mut stack = read_byte_stack(254, 256, 1, &filename);
        let mut image = ImageProcessor::new(256, 254, stack.data()[0].clone(), Gray8::new());

        image.update_stats();
        println!("{:?}", image.histogram());

        // Histogramme théoriquement implémenté pour Float, pas eu le temps de tester
    */
}
