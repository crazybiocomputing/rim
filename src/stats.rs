//
//  RIM - Rust Image
//  Copyright (C) 2022  Jean-Christophe Taveau, Nicolas Maurice, Bluwen Guidoux.
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


use crate::image_processor::*;
use crate::image_traits::Access;
use std::collections::HashMap;
use std::ops::Div;
use core::ops::Mul;
use std::mem;

pub trait Stats {
    type Output;
    fn get_min_value(&self) -> Self::Output;
    fn get_max_value(&self) -> Self::Output;
    unsafe fn get_mean(&self) -> Self::Output;
    unsafe fn get_mean_f32(&self) -> Self::Output;
    //unsafe fn get_mean_rgb(&self) -> Self::Output;
    //fn get standard_deviation(&self) -> Self::Output;

    //fn get_histogram(&self) -> HashMap<Self::Output,usize>;
    // get histograms specified bins
    // get histograms autobins
    
    //get mean_value
    //get standard deviation
}


impl<T> Stats for ImageProcessor<T> where T:Copy 
                                    + std::cmp::PartialOrd 
                                    + std::ops::Add<Output=T> 
                                    + std::ops::Div<T> 
                                    + Div<Output = T>
                                    + std::fmt::Display
                                    + std::fmt::Debug{  
    type Output = T;
    
    /// Returns the minimum displayed value in the image
    fn get_min_value(&self) -> Self::Output {
        let size = self.get_height() * self.get_width();
        let mut minimum = self.get(usize::try_from(0).unwrap());
        for i in 1..size {
            let tmp = self.get(usize::try_from(i).unwrap());
            if tmp < minimum {
                minimum = tmp;
            }
        }
        return minimum
    }
    
    /// Returns the maximum displayed value in the image
    fn get_max_value(&self) -> Self::Output {
        let size = self.get_height() * self.get_width();
        let mut maximum = self.get(usize::try_from(0).unwrap());
        for i in 1..size {
            let tmp = self.get(usize::try_from(i).unwrap());
            if tmp > maximum {
                maximum = tmp;
            }
        }
        return maximum
    }

   unsafe fn get_mean(&self) -> Self::Output {
        let size = self.get_height() * self.get_width();
        let size_t= mem::transmute_copy::<u32, T>(&size);
        let mut average = self.get(usize::try_from(0).unwrap())/size_t;
        for i in 1..size {
            average = average + (self.get(usize::try_from(i).unwrap())/size_t);
        }
        return average
    }
    

    unsafe fn get_mean_f32(&self) -> Self::Output {
        let size = self.get_height() * self.get_width();
        let tmp = size as f32;
        let size_t= mem::transmute_copy::<f32, T>(&tmp);
        let mut average = self.get(usize::try_from(0).unwrap())/size_t;
        for i in 1..size {
            average = average + (self.get(usize::try_from(i).unwrap())/size_t);
        }
        return average
    }
 /*  
    unsafe fn get_mean_rgb(&self) -> Self::Output {
        let size = self.get_height() * self.get_width();
        //let tmp = size as f32;
        let size_t= mem::transmute_copy::<u32, T>(&size);
        let mut pixel_rgb = self.get(usize::try_from(0).unwrap());
        let mut r:u8 = pixel_rgb.0/size_t;
        let mut g:u8 = pixel_rgb.1/size_t;
        let mut b:u8 = pixel_rgb.2/size_t;
        //let mut average = self.get(usize::try_from(0).unwrap())/size_t;
        for i in 1..size {
            pixel_rgb = self.get(usize::try_from(i).unwrap());
            r = r + pixel_rgb.0/size_t;
            g = g + pixel_rgb.1/size_t;
            b = b + pixel_rgb.2/size_t;
            //average = average + (self.get(usize::try_from(i).unwrap())/size_t);
        }
        return (r,g,b);
    }*/

    
   /* fn get_histogram(&self) -> HashMap<Self::Output,usize>{
        let mut out : HashMap<Self::Output,usize> =HashMap::with_capacity(100);
        // Vecteur vide de taille (max-min),On le remplit lentement ?
        // Dictionnaire, augmente si valeur connue, crée sinon ?
        let limit = self.get_width()*self.get_height();
        let mut pixel = self.get(usize::try_from(0).unwrap());
        println!("pixel {:?}", pixel);
        //out.shrink_to(10);
        out.insert(1,1);
        println!("out {:?}", out);
        /*out.insert(pixel,1);
        for i in 1..limit {
            pixel = self.get(usize::try_from(i).unwrap());
            let mut values = 0;
            if out.contains_key(&pixel) { 
                values=out[&pixel];
            } else {
                values= 0; }
            out.insert(pixel, 1 + values);//if out.contains_key(&pixel) { out[&pixel] } else { 0 });
        }*/
        
        return out
    }*/
    
    
}

