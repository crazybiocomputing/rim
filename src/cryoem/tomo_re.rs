use crate::{image_processor::ImageProcessor, grayscale::Gray32, color_space::ColorSpace, float_processor::FloatProcessor, image_stack::ImageStack, io::{image_reader::OutputProcessor, image_writer::FileSaver}};
use std::{f64::consts::PI};
use crate::image_traits::Access;
use crate::io::file_info::*;



pub fn tomography() {
    let test = sino_to_section();
    let op = OutputProcessor::FloatProcessor(test);
    FileSaver::save_processor("./test_backprojection", FileInfo::GRAY32_FLOAT, op)
}


//Open test file
pub fn sino_to_section() -> FloatProcessor{
    let section = vec![
        0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,
        0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,
        0.000,0.000,1.000,1.000,0.000,0.000,0.000,0.000,
        0.000,0.000,0.000,1.000,1.000,0.000,0.000,0.000,
        0.000,0.000,0.000,1.000,0.000,0.000,0.000,0.000,
        0.000,0.000,0.000,1.000,1.000,1.000,0.000,0.000,
        0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,
        0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000];

    let sinogram = vec![
        0.000,0.000,0.245,1.000,0.498,0.245,0.000,0.000,
        0.087,0.000,0.253,0.956,0.640,0.174,0.024,0.024,
        0.119,0.024,0.245,0.925,0.814,0.079,0.024,0.087,
        0.087,0.047,0.435,0.672,0.727,0.119,0.024,0.119,
        0.024,0.047,0.601,0.451,0.617,0.348,0.000,0.087,
        0.000,0.000,0.751,0.245,0.498,0.498,0.000,0.000,
        0.000,0.111,0.577,0.308,0.474,0.435,0.142,0.119,
        0.055,0.166,0.451,0.395,0.451,0.427,0.229,0.150,
        0.055,0.142,0.292,0.553,0.490,0.340,0.206,0.150,
        0.000,0.095,0.261,0.514,0.743,0.324,0.126,0.119,
        ];
    

        let sino_ip = ImageProcessor::new(8,10, sinogram, Gray32::new());
        let nb_proj = sino_ip.get_height();
        
        // Precompute angles
        let step = (180/nb_proj) as f32;
        let mut angles: Vec<f32> = Vec::new();
        for i in 0..nb_proj {
            angles.push(-1.0 * i as f32 * step);
        }
        //new(&sino_ip, &angles)
        test(&sino_ip, -54.0)
        
        
}

// Use back_projection with all angles compute
pub fn new(ip: &FloatProcessor, angle_list : &Vec<f32>) -> FloatProcessor {
    let num = angle_list.len();
    let mut r : Vec<f32> = vec![0.0 ; (ip.get_width()*ip.get_width()) as usize];
    for i in 0..num {
        let angle : f32 =  angle_list[i];
        println!("{angle}");
        let bp = back_projection(&ip, angle, i as u32);
        for y in 0..ip.get_width(){
            for x in 0..ip.get_width(){
                let index = x + ip.get_width()*y;
                r[index as usize] += bp[x as usize];
                
            }
        }
    }
    //return the section
    FloatProcessor::new(ip.get_width(),ip.get_width(),r as Vec<f32>, Gray32::new())
}

pub fn test(ip: &FloatProcessor, angle : f32) -> FloatProcessor {
    let mut r : Vec<f32> = vec![0.0 ; (ip.get_width()*ip.get_width()) as usize];
    let bp = back_projection(&ip, angle, 3);
    for y in 0..ip.get_width(){
        for x in 0..ip.get_width(){
            let index = x + ip.get_width()*y;
            r[index as usize] += bp[x as usize];
        }
    }
    FloatProcessor::new(ip.get_width(),ip.get_width(),r as Vec<f32>, Gray32::new())
    }

// Back projection for an angle
pub fn back_projection(ip : &FloatProcessor, angle : f32, y: u32) -> Vec<f32> {
    let mut result : Vec<f32> = vec![0.0 ; ip.get_width() as usize];

    let center : f32 = (ip.get_width()as f32/2.0).round();
        for x in 0..ip.get_width(){

            // Convert the angle to radians
            let angle_rad : f32 = (angle as f32) * PI as f32 / 180.0;

            // Compute x'
            let x_prime : f32 = ((((x as f32) - center )*angle_rad.cos() - ((y as f32) - center)*angle_rad.sin()) as f32) + center ;

            // integration
            if x_prime <= 8.0 && x_prime >= -1.0{
                let x_floor = x_prime.floor();
                let x_ceil = x_prime.ceil();
                let x_floor_prop = x_ceil - x_prime;
                let x_ceil_prop = 1.0 - x_floor_prop;
                
                if x_floor >= 0.0 {
                    result[x_floor as usize] += ip.get_pixel_at(x as u32, y as u32).unwrap() as f32 * x_floor_prop; 
                }
                if x_ceil <= 7.0 {
                    result[x_ceil as usize] += ip.get_pixel_at(x as u32, y as u32).unwrap() as f32 * x_ceil_prop; 
                }
                println!("X : {x}, y : {y}, X PRIME : {x_prime}, CEIL PROP : {x_ceil}, FLOOR PROP {x_floor}, CENTER {center}, ANGLE {angle}, VALUE : {}, ADDFLOOR: {}, ADDCEIL: {}", ip.get_pixel_at(x as u32, y as u32).unwrap(), ip.get_pixel_at(x as u32, y as u32).unwrap() as f32 * x_floor_prop, ip.get_pixel_at(x as u32, y as u32).unwrap() as f32 * x_ceil_prop);
            }   
        }
    //Return
    result
}

/* 
pub fn projection_to_stack(){
    }
*/


pub fn linear_resize(vec: Vec<f64>, size: usize) -> Vec<f64>{
    let mut output = Vec::new();
    let ratio = (vec.len() -1) / (size - 1);
    for i in 0..size {
        let low = ((ratio*i) as f64).floor();
        let high = ((ratio*i) as f64).ceil();
        let weight = (ratio * i) as f64 - low;

        let a = vec[low as usize];
        let b = vec[high as usize];

        output.push(a * (1.0-weight)+b*weight);
    }
    return output;
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

