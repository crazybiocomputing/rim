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

/*
use std::ops::Add;
use rand::Rng;
use rand::distributions::normal::Normal;
use rand::distributions::IndependentSample;


use crate::image_processor::*;
use crate::image_stack::ImageStack;
*/

use crate::pixel::PixelType;

pub trait Operator<T: PixelType> {
    type Output;
    fn macro_op(&mut self, f: fn(T, f32, f32, f32, u32, u32, f32, f32) -> T);
    ///
    /// Generic operation to modify a processor by a scalar value
    fn macro_scalar(&mut self, scalar: T, f: fn(T, f32) -> T);

    /// Adds 'value' to each pixel in the image or ROI.
    fn add(&mut self, value: T);

    /// Subtracts 'value' to each pixel in the image or ROI.
    fn subtract(&mut self, value: T);

    /// Multiplies each pixel in the image or ROI by 'value'.
    fn multiply(&mut self, value: T);

    /// Divides each pixel in the image or ROI by 'value'.
    fn divide(&mut self, value: T);

    /// Pixels greater than 'value' are set to 'value'.
    fn ceil(&mut self, value: T);

    /// Pixels less than 'value' are set to 'value'.
    fn floor(&mut self, value: T);

    /// Binary AND of each pixel in the image or ROI with 'value'.
    fn and(&mut self, value: T);

    /// Binary OR of each pixel in the image or ROI with 'value'.
    fn or(&mut self, value: T);

    /// Binary exclusive OR of each pixel in the image or ROI with 'value'.
    fn xor(&mut self, value: T);

    /// Adds pseudorandom, Gaussian ("normally") distributed values,
    /// with mean 0.0 and the specified standard deviation, to this image or ROI.
    fn noise(&mut self, standard_deviation: f64);

    /// If this is a 32-bit or signed 16-bit image, performs an absolute value transform, otherwise does nothing.
    fn abs(&mut self);

    /// Performs a exponential transform on the image or ROI.
    fn exp(&mut self);

    /// Performs a square root transform on the image or ROI.
    fn sqrt(&mut self);

    /// Does a natural logarithmic (base e) transform of the image or ROI.
    fn ln(&mut self);

    /// Does a logarithmic (base 10) transform of the image or ROI.
    fn log(&mut self);

    /// Performs gamma correction of the image or ROI.
    fn gamma(&mut self, value: T);

    // Private functions
    // Only for internal use
    fn add_func(v: T, scalar: f32) -> T {
        <T as PixelType>::clamp_pixel(v.to_f32() + scalar)
    }
    fn sub_func(v: T, scalar: f32) -> T {
        <T as PixelType>::clamp_pixel(v.to_f32() - scalar)
    }
    fn mul_func(v: T, scalar: f32) -> T {
        <T as PixelType>::clamp_pixel(v.to_f32() * scalar)
    }
    fn div_func(v: T, scalar: f32) -> T {
        <T as PixelType>::clamp_pixel(v.to_f32() / scalar)
    }
    fn ceil_func(v: T, scalar: f32) -> T {
        // TODO
        <T as PixelType>::clamp_pixel(v.to_f32() + scalar)
    }
    fn floor_func(v: T, scalar: f32) -> T {
        // TODO
        <T as PixelType>::clamp_pixel(v.to_f32() + scalar)
    }
    fn and_func(v: T, scalar: f32) -> T {
        // TODO
        <T as PixelType>::clamp_pixel(v.to_f32() + scalar)
    }
    fn or_func(v: T, scalar: f32) -> T {
        // TODO
        <T as PixelType>::clamp_pixel(v.to_f32() + scalar)
    }
    fn xor_func(v: T, scalar: f32) -> T {
        // TODO
        <T as PixelType>::clamp_pixel(v.to_f32() + scalar)
    }
}

