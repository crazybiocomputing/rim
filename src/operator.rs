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

use crate::image_processor::ImageProcessor;
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

    /*
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
    fn gamma​(value: f64);

    
    
    
    static fn set_random_seed​(random_seed: f64);

    /// Adds pseudorandom, Gaussian ("normally") distributed values, 
    /// with mean 0.0 and the specified standard deviation, to this image or ROI.
    fn noise​(&self,standard_deviation: f64);


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
            i.add(value)
        }
    }
    fn substract(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.substract(value)
        }
    }
    fn multiply(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.multiply(value)
        }
    }
    fn divide(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.divide(value)
        }
    }

    fn ceil(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.ceil(value)
        }
    }
    fn floor(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.floor(value)
        }
    }

    fn and(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.and(value)
        }
    }fn or(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.or(value)
        }
    }fn xor(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.xor(value)
        }
    }

}