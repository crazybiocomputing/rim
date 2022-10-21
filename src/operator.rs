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
    

}

impl Operator<u8> for ImageProcessor<u8> {

    type Input = u8;

    fn add(&self, value: Self::Input){
        for i in self.get_data().iter_mut() {
            *i += value;
        }
    }
    fn substract(&self, value: Self::Input){
        for i in self.get_data().iter_mut() {
            if(value > *i){
                *i = 0;
            } else {
                *i -= value;
            }
            
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

impl Operator<f32> for ImageProcessor<f32> {

    type Input = f32;

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
        !panic!("Cannot do bit operations on float processors !");
    }
    fn or(&self,value: Self::Input){
        !panic!("Cannot do bit operations on float processors !");
    }
    fn xor(&self,value: Self::Input){
        !panic!("Cannot do bit operations on float processors !");
    }

    
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

impl Operator<u8> for ImageStack<u8>{

    type Input = u8;

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
    }
    fn or(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().or(value);
        }
    }
    fn xor(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().xor(value);
        }
    }

    fn noise(&self,standard_deviation: f64){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().noise(standard_deviation);
        }
    }

    fn abs(&self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().abs();
        }
    }

    fn exp(&self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().exp();
        }
    }

    fn sqrt(&self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().sqrt();
        }
    }

    fn ln(&self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().ln();
        }
    }

    fn log(&self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().log();
        }
    }

    fn gamma(&self, value: f64){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().gamma(value);
        }
    }

}

impl Operator<f32> for ImageStack<f32>{

    type Input = f32;

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
    }
    fn or(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().or(value);
        }
    }
    fn xor(&self, value: Self::Input){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().xor(value);
        }
    }

    fn noise(&self,standard_deviation: f64){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().noise(standard_deviation);
        }
    }

    fn abs(&self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().abs();
        }
    }

    fn exp(&self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().exp();
        }
    }

    fn sqrt(&self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().sqrt();
        }
    }

    fn ln(&self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().ln();
        }
    }

    fn log(&self){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().log();
        }
    }

    fn gamma(&self, value: f64){
        for i in self.get_data_stack().iter_mut() {
            i.borrow_mut().gamma(value);
        }
    }

}


#[cfg(test)]
mod test {
    use crate::image_processor::*;
    use crate::image_stack::*;
    use crate::image_traits::Access;
    use crate::operator::Operator;
    

    #[test]
    fn test_add_byte(){
        let img = ByteProcessor::create_byte_processor(10,10);
        img.add(10);
        assert_eq!(img.get(3),10);
    }
    
    #[test]
    fn test_add_float(){
        let img = FloatProcessor::create_float_processor(10,10);
        img.add(4.0);
        assert_eq!(img.get(3),4.0);
    }

    #[test]
    fn test_substract_byte(){
        let img = ByteProcessor::create_byte_processor(10,10);
        img.add(20);
        img.substract(10);
        assert_eq!(img.get(3),10);
    }

    #[test]
    fn test_substract_underflow(){
        let img = ByteProcessor::create_byte_processor(10,10);
        img.substract(10);
        assert_eq!(img.get(3),0);
    }
    
    #[test]
    fn test_substract_float(){
        let img = FloatProcessor::create_float_processor(10,10);
        img.add(20.0);
        img.substract(4.0);
        assert_eq!(img.get(3),16.0);
    }

    #[test]
    fn test_add_substract_stack(){
        let img = ByteStack::create_byte_stack(10,10,10);
        img.add(20);
        img.substract(4);
        assert_eq!(img.get(3),16);
    }

    #[test]
    fn test_multiply_byte(){
        let img = ByteProcessor::create_byte_processor(10,10);
        img.add(10);
        img.multiply(2);
        assert_eq!(img.get(3),20);
    }
    
    #[test]
    fn test_multiply_float(){
        let img = FloatProcessor::create_float_processor(10,10);
        img.add(4.0);
        img.multiply(3.0);
        assert_eq!(img.get(3),12.0);
    }

    #[test]
    fn test_divide_byte(){
        let img = ByteProcessor::create_byte_processor(10,10);
        img.add(30);
        img.divide(10);
        assert_eq!(img.get(3),3);
    }
    
    #[test]
    fn test_divide_float(){
        let img = FloatProcessor::create_float_processor(10,10);
        img.add(20.0);
        img.divide(5.0);
        assert_eq!(img.get(3),4.0);
    }

    #[test]
    fn test_divide_stack(){
        let img = ByteStack::create_byte_stack(10,10,10);
        img.add(20);
        img.divide(2);
        assert_eq!(img.get(3),10);
    }

    #[test]
    fn test_multiply_stack(){
        let img = FloatStack::create_float_stack(10,10,10);
        img.add(20.0);
        img.multiply(3.0);
        assert_eq!(img.get(3),60.0);
    }

    #[test]
    fn test_floor_byte(){
        let img = ByteProcessor::create_byte_processor(10,10);
        img.floor(3);
        assert_eq!(img.get(3),3);
    }
    