/*
impl Operator<u8> for ImageProcessor<u8> {

    type Input = u8;

    fn add(&mut self, value: T){
        for i in self.get_data().iter_mut() {
            *i += value;
        }
    }
    fn substract(&mut self, value: T){
        for i in self.get_data().iter_mut() {
            if(value > *i){
                *i = 0;
            } else {
                *i -= value;
            }

        }
    }
    fn multiply(&mut self, value: T){
        for i in self.get_data().iter_mut() {
            *i *= value;
        }
    }
    fn divide(&mut self, value: T){
        for i in self.get_data().iter_mut() {
            *i /= value;
        }
    }

    fn ceil(&mut self, value: T){
        for i in self.get_data().iter_mut() {
            if *i > value {
                *i = value;
            }
        }
    }
    fn floor(&mut self, value: T){
        for i in self.get_data().iter_mut() {
            if *i < value {
                *i = value;
            }
        }
    }

    fn and(&mut self,value: T){
        for i in self.get_data().iter_mut() {
            *i = *i & value;
        }
    }
    fn or(&mut self,value: T){
        for i in self.get_data().iter_mut() {
            *i = *i | value;
        }
    }
    fn xor(&mut self,value: T){
        for i in self.get_data().iter_mut() {
            *i = *i ^ value;
        }
    }

    fn noise(&mut self,standard_deviation: f64){
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
    fn abs(&mut self){}
    fn exp(&mut self){
        for i in self.get_data().iter_mut() {
            *i = f64::powf(std::f64::consts::E, (*i).into()) as u8;
        }
    }
    fn sqrt(&mut self){
        for i in self.get_data().iter_mut() {
            *i = f32::powf((*i).into(),0.5 ) as u8;
        }
    }
    fn ln(&mut self){
        for i in self.get_data().iter_mut() {
            *i = (*i as f32).ln() as u8;
        }
    }
    fn log(&mut self){
        for i in self.get_data().iter_mut() {
            *i = (*i as f32).log(10.0) as u8;
        }
    }
    fn gamma(&mut self, value: f64){
        let max = self.get_max_possible();
        for i in self.get_data().iter_mut() {
            *i = (f64::powf((*i/max).into(), value) * (max as f64)) as u8 ;
        }
    }
}

impl Operator<f32> for ImageProcessor<f32> {

    type Input = f32;

    fn add(&mut self, value: T){
        for i in self.get_data().iter_mut() {
            *i += value;
        }
    }
    fn substract(&mut self, value: T){
        for i in self.get_data().iter_mut() {
            *i -= value;

        }
    }
    fn multiply(&mut self, value: T){
        for i in self.get_data().iter_mut() {
            *i *= value;
        }
    }
    fn divide(&mut self, value: T){
        for i in self.get_data().iter_mut() {
            *i /= value;
        }
    }

    fn ceil(&mut self, value: T){
        for i in self.get_data().iter_mut() {
            if *i > value {
                *i = value;
            }
        }
    }
    fn floor(&mut self, value: T){
        for i in self.get_data().iter_mut() {
            if *i < value {
                *i = value;
            }
        }
    }

    fn and(&mut self,value: T){
        !panic!("Cannot do bit operations on float processors !");
    }
    fn or(&mut self,value: T){
        !panic!("Cannot do bit operations on float processors !");
    }
    fn xor(&mut self,value: T){
        !panic!("Cannot do bit operations on float processors !");
    }


    fn noise(&mut self,standard_deviation: f64){
        let normal = Normal::new(0.0, standard_deviation);
        for i in self.get_data().iter_mut() {
            let value = normal.ind_sample(&mut rand::thread_rng());
            *i += (value as f32);

        }

    }
    fn abs(&mut self){
        for i in self.get_data().iter_mut() {
            *i = *i * -1.0;
        }
    }
    fn exp(&mut self){
        for i in self.get_data().iter_mut() {
            *i = f64::powf(std::f64::consts::E, (*i).into()) as f32;
        }
    }
    fn sqrt(&mut self){
        for i in self.get_data().iter_mut() {
            *i = f32::powf((*i).into(),0.5 ) as f32;
        }
    }
    fn ln(&mut self){
        for i in self.get_data().iter_mut() {
            *i = (*i as f32).ln() as f32;
        }
    }
    fn log(&mut self){
        for i in self.get_data().iter_mut() {
            *i = (*i as f32).log(10.0) as f32;
        }
    }
    fn gamma(&mut self, value: f64){
        let max = self.get_max_possible();
        for i in self.get_data().iter_mut() {
            *i = (f64::powf((*i/max).into(), value) * (max as f64)) as f32;
        }
    }
}

impl Operator<u8> for ImageStack<u8>{

    type Input = u8;

    fn add(&mut self, value: T){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().add(value);
        }
    }
    fn substract(&mut self, value: T){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().substract(value);
        }
    }
    fn multiply(&mut self, value: T){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().multiply(value);
        }
    }
    fn divide(&mut self, value: T){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().divide(value);
        }
    }

    fn ceil(&mut self, value: T){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().ceil(value);
        }
    }
    fn floor(&mut self, value: T){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().floor(value);
        }
    }

    fn and(&mut self, value: T){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().and(value);
        }
    }
    fn or(&mut self, value: T){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().or(value);
        }
    }
    fn xor(&mut self, value: T){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().xor(value);
        }
    }

    fn noise(&mut self,standard_deviation: f64){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().noise(standard_deviation);
        }
    }

    fn abs(&mut self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().abs();
        }
    }

    fn exp(&mut self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().exp();
        }
    }

    fn sqrt(&mut self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().sqrt();
        }
    }

    fn ln(&mut self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().ln();
        }
    }

    fn log(&mut self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().log();
        }
    }

    fn gamma(&mut self, value: f64){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().gamma(value);
        }
    }

}

impl Operator<f32> for ImageStack<f32>{

    type Input = f32;

    fn add(&mut self, value: T){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().add(value);
        }
    }
    fn substract(&mut self, value: T){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().substract(value);
        }
    }
    fn multiply(&mut self, value: T){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().multiply(value);
        }
    }
    fn divide(&mut self, value: T){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().divide(value);
        }
    }

    fn ceil(&mut self, value: T){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().ceil(value);
        }
    }
    fn floor(&mut self, value: T){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().floor(value);
        }
    }

    fn and(&mut self, value: T){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().and(value);
        }
    }
    fn or(&mut self, value: T){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().or(value);
        }
    }
    fn xor(&mut self, value: T){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().xor(value);
        }
    }

    fn noise(&mut self,standard_deviation: f64){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().noise(standard_deviation);
        }
    }

    fn abs(&mut self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().abs();
        }
    }

    fn exp(&mut self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().exp();
        }
    }

    fn sqrt(&mut self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().sqrt();
        }
    }

    fn ln(&mut self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().ln();
        }
    }

    fn log(&mut self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().log();
        }
    }

    fn gamma(&mut self, value: f64){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().gamma(value);
        }
    }

}

*/
