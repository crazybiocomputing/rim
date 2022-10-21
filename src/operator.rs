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
use std::ops::Add;
use rand::Rng;
use rand::distributions::normal::Normal;
use rand::distributions::IndependentSample;

use crate::image_processor::*;
use crate::image_stack::ImageStack;

pub trait Operator<T> {
    type Input;
    /// Adds 'value' to each pixel in the image or ROI.
    fn add(&self, value: Self::Input);

    /// Removes 'value' to each pixel in the image or ROI.
    fn substract(&self, value: Self::Input);

    /// Multiplies each pixel in the image or ROI by 'value'.
    fn multiply(&self, value: Self::Input);

    /// Divides each pixel in the image or ROI by 'value'.
    fn divide(&self, value: Self::Input);

    /// Pixels greater than 'value' are set to 'value'.
    fn ceil(&self, value: Self::Input);

    /// Pixels less than 'value' are set to 'value'.
    fn floor(&self, value: Self::Input);

    /// Binary AND of each pixel in the image or ROI with 'value'.
    fn and(&self, value: Self::Input);

    /// Binary OR of each pixel in the image or ROI with 'value'.
    fn or(&self, value: Self::Input);
    
    /// Binary exclusive OR of each pixel in the image or ROI with 'value'.
    fn xor(&self, value: Self::Input);
    

}

impl<T> Operator<T> for ImageProcessor<T> 
    where T : Copy 
        + std::ops::AddAssign 
        + std::ops::SubAssign 
        + std::ops::MulAssign
        + std::ops::DivAssign
        + std::cmp::PartialOrd
        + std::ops::BitAnd<Output=T>
        + std::ops::BitOr<Output=T>
        + std::ops::BitXor<Output=T>
    {

    type Input = T;

    fn add(&self, value: Self::Input){
        for i in self.get_data().iter_mut() {
            *i += value;
        }
    }
    fn substract(&self, value: Self::Input){
        for i in self.get_data().iter_mut() {
            *i -= value;
        }
    }
    fn multiply(&self, value: Self::Input){
        for i in self.get_data().iter_mut() {
            *i *= value;
        }
    }
    fn divide(&self, value: Self::Input){
        for i in self.get_data().iter_mut() {
            *i /= value;
        }
    }

    fn ceil(&self, value: Self::Input){
        for i in self.get_data().iter_mut() {
            if *i > value {
                *i = value;
            }
    
        }
    }
    fn floor(&self, value: Self::Input){
        for i in self.get_data().iter_mut() {
            if *i < value {
                *i = value;
            }
        }
    }

    fn and(&self,value: Self::Input){
        for i in self.get_data().iter_mut() {
            *i = *i & value;
        }
    }
    fn or(&self,value: Self::Input){
        for i in self.get_data().iter_mut() {
            *i = *i | value;
        }
    }
    fn xor(&self,value: Self::Input){
        for i in self.get_data().iter_mut() {
            *i = *i ^ value;
        }
    }
}

impl<T> Operator<T> for ImageStack<T>
    where T : Copy 
        + std::ops::AddAssign 
        + std::ops::SubAssign 
        + std::ops::MulAssign
        + std::ops::DivAssign
        + std::cmp::PartialOrd
        + std::ops::BitAnd<Output=T>
        + std::ops::BitOr<Output=T>
        + std::ops::BitXor<Output=T>
    {

    type Input = T;

    fn add(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().add(value);
        }
    }
    fn substract(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().substract(value);
        }
    }
    fn multiply(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().multiply(value);
        }
    }
    fn divide(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().divide(value);
        }
    }

    fn ceil(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().ceil(value);
        }
    }
    fn floor(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().floor(value);
        }
    }

    fn and(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().and(value);
        }
    }fn or(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().or(value);
        }
    }fn xor(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().xor(value);
        }
    }


}

pub trait OperatorSpe<T> {

