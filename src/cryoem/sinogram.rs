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
// Authors: Romane Cathelin, Paul Sastourne
 
use std::f64::consts::PI;
use crate::float_processor::FloatProcessor;
use crate::color_space::ColorSpace;
use crate::grayscale::Gray;
use crate::image_traits::Access;

pub struct Sinogram {}

impl Sinogram {
    ///
    ///
    ///
    ///
    pub fn new(ip: &FloatProcessor, angle_list : &Vec<f32>) -> FloatProcessor {
        let num = angle_list.len();
        let mut r : Vec<f32> = Vec::<f32>::new();
        for i in 0..num {
            let th_deg : f32 =  angle_list[i];
            r.append(&mut Self::radon_transform(&ip, th_deg));
        }
        //return
        FloatProcessor::new(ip.get_width(),ip.get_height(),r, Gray::<f32>::new())

    }
    ///
    ///
    ///
    pub fn new_in_range(ip: &FloatProcessor,start: f32, end: f32, step: f32) -> FloatProcessor {
        let num : usize = ((end - start) / step) as usize;
        let angles : Vec<f32> = (0..num).map(|i| i as f32 * step).collect();
        Sinogram::new(ip,&angles)
    }
    

    // Private Function to perform the radon transform on the attenuation matrix
    fn radon_transform(ip : &FloatProcessor, angle : f32) -> Vec<f32> {
        let mut r : Vec<f32> = vec![0.0 ; ip.get_width() as usize];

        let cx : f32 = (ip.get_width()as f32 / 2.0).floor();
        let cy : f32 = (ip.get_height() as f32 / 2.0).floor();
            for x in 0..ip.get_width() {
                for y in 0..ip.get_height() {
                        // convert the angle to radians
                        let th_rad : f32 = angle * std::f32::consts::PI / 180.0;
                        // calculate perpendicular distance from line given angle theta 
                        let t : i64 = (((x as f32)-cx)*th_rad.cos() + ((y as f32)-cy)*th_rad.sin()).round() as i64;
                        // adding values along line
                        // Nearest or bilinear????
                        let index = (t + cx as i64) as usize;
                        if index  < ip.get_width() as usize {
                            r[index] += ip.get_pixel_at(x as u32, y as u32).unwrap(); // as f32; 
                        }
                    }
                }
        

        r // return the radon transform matrix
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::image_processor::*;
    use crate::image_traits::Access;
    use crate::operator::Operator;

    #[test]
    pub fn sinogram_with_5x5_ip() {
        let test = vec![
          1.0,2.0,3.0,4.0,5.0,
          6.0,7.0,8.0,9.0,10.0,
          11.0,12.0,13.0,14.0,15.0,
          16.0,17.0,18.0,19.0,20.0,
          21.0,22.0,23.0,24.0,25.0
        ];
        let ip = FloatProcessor::new(5,5,test, Gray::<f32>::new());
        let op = Sinogram::new_in_range(&ip,0.0,1.0,1.0);
        let pixels = op.data(); // Vec<f32>
        let answer = vec![55.0,60.0,65.0,70.0, 75.0];
        assert!(pixels.iter().zip(answer).all(|(a,b)| *a == b));
    }
}
