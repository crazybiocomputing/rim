#![allow(non_snake_case)]
#![allow(unused)]

//mod float_processor;

mod image_processor;
mod color_space;
mod image_traits;
mod image_stack;
mod stats;


fn main() {
    use image_processor::ImageProcessor;
    use color_space::ColorSpace;
    use image_traits::Access;
    use image_stack::ImageStack;
    use stats::Stats;


    let mut img = ImageProcessor::<(u8,u8,u8)>::create_color_processor(10,100);
    img.debug();

    let mut pixel = img.get_pixel_at(0,0);
    println!("Pixel in position x=1, y=1 : {},{},{}", pixel.0,pixel.1,pixel.2);

    img.set(0,(10,20,50));

    pixel = img.get_pixel_at(0,0);
    println!("Pixel in position x=1, y=1 : {},{},{}", pixel.0,pixel.1,pixel.2);

    /*let column = img.get_column(0,0);
    for i in column {
        println!("{},{},{}",i.0, i.1, i.2)
    }*/

    let test = ImageStack::<f32>::create_float_stack(10,10,0);
    let im = ImageProcessor::<f32>::create_float_processor(10,10);
    test.debug_stack();
    test.set_data_stack(im);
    test.debug_stack();
    let im1 = ImageProcessor::<f32>::create_float_processor(10,10);
    test.set_data_stack(im1);
    test.set_slice_number(1);

    let mut img_stack= test.get_one_slice();

    let mut p = img_stack.get_pixel_at(0,0);
    println!("Pixel in position x=1, y=1 : {}", p);

    img_stack.set(0,10.0);
    let mut p = img_stack.get_pixel_at(0,0);
    println!("Pixel in position x=1, y=1 : {}", p);
    test.set_slice_number(1);




}
