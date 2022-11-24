//
//  RIM - Rust Image
//  Copyright (C) 2022  Jean-Christophe Taveau.
//
//  This file is part of RIM
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with RIM.  If not, see <http://www.gnu.org/licenses/>.
//
// Authors: Mathieu Arru, Léa Chabot

use crate::{grayscale::Gray32, color_space::ColorSpace, float_processor::FloatProcessor};
use std::{f64::consts::PI, vec};
use crate::image_traits::Access;

pub struct Section {}

impl Section {

    pub fn new(ip: &FloatProcessor) -> FloatProcessor {
        let mut result : Vec<f32> = vec![0.0 ; (ip.get_width()*ip.get_width()) as usize];
        let angles = Self::compute_angles(ip);

        for row in 0..angles.len(){ 
            let bp = Self::back_projection_nearest(&ip, angles[row], row as u32);
            for y in 0..ip.get_width(){
                for x in 0..ip.get_width(){
                    let index = x + ip.get_width()*y;
                    result[index as usize] += bp[x as usize];
                }
            }
            println!("{:?}", result);
        }
        FloatProcessor::new(ip.get_width(),ip.get_width(), result as Vec<f32>, Gray32::new())
    }

    // Back projection for an angle (linear)
    pub fn back_projection_linear(ip : &FloatProcessor, angle : f32, y : u32) -> Vec<f32> {
        let index = Self::rotate(ip, angle, y);
        Self::interploate_linear(ip, index, y)
    }

    // Back projection for an angle (nearest)
    pub fn back_projection_nearest(ip : &FloatProcessor, angle : f32, y : u32) -> Vec<f32> {
        let index = Self::rotate(ip, angle, y);
        Self::nearest(ip, index, y)
    }


    pub fn compute_angles(ip : &FloatProcessor) -> Vec<f32> {
        let mut angles: Vec<f32> = Vec::new();
        let step = (180/ip.get_height()) as f32;
            for i in 0..ip.get_height() {
                angles.push(-1.0 * i as f32 * step);
            }
        angles
    }

    pub fn rotate(ip: &FloatProcessor, angle: f32, y: u32) -> Vec<(f32,f32)> {
        let mut index = Vec::new();

        //Compute center
        let center: f32 = (ip.get_width()as f32/2.0).round();

        // Convert the angle to radians
        let angle_rad: f32 = (angle as f32) * PI as f32 / 180.0;

        // Compute x' and y'
        for x in 0..ip.get_width(){
            let x_prime: f32 = (((x as f32) - center )*angle_rad.cos() - ((y as f32) - center)*angle_rad.sin()) as f32;
            let y_prime: f32 = (((x as f32) - center )*angle_rad.sin() + ((y as f32) - center)*angle_rad.cos()) as f32;
            index.push((x_prime, y_prime));
            println!("XPRIME AVANT CENTER {x_prime}, YPRIME {y_prime}, ANGLE {angle}");
        }
        index
    }

    pub fn interploate_linear(ip: &FloatProcessor, index: Vec<(f32,f32)>, y: u32) -> Vec<f32>{
        let mut result : Vec<f32> = vec![0.0 ; ip.get_width() as usize];

        //Compute center
        let center: f32 = (ip.get_width()as f32/2.0).round();

        for x in 0..ip.get_width(){
            let x_prime = index[x as usize].0 + center;
            if x_prime < 8.0 && x_prime > -1.0{
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
            }
        }
        result
    }

    pub fn nearest(ip: &FloatProcessor, index: Vec<(f32,f32)>, y: u32) -> Vec<f32>{
        let mut result : Vec<f32> = vec![0.0 ; ip.get_width() as usize];

        //Compute center
        let center: f32 = (ip.get_width()as f32/2.0).round();

        for x in 0..ip.get_width(){
            let x_prime = index[x as usize].0.round() + center;
            if x_prime >= 0.0 && x_prime < ip.get_width() as f32{
                result[x_prime as usize] += ip.get_pixel_at(x as u32, y as u32).unwrap() as f32;
                println!("XPRIME {x_prime} | VALUE ADD{}", ip.get_pixel_at(x as u32, y as u32).unwrap());
            }
        }
    println!("ROW ADD{:?}", result);
    result
    }
        

}
    

//#[cfg(test)]
//mod tests {

    use super::*;
    use crate::io::{image_reader::OutputProcessor, image_writer::FileSaver};
    use crate::image_processor::ImageProcessor;
    use crate::io::file_info::FileInfo;

    //#[test]
    pub fn tomography() {

        let section = vec![
            0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,
            0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,
            0.000,0.000,1.000,1.000,0.000,0.000,0.000,0.000,
            0.000,0.000,0.000,1.000,1.000,0.000,0.000,0.000,
            0.000,0.000,0.000,1.000,0.000,0.000,0.000,0.000,
            0.000,0.000,0.000,1.000,1.000,1.000,0.000,0.000,
            0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000,
            0.000,0.000,0.000,0.000,0.000,0.000,0.000,0.000];

        let chessboard = vec![
            255f32, 0f32, 255f32, 0f32, 255f32, 0f32, 255f32, 0f32,
            0f32, 255f32, 0f32, 255f32, 0f32, 255f32, 0f32, 255f32,
            255f32, 0f32, 255f32, 0f32, 255f32, 0f32, 255f32, 0f32,
            0f32, 255f32, 0f32, 255f32, 0f32, 255f32, 0f32, 255f32,
            255f32, 0f32, 255f32, 0f32, 255f32, 0f32, 255f32, 0f32,
            0f32, 255f32, 0f32, 255f32, 0f32, 255f32, 0f32, 255f32,
            255f32, 0f32, 255f32, 0f32, 255f32, 0f32, 255f32, 0f32,
            0f32, 255f32, 0f32, 255f32, 0f32, 255f32, 0f32, 255f32,
        ];

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
        let test = Section::new(&sino_ip);
        let op = OutputProcessor::FloatProcessor(test);
        FileSaver::save_processor("./src/cryoem/test_backprojection", FileInfo::GRAY32_FLOAT, op)
    }
//}