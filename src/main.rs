#![allow(non_snake_case)]
#![allow(unused)]

mod image_processor;
mod color_space;
mod image_traits;
mod image_stack;
mod stats;
pub mod io;

fn main() {
    use image_processor::*;
    use color_space::*;
    use image_traits::*;
    use image_stack::*;
    use stats::*;
    use rim::io::raw_reader::*;
    use rim::io::image_reader::*;
    use std::env;


    let args: Vec<String> = env::args().collect();
    let filename = &args[1].to_string();
    let filename = &args[2].to_string();


    let stack = read_byte_stack(2,2,2,&filename);
    let name = "name".to_string();
    save_byte_stack(stack,&name);

    let stack_2 = read_color_stack(2,2,2,&filename);
    let name_2 = "name_2".to_string();
    save_color_stack(stack,&name);
}