    #[test]
    fn test_floor_float(){
        let img = FloatProcessor::create_float_processor(10,10);
        img.floor(4.0);
        assert_eq!(img.get(3),4.0);
    }

    #[test]
    fn test_floor_stack(){
        let img = ByteStack::create_byte_stack(10,10,10);
        img.add(20);
        img.floor(6);
        assert_eq!(img.get(3),20);
    }

    #[test]
    fn test_ceil_byte(){
        let img = ByteProcessor::create_byte_processor(10,10);
        img.add(20);
        img.ceil(12);
        assert_eq!(img.get(3),12);
    }
    
    #[test]
    fn test_ceil_float(){
        let img = FloatProcessor::create_float_processor(10,10);
        img.add(20.0);
        img.ceil(4.0);
        assert_eq!(img.get(3),4.0);
    }

    #[test]
    fn test_ceil_stack(){
        let img = ByteStack::create_byte_stack(10,10,10);
        img.add(20);
        img.ceil(6);
        assert_eq!(img.get(3),6);
    }

    #[test]
    #[ignore] //Aléatoire, échoue parfois, réussi parfois
    fn test_noise_byte(){
        let img = ByteProcessor::create_byte_processor(10,10);
        img.add(100);
        img.noise(10.0);
        assert_ne!(img.get(3),100);
    }

    #[test]
    #[ignore] //Aléatoire, échoue parfois, réussi parfois
    fn test_noise_float(){
        let img = FloatProcessor::create_float_processor(10,10);
        img.noise(2.0);
        assert_ne!(img.get(3),0.0);
    }

    #[test]
    #[ignore] //Aléatoire, échoue parfois, réussi parfois
    fn test_noise_stack(){
        let img = FloatStack::create_float_stack(10,10,10);
        img.noise(3.0);
        assert_ne!(img.get(3),0.0);
    }

    #[test]
    fn test_abs_byte(){
        let img = ByteProcessor::create_byte_processor(10,10);
        img.add(100);
        img.abs();
        assert_eq!(img.get(3),100);
    }

    #[test]
    fn test_abs_float(){
        let img = FloatProcessor::create_float_processor(10,10);
        img.add(100.0);
        img.abs();
        assert_eq!(img.get(3),-100.0);
    }

    #[test]
    fn test_abs_stack(){
        let img = FloatStack::create_float_stack(10,10,10);
        img.add(100.0);
        img.abs();
        assert_eq!(img.get(3),-100.0);
    }

    #[test]
    fn test_exp_byte(){
        let img = ByteProcessor::create_byte_processor(10,10);
        img.add(2);
        img.exp();
        assert_eq!(img.get(3),7);
    }

    #[test]
    fn test_exp_float(){
        let img = FloatProcessor::create_float_processor(10,10);
        img.add(3.0);
        img.exp();
        assert_eq!(img.get(3),20.085537);
    }

    #[test]
    fn test_exp_stack(){
        let img = FloatStack::create_float_stack(10,10,10);
        img.add(4.0);
        img.exp();
        assert_eq!(img.get(3),54.59815);
    }

    #[test]
    fn test_sqrt_byte(){
        let img = ByteProcessor::create_byte_processor(10,10);
        img.add(4);
        img.sqrt();
        assert_eq!(img.get(3),2);
    }

    #[test]
    fn test_sqrt_float(){
        let img = FloatProcessor::create_float_processor(10,10);
        img.add(9.0);
        img.sqrt();
        assert_eq!(img.get(3),3.0);
    }

    #[test]
    fn test_sqrt_stack(){
        let img = ByteStack::create_byte_stack(10,10,10);
        img.add(16);
        img.sqrt();
        assert_eq!(img.get(3),4);
    }

    #[test]
    fn test_ln_byte(){
        let img = ByteProcessor::create_byte_processor(10,10);
        img.add(35);
        img.ln();
        assert_eq!(img.get(3),3);
    }

    #[test]
    fn test_ln_float(){
        let img = FloatProcessor::create_float_processor(10,10);
        img.add(29.0);
        img.ln();
        assert_eq!(img.get(3),3.3672957);
    }

    #[test]
    fn test_ln_stack(){
        let img = ByteStack::create_byte_stack(10,10,10);
        img.add(99);
        img.ln();
        assert_eq!(img.get(3),4);
    }

    #[test]
    fn test_log_byte(){
        let img = ByteProcessor::create_byte_processor(10,10);
        img.add(35);
        img.log();
        assert_eq!(img.get(3),1);
    }

    #[test]
    fn test_log_float(){
        let img = FloatProcessor::create_float_processor(10,10);
        img.add(29.0);
        img.log();
        assert_eq!(img.get(3),1.4623979);
    }

    #[test]
    fn test_log_stack(){
        let img = ByteStack::create_byte_stack(10,10,10);
        img.add(200);
        img.log();
        assert_eq!(img.get(3),2);
    }



    //gamma

    
}