    /// Adds pseudorandom, Gaussian ("normally") distributed values, 
    /// with mean 0.0 and the specified standard deviation, to this image or ROI.
    fn noise(&self,standard_deviation: f64);

    /// If this is a 32-bit or signed 16-bit image, performs an absolute value transform, otherwise does nothing.
    fn abs(&self);

    
    /// Performs a exponential transform on the image or ROI.
    fn exp(&self);

    /// Performs a square root transform on the image or ROI.
    fn sqrt(&self);

    /// Does a natural logarithmic (base e) transform of the image or ROI.
    fn ln(&self);

    /// Does a logarithmic (base 10) transform of the image or ROI.
    fn log(&self);

    /// Performs gamma correction of the image or ROI.
    fn gamma(&self, value: f64);

    /*

    


    /// Uses the Process/Math/Macro command to apply functional code to this image/volume.
    /// The function takes eight arguments:
    /// v : current pixel/voxel value.
    /// x,y,z : XY- or XYZ-coordinates of the pixel/voxel
    /// w,h: width and height of the processor
    /// a: angle (polar coordinate)
    /// d: distance from center (polar coordinate)
    fn apply_func​(&self, func: F);

    /// Uses the Process/Math/Macro command to apply macro code to this image.
    /// See apply_func(..)
    fn apply_macro​(&self, func: F);

    */


    
    
}


impl OperatorSpe<u8> for ImageProcessor<u8> {
    fn noise(&self,standard_deviation: f64){
        let normal = Normal::new(0.0, standard_deviation);
        for i in self.get_data().iter_mut() {
            let value = normal.ind_sample(&mut rand::thread_rng());
            //println!("{} : {}/{}", *i, value, value as u8);
            if value > 0.0 {
                *i += (value as u8);
            } else if *i < (-value as u8){
                *i = 0;
            } else {
                *i -= (-value as u8);
            }
        }
    }
    fn abs(&self){}
    fn exp(&self){
        for i in self.get_data().iter_mut() {
            *i = f64::powf(std::f64::consts::E, (*i).into()) as u8;
        }
    }
    fn sqrt(&self){
        for i in self.get_data().iter_mut() {
            *i = f32::powf((*i).into(),0.5 ) as u8;
        }
    }
    fn ln(&self){
        for i in self.get_data().iter_mut() {
            *i = (*i as f32).ln() as u8; 
        }
    }
    fn log(&self){
        for i in self.get_data().iter_mut() {
            *i = (*i as f32).log(10.0) as u8; 
        }
    }
    fn gamma(&self, value: f64){
        let max = self.get_max_possible();
        for i in self.get_data().iter_mut() {
            *i = (f64::powf((*i/max).into(), value) * (max as f64)) as u8 ;
        }
    }
}

impl OperatorSpe<f32> for ImageProcessor<f32> {
    fn noise(&self,standard_deviation: f64){
        let normal = Normal::new(0.0, standard_deviation);
        for i in self.get_data().iter_mut() {
            let value = normal.ind_sample(&mut rand::thread_rng());


            *i += (value as f32);
        }

    }
    fn abs(&self){
        for i in self.get_data().iter_mut() {
            *i = *i * -1.0;
        }
    }
    fn exp(&self){
        for i in self.get_data().iter_mut() {
            *i = f64::powf(std::f64::consts::E, (*i).into()) as f32;
        }
    }
    fn sqrt(&self){
        for i in self.get_data().iter_mut() {
            *i = f32::powf((*i).into(),0.5 ) as f32;
        }
    }
    fn ln(&self){
        for i in self.get_data().iter_mut() {
            *i = (*i as f32).ln() as f32;
        }
    }
    fn log(&self){
        for i in self.get_data().iter_mut() {
            *i = (*i as f32).log(10.0) as f32;
        }
    }
    fn gamma(&self, value: f64){
        let max = self.get_max_possible();
        for i in self.get_data().iter_mut() {
            *i = (f64::powf((*i/max).into(), value) * (max as f64)) as f32;
        }
    }
}   