#[cfg(test)]
mod test{
    //use super ::super::ImageStack::{*};
    //use super :: super:: ImageProcessor::{*};
    use crate :: image_processor::*;
    use crate :: image_traits::*;
    use crate::stats::Stats;
    /*use crate::image_stack::ImageStack;
    use crate::image_stack::ImageProcessor;
    use crate::image_stack::ByteStack;
    use crate::image_stack::ByteProcessor;
    use crate::image_stack::FloatProcessor;
    use crate::image_stack::FloatStack;
    use crate::color_space::ColorSpace;
    use crate::image_stack::ColorStack;
    use crate::image_stack::ColorProcessor;*/
    use core::cell::RefCell;
    use core::cell::Cell;

    #[test]
    fn test_ImageProcessor_get_min_value_byte(){
        let img =ImageProcessor::<u8>::create_byte_processor(10, 15);
        assert_eq!(img.get_min_value(),0);
    }

    #[test]
    fn test_ImageProcessor_get_min_value_float(){
        let img =FloatProcessor::create_float_processor(10,20);
        assert_eq!(img.get_min_value(),0.0);
    }
    /*
    #[test]
    fn test_ImageProcessor_get_min_value_color(){
        let img =ColorProcessor::create_color_processor(10,20);
        assert_eq!(img.get_min_value(),(0,0,0));
    }
    */
    #[test]
    fn test_ImageProcessor_get_max_value_byte(){
        let mut img =ImageProcessor::<u8>::create_byte_processor(2, 2);
        img.set_row(0,0,vec![255,130]);
        assert_eq!(img.get_max_value(),255);
    }

    #[test]
    fn test_ImageProcessor_get_max_value_float(){
        let mut img =FloatProcessor::create_float_processor(2,2);
        img.set_row(0,0,vec![3.4028235e38,100.0]);
        assert_eq!(img.get_max_value(),3.4028235e38);
    }
    /*
    #[test]
    fn test_ImageProcessor_get_max_value_color(){
        let mut img =ColorProcessor::create_color_processor(2,2);
        img.set_row(0,0,vec![(255,60,2)]);
        assert_eq!(img.get_max_value(),255);
    }
    */

    #[test]
    fn test_ImageProcessor_get_mean_byte(){
        let mut img =ImageProcessor::<u8>::create_byte_processor(2, 2);
        img.set_row(0,0,vec![255,130]);
        assert_eq!(unsafe{img.get_mean()},95);
    }

    #[test]
    fn test_ImageProcessor_get_mean_float(){
        let mut img =FloatProcessor::create_float_processor(2,2);
        img.set_row(0,0,vec![255.0,130.0]);
        assert_eq!(unsafe{img.get_mean_f32()},96.25);
    }
/*
    #[test]
    fn test_ImageProcessor_get_mean_rgb(){
        let mut img =ColorProcessor::create_color_processor(2,2);
        img.set_row(0,0,vec![(255,60,2)]);
        assert_eq!(img.get_mean_rgb(),255);
    }*/
}
/*
    /// Returns the histogram of the image or ROI.
    fn i32[] get_histogram() {};

    /// Returns the histogram of the image or ROI, using the specified number of bins.
    i32[] get_histogram​(i32 n_Bins) {};

    /// Returns the maximum histogram value used for histograms of float images.
    f64 get_histogram_max() {};

    /// Returns the minimum histogram value used for histograms of float images.
    f64 get_histogram_min() {};

    /// Returns the number of float image histogram bins.
    i32 get_histogram_size() {};
    
    /// Returns the largest displayed pixel value.
    fn f64 get_max() {};
    
    /// Returns the smallest displayed pixel value.
    fn f64 get_min() {};

    /// Calculates and returns complete uncalibrated (raw) statistics for this image or ROI but it is up to 70 times slower than get_Stats().
    fn get_Statistics(&self)  -> ImageStatistics {};

    /// Calculates and returns uncalibrated statistics for this image or ROI, including histogram, area, mean, min and max, standard deviation, and mode.
    fn  get_Stats(&self) -> ImageStatistics {};

    /// Returns the maximum possible pixel value.
    f64 max_value() {};

    /// Returns the minimum possible pixel value.
    f64 min_value() {};

    fn set_histogram_range​(f64 hist_Min, f64 hist_Max) {};
    /// Set the range used for histograms of float images.

    fn set_histogram_size​(i32 size) {};
    /// Set the number of bins to be used for histograms of float images.

*/