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
use crate::image_stack::*;
use crate::image_traits::Access;
use std::collections::HashMap;
use std::ops::Div;
use core::ops::Mul;
use std::mem;
use histogram::Histogram;
//use std::num::sqrt;
//use num::integer::sqrt;
use num_traits::Pow;



pub trait Stats {
    type Output;
///Returns the minimum value in the image or stack. For rgb, separate minimum for red, blue and green
fn get_min_value(&self) -> Self::Output;
///Returns the maximum value in the image or stack. For rgb, separate minimum for red, blue and green
fn get_max_value(&self) -> Self::Output;
///Returns the average value in the image or stack. For rgb, separate averages for red, blue and green
fn get_mean(&self) -> Self::Output;
///Returns the histogram of the values in the image or stack. Even for rgb, one histogram
fn get_histogram(&self) -> Histogram;
///Returns the standard deviation of the values in the image or stack. For rgb, separate standard deviations for red, blue and green
fn get_standard_deviation(&self) -> Self::Output;
    
}

impl Stats for ImageProcessor<u8> {
    type Output = u8;

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

    fn get_mean(&self) -> Self::Output {
        let size = (self.get_height() * self.get_width());
        let mut average = self.get(usize::try_from(0).unwrap())/(size as u8);
        for i in 1..size {
            average = average + (self.get(usize::try_from(i).unwrap())/(size as u8));
        }
        return average
    }

    fn get_histogram(&self) -> Histogram{
        let mut hist = Histogram::new();
        let limit = self.get_width()*self.get_height();
        let mut pixel = (self.get(usize::try_from(0).unwrap())) as u64;
        hist.increment(pixel);
        for i in 1..limit{
            pixel = (self.get(usize::try_from(i).unwrap())) as u64;
            hist.increment(pixel);
        }
        return hist
    }

    fn get_standard_deviation(&self) -> Self::Output{
        let size = (self.get_height() * self.get_width()) as f64;
        let sizes = (self.get_height() * self.get_width());
        let mean = self.get_mean() as f64;
        let mut var = (f64::powf(self.get(usize::try_from(0).unwrap()).into(),2.0)-mean)/size;
        for i in 1..sizes {
            var = var + (f64::powf(self.get(usize::try_from(i).unwrap()).into(),2.0)-mean)/size;
        }
        let std = f64::powf(var as f64,0.5);
        return std as u8
    }

}

impl Stats for ImageProcessor<f32> {
    type Output = f32;

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

    fn get_mean(&self) -> Self::Output {
        let size = (self.get_height() * self.get_width());
        let mut average = self.get(usize::try_from(0).unwrap())/(size as f32);
        for i in 1..size {
            average = average + (self.get(usize::try_from(i).unwrap())/(size as f32));
        }
        return average
    }

    fn get_histogram(&self) -> Histogram{
        let mut hist = Histogram::new();
        let limit = self.get_width()*self.get_height();
        let mut pixel = (self.get(usize::try_from(0).unwrap())) as u64;
        hist.increment(pixel);
        for i in 1..limit{
            pixel = (self.get(usize::try_from(i).unwrap())) as u64;
            hist.increment(pixel);
        }
        return hist
    }

    fn get_standard_deviation(&self) -> Self::Output{
        let size = (self.get_height() * self.get_width()) as f64;
        let sizes = (self.get_height() * self.get_width());
        let mut var = f32::pow(self.get(usize::try_from(0).unwrap()),2)/(size as f32);
        let mean = self.get_mean() as f64;
        let mut var = (f64::powf(self.get(usize::try_from(0).unwrap()).into(),2.0)-mean)/size;
        for i in 1..sizes {
            var = var + (f64::powf(self.get(usize::try_from(i).unwrap()).into(),2.0)-mean)/size;
        }
        let std = f64::powf(var as f64,0.5);
        return std as f32
    }
}

impl Stats for ImageProcessor<(u8,u8,u8)>{
    type Output = (u8,u8,u8);

    fn get_min_value(&self) -> Self::Output {
        let size = self.get_height() * self.get_width();
        let minimum = self.get(usize::try_from(0).unwrap());
        let mut r_min = minimum.0;
        let mut g_min = minimum.1;
        let mut b_min = minimum.2;
        for i in 1..size {
            let tmp = self.get(usize::try_from(i).unwrap());
            if tmp.0 < r_min {
                r_min = tmp.0;
            }
            if tmp.1 < g_min {
                g_min = tmp.1;
            }
            if tmp.2 < b_min {
                b_min = tmp.2;
            }
        }
        return (r_min,g_min,b_min)
    }

    fn get_max_value(&self) -> Self::Output {
        let size = self.get_height() * self.get_width();
        let maximum = self.get(usize::try_from(0).unwrap());
        let mut r_max = maximum.0;
        let mut g_max = maximum.1;
        let mut b_max = maximum.2;
        for i in 1..size {
            let tmp = self.get(usize::try_from(i).unwrap());
            if tmp.0 > r_max {
                r_max = tmp.0;
            }
            if tmp.1 > g_max {
                g_max = tmp.1;
            }
            if tmp.2 > b_max {
                b_max = tmp.2;
            }
        }
        return (r_max,g_max,b_max)
    }

    fn get_mean(&self) -> Self::Output {
        let size = (self.get_height() * self.get_width())as u8;
        let mut pixel_rgb = self.get(usize::try_from(0).unwrap());
        let mut r = pixel_rgb.0/size;
        let mut g = pixel_rgb.1/size;
        let mut b = pixel_rgb.2/size;
        for i in 1..size {
            pixel_rgb = self.get(usize::try_from(i).unwrap());
            r = r + pixel_rgb.0/size;
            g = g + pixel_rgb.1/size;
            b = b + pixel_rgb.2/size;
        }
        return (r,g,b);
    }
    
    fn get_histogram(&self) -> Histogram{
        let mut hist = Histogram::new();
        let limit = self.get_width()*self.get_height();
        let mut pixel = (self.get(usize::try_from(0).unwrap()));
        let mut r = pixel.0;
        let mut g = pixel.1;
        let mut b = pixel.2;
        hist.increment(r.into());
        hist.increment(g.into());
        hist.increment(b.into());
        for i in 1..limit{
            pixel = (self.get(usize::try_from(i).unwrap()));
            r = pixel.0;
            g = pixel.1;
            b = pixel.2;
            hist.increment(r.into());
            hist.increment(g.into());
            hist.increment(b.into());
        }
        return hist
    }

    fn get_standard_deviation(&self) -> Self::Output{
        let size = (self.get_height() * self.get_width()) as f64;
        let sizes = (self.get_height() * self.get_width());
        let mean = self.get_mean();
        let r_mean = mean.0 as f64;
        let g_mean = mean.1 as f64;
        let b_mean = mean.2 as f64;
        let mut pixel_rgb = self.get(usize::try_from(0).unwrap());
        let mut r_var = (f64::powf(pixel_rgb.0.into(),2.0)-r_mean)/size;
        let mut g_var = (f64::powf(pixel_rgb.1.into(),2.0)-g_mean)/size;
        let mut b_var = (f64::powf(pixel_rgb.2.into(),2.0)-b_mean)/size;
        for i in 1..sizes {
            pixel_rgb = self.get(usize::try_from(i).unwrap());
            r_var = r_var + (f64::powf(pixel_rgb.0.into(),2.0)-r_mean)/size;
            g_var = g_var + (f64::powf(pixel_rgb.1.into(),2.0)-g_mean)/size;
            b_var = b_var + (f64::powf(pixel_rgb.2.into(),2.0)-b_mean)/size;
        }
        let r_std = f64::powf(r_var as f64,0.5);
        let g_std = f64::powf(g_var as f64,0.5);
        let b_std = f64::powf(b_var as f64,0.5);
        return (r_std as u8,g_std as u8,b_std as u8)
    }

}


impl Stats for ImageStack<u8> {
    type Output = u8;

    fn get_min_value(&self) -> Self::Output {
        let tmp_slice = self.get_focus_slice();
        let mut minimum = 0;
        let size = self.get_size();
        for i in 0..size {
            self.set_slice_number(i);
            minimum += self.get_one_slice_copy().get_min_value();
        }
        self.set_slice_number(tmp_slice);
        return minimum
    }

    fn get_max_value(&self) -> Self::Output {
        let tmp_slice = self.get_focus_slice();
        let mut maximum = 0;
        let size = self.get_size();
        for i in 0..size {
            self.set_slice_number(i);
            maximum += self.get_one_slice_copy().get_max_value();
        }
        self.set_slice_number(tmp_slice);
        return maximum
    }

    fn get_mean(&self) -> Self::Output {
        let tmp_slice = self.get_focus_slice();
        let mut average = 0;
        let size = self.get_size();
        for i in 0..size {
            self.set_slice_number(i);
            average += self.get_one_slice_copy().get_mean() / (size as u8);
        }
        self.set_slice_number(tmp_slice);
        return average
    }

    fn get_histogram(&self) -> Histogram{
        let tmp_slice = self.get_focus_slice();
        let mut hist = Histogram::new();
        let size = self.get_size();
        let limit = self.get_width_stack()*self.get_height_stack();
        for i in 0..size {
            self.set_slice_number(i);
            let mut pixel = (self.get(usize::try_from(0).unwrap())) as u64;
            hist.increment(pixel);
            for j in 1..limit {
                pixel = (self.get(usize::try_from(j).unwrap())) as u64;
                hist.increment(pixel);
            }
        }
        self.set_slice_number(tmp_slice);
        return hist
    }

    fn get_standard_deviation(&self) -> Self::Output{
        let tmp_slice = self.get_focus_slice();
        let size = self.get_size();
        let limit = self.get_width_stack()*self.get_height_stack();
        let size_f64 = (self.get_height_stack() * self.get_width_stack()) as f64;
        let mut var:f64 = 0.0;
        let mean = self.get_mean() as f64;
        for i in 0..size{
            self.set_slice_number(i);
            var = (f64::powf(self.get(usize::try_from(0).unwrap()).into(),2.0)-mean)/size_f64;
            for i in 1..limit {
                var = var + (f64::powf(self.get(usize::try_from(i).unwrap()).into(),2.0)-mean)/size_f64;
            }
        }
        self.set_slice_number(tmp_slice);
        let std = f64::powf(var as f64,0.5);
        return std as u8
    }
}

impl Stats for ImageStack<f32> {
    type Output = f32;

    fn get_min_value(&self) -> Self::Output {
        let tmp_slice = self.get_focus_slice();
        let mut minimum = 0.0 as f32;
        let size = self.get_size();
        for i in 0..size {
            self.set_slice_number(i);
            minimum += self.get_one_slice_copy().get_min_value();
        }
        self.set_slice_number(tmp_slice);
        return minimum
    }

    fn get_max_value(&self) -> Self::Output {
        let tmp_slice = self.get_focus_slice();
        let mut maximum = 0 as f32;
        let size = self.get_size();
        for i in 0..size {
            self.set_slice_number(i);
            maximum += self.get_one_slice_copy().get_max_value();
        }
        self.set_slice_number(tmp_slice);
        return maximum
    }

    fn get_mean(&self) -> Self::Output {
        let tmp_slice = self.get_focus_slice();
        let mut average = 0 as f32;
        let size = self.get_size();
        for i in 0..size {
            self.set_slice_number(i);
            average += self.get_one_slice_copy().get_mean() / (size as f32);
        }
        self.set_slice_number(tmp_slice);
        return average
    }

    fn get_histogram(&self) -> Histogram{
        let tmp_slice = self.get_focus_slice();
        let mut hist = Histogram::new();
        let size = self.get_size();
        let limit = self.get_width_stack()*self.get_height_stack();
        for i in 0..size {
            self.set_slice_number(i);
            let mut pixel = (self.get(usize::try_from(0).unwrap())) as u64;
            hist.increment(pixel);
            for j in 1..limit {
                pixel = (self.get(usize::try_from(j).unwrap())) as u64;
                hist.increment(pixel);
            }
        }
        self.set_slice_number(tmp_slice);
        return hist
    }

    fn get_standard_deviation(&self) -> Self::Output{
        let tmp_slice = self.get_focus_slice();
        let size = self.get_size();
        let limit = self.get_width_stack()*self.get_height_stack();
        let size_f64 = (self.get_height_stack() * self.get_width_stack()) as f64;
        let mut var:f64 = 0.0;
        let mean = self.get_mean() as f64;
        for i in 0..size{
            self.set_slice_number(i);
            var = (f64::powf(self.get(usize::try_from(0).unwrap()).into(),2.0)-mean)/size_f64;
            for i in 1..limit {
                var = var + (f64::powf(self.get(usize::try_from(i).unwrap()).into(),2.0)-mean)/size_f64;
            }
        }
        self.set_slice_number(tmp_slice);
        let std = f64::powf(var as f64,0.5);
        return std as f32
    }    
}


impl Stats for ImageStack<(u8,u8,u8)> {
    type Output = (u8,u8,u8);

    fn get_min_value(&self) -> Self::Output {
        let tmp_slice = self.get_focus_slice();
        let mut r_min=0;
        let mut g_min=0;
        let mut b_min=0;
        let size = self.get_size();
        for i in 0..size {
            self.set_slice_number(i);
            r_min += self.get_one_slice_copy().get_min_value().0;
            g_min += self.get_one_slice_copy().get_min_value().1;
            b_min += self.get_one_slice_copy().get_min_value().2;
        }
        self.set_slice_number(tmp_slice);
        return (r_min,g_min,b_min)
    }

    fn get_max_value(&self) -> Self::Output {
        let tmp_slice = self.get_focus_slice();
        let mut r_max=0;
        let mut g_max=0;
        let mut b_max=0;
        let size = self.get_size();
        for i in 0..size {
            self.set_slice_number(i);
            r_max+= self.get_one_slice_copy().get_max_value().0;
            g_max+= self.get_one_slice_copy().get_max_value().1;
            b_max+= self.get_one_slice_copy().get_max_value().2;
        }
        self.set_slice_number(tmp_slice);
        return (r_max,g_max,b_max)
    }

    fn get_mean(&self) -> Self::Output {
        let tmp_slice = self.get_focus_slice();
        let mut average;
        let mut r_mean=0;
        let mut g_mean=0;
        let mut b_mean=0;
        let size = self.get_size();
        for i in 0..size {
            self.set_slice_number(i);
            average = self.get_one_slice_copy().get_mean();
            r_mean += average.0;
            g_mean += average.1;
            b_mean += average.2;
        }
        self.set_slice_number(tmp_slice);
        return (r_mean,g_mean,b_mean)
    }

    fn get_histogram(&self) -> Histogram{
        let tmp_slice = self.get_focus_slice();
        let mut hist = Histogram::new();
        let size = self.get_size();
        let limit = self.get_width_stack()*self.get_height_stack();
        for i in 0..size {
            self.set_slice_number(i);
            let mut pixel = self.get(usize::try_from(0).unwrap());
            let mut r = pixel.0;
            let mut g = pixel.1;
            let mut b = pixel.2;
            hist.increment(r.into());
            hist.increment(g.into());
            hist.increment(b.into());
            for j in 1..limit {
                pixel = self.get(usize::try_from(j).unwrap());
                r = pixel.0;
                g = pixel.1;
                b = pixel.2;
                hist.increment(r.into());
                hist.increment(g.into());
                hist.increment(b.into());
            }
        }
        self.set_slice_number(tmp_slice);
        return hist
    }

    fn get_standard_deviation(&self) -> Self::Output{
        let tmp_slice = self.get_focus_slice();
        let size = self.get_size();
        let limit = self.get_width_stack()*self.get_height_stack();
        let size_f64 = (self.get_height_stack() * self.get_width_stack()) as f64;
        //let mut var:f64 = 0.0;
        let mut r_var = 0.0 as f64; 
        let mut g_var = 0.0 as f64;
        let mut b_var = 0.0 as f64;
        let mean = self.get_mean();
        let r_mean = mean.0 as f64;
        let g_mean = mean.1 as f64;
        let b_mean = mean.2 as f64;

        for i in 0..size{
            self.set_slice_number(i);
            let mut pixel_rgb = self.get(usize::try_from(0).unwrap());
            //var = (f64::powf(self.get(usize::try_from(0).unwrap()).into(),2.0)-mean)/size_f64;
            r_var = (f64::powf(pixel_rgb.0.into(),2.0)-r_mean)/size_f64;
            g_var = (f64::powf(pixel_rgb.1.into(),2.0)-g_mean)/size_f64;
            b_var = (f64::powf(pixel_rgb.2.into(),2.0)-b_mean)/size_f64;
            for i in 1..limit {
                pixel_rgb = self.get(usize::try_from(i).unwrap());
                r_var = r_var + (f64::powf(pixel_rgb.0.into(),2.0)-r_mean)/size_f64;
                g_var = g_var + (f64::powf(pixel_rgb.1.into(),2.0)-g_mean)/size_f64;
                b_var = b_var + (f64::powf(pixel_rgb.2.into(),2.0)-b_mean)/size_f64;
            }
        }
        self.set_slice_number(tmp_slice);
        let r_std = f64::powf(r_var as f64,0.5);
        let g_std = f64::powf(g_var as f64,0.5);
        let b_std = f64::powf(b_var as f64,0.5);
        return (r_std as u8,g_std as u8,b_std as u8)
    }    
}


#[cfg(test)]
mod test{
    use crate :: image_processor::*;
    use crate :: image_stack::*;
    use crate :: image_traits::*;
    use crate::stats::Stats;
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
    
    #[test]
    fn test_ImageProcessor_get_min_value_color(){
        let img =ColorProcessor::create_color_processor(10,20);
        assert_eq!(img.get_min_value(),(0,0,0));
    }
    
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
    
    #[test]
    fn test_ImageProcessor_get_max_value_color(){
        let mut img =ColorProcessor::create_color_processor(2,2);
        img.set_row(0,0,vec![(255,60,2)]);
        assert_eq!(img.get_max_value(),(255,60,2));
    }
    

    #[test]
    fn test_ImageProcessor_get_mean_byte(){
        let mut img =ImageProcessor::<u8>::create_byte_processor(2, 2);
        img.set_row(0,0,vec![255,130]);
        assert_eq!(img.get_mean(),95);
    }

    #[test]
    fn test_ImageProcessor_get_mean_float(){
        let mut img =FloatProcessor::create_float_processor(2,2);
        img.set_row(0,0,vec![255.0,130.0]);
        assert_eq!(img.get_mean(),96.25);
    }

    #[test]
    fn test_ImageProcessor_get_mean_rgb(){
        let mut img =ColorProcessor::create_color_processor(2,2);
        img.set_row(0,0,vec![(255,60,2)]);
        assert_eq!(img.get_mean(),(63, 15, 0));
    }

    #[test]
    fn test_ImageProcessor_get_histogram_byte(){
        let mut img =ImageProcessor::<u8>::create_byte_processor(2, 2);
        img.set_pixel(0,255);
        img.set_pixel(1,255);
        let hist = img.get_histogram();
        assert_eq!(hist.get(255).unwrap(),2);
    }

    #[test]
    fn test_ImageProcessor_get_histogram_float(){
        let mut img =FloatProcessor::create_float_processor(2,2);
        img.set_pixel(0,255.0);
        img.set_pixel(1,25.4);
        let hist = img.get_histogram();
        assert_eq!(hist.get(25).unwrap(),1);
    }

    #[test]
    fn test_ImageProcessor_get_histogram_rgb(){
        let mut img =ColorProcessor::create_color_processor(2,2);
        img.set_row(0,0,vec![(255,60,2)]);
        img.set_pixel(0,(255,5,60));
        img.set_pixel(1,(120,150,5));
        let hist = img.get_histogram();
        assert_eq!(hist.get(5).unwrap(),2);
    }

    #[test]
    fn test_ImageProcessor_get_standard_deviation_byte(){
        let mut img =ImageProcessor::<u8>::create_byte_processor(2, 2);
        img.set_pixel(0,255);
        img.set_pixel(1,255);
        assert_eq!(img.get_standard_deviation(),179);
    }

    #[test]
    fn test_ImageProcessor_get_standard_deviation_float(){
        let mut img =FloatProcessor::create_float_processor(2,2);
        img.set_pixel(0,255.0);
        img.set_pixel(1,25.4);
        assert_eq!(img.get_standard_deviation(),127.85711);
    }

    #[test]
    fn test_ImageProcessor_get_standard_deviation_rgb(){
        let mut img =ColorProcessor::create_color_processor(2,2);
        img.set_pixel(0,(120,30,0));
        img.set_pixel(1,(54,20,1));
        assert_eq!(img.get_standard_deviation(),(65, 17, 0));
    }

    #[test]
    fn test_ImageStack_get_min_value_byte(){
        let img =ImageStack::<u8>::create_byte_stack(10, 15, 1);
        assert_eq!(img.get_min_value(),0);
    }

    #[test]
    fn test_ImageStack_get_max_value_byte(){
        let mut img =ImageStack::<u8>::create_byte_stack(10, 15, 1);
        img.set_row(0,0,vec![255,130]);
        assert_eq!(img.get_max_value(),255);
    }

    #[test]
    fn test_ImageStack_get_mean_byte(){
        let mut img =ImageStack::<u8>::create_byte_stack(2, 2, 2);
        img.set_row(0,0,vec![255,130]);
        img.set_slice_number(1);
        img.set_row(0,0,vec![255,130]);
        assert_eq!(img.get_mean(),94);
    }

    #[test]
    fn test_ImageStack_get_histogram_byte(){
        let mut img =ImageStack::<u8>::create_byte_stack(2, 2, 2);
        img.set_pixel(1,255);
        img.set_pixel(0,255);
        img.set_slice_number(1);
        img.set_row(0,0,vec![255,130]);
        let hist = img.get_histogram();
        assert_eq!(hist.get(255).unwrap(),3);
    }

    #[test]
    fn test_ImageStack_get_standard_deviation_byte(){
        let mut img =ImageStack::<u8>::create_byte_stack(2, 2, 2);
        img.set_pixel(1,255);
        img.set_pixel(0,255);
        img.set_slice_number(1);
        img.set_row(0,0,vec![255,130]);
        assert_eq!(img.get_standard_deviation(),142);
    }

    #[test]
    fn test_ImageStack_get_min_value_float(){
        let img =ImageStack::<f32>::create_float_stack(10, 15, 1);
        assert_eq!(img.get_min_value(),0.0);
    }

    #[test]
    fn test_ImageStack_get_max_value_float(){
        let mut img =ImageStack::<f32>::create_float_stack(10, 15, 1);
        img.set_row(0,0,vec![255.0,130.78]);
        assert_eq!(img.get_max_value(),255.0);
    }

    #[test]
    fn test_ImageStack_get_mean_float(){
        let mut img =ImageStack::<f32>::create_float_stack(2, 2, 2);
        img.set_row(0,0,vec![255.0,130.5]);
        img.set_slice_number(1);
        img.set_row(0,0,vec![255.0,130.4]);
        assert_eq!(img.get_mean(),96.3625);
    }

    #[test]
    fn test_ImageStack_get_histogram_float(){
        let mut img =ImageStack::<f32>::create_float_stack(2, 2, 2);
        img.set_pixel(1,255.0);
        img.set_pixel(0,255.0);
        img.set_slice_number(1);
        img.set_row(0,0,vec![255.0,130.87]);
        let hist = img.get_histogram();
        assert_eq!(hist.get(255).unwrap(),3);
    }

    #[test]
    fn test_ImageStack_get_standard_deviation_float(){
        let mut img =ImageStack::<f32>::create_float_stack(2, 2, 2);
        img.set_pixel(1,255.0);
        img.set_pixel(0,255.0);
        img.set_slice_number(1);
        img.set_row(0,0,vec![255.0,130.87]);
        assert_eq!(img.get_standard_deviation(),142.91957);
    }

    #[test]
    fn test_ImageStack_get_min_value_rgb(){
        let img =ColorStack::create_color_stack(10,15,12);
        assert_eq!(img.get_min_value(),(0,0,0));
    }

    #[test]
    fn test_ImageStack_get_max_value_rgb(){
        let mut img =ColorStack::create_color_stack(10,15,12);
        img.set_row(0,0,vec![(255,130,239)]);
        assert_eq!(img.get_max_value(),(255,130,239));
    }

    #[test]
    fn test_ImageStack_get_mean_rgb(){
        let mut img =ColorStack::create_color_stack(2,2,2);
        img.set_row(0,0,vec![(255,130,239)]);
        img.set_slice_number(1);
        img.set_row(0,0,vec![(25,141,99)]);
        assert_eq!(img.get_mean(),(69, 67, 83));
    }

    #[test]
    fn test_ImageStack_get_histogram_rgb(){
        let mut img =ColorStack::create_color_stack(2,2,2);
        img.set_row(0,0,vec![(255,130,130)]);
        img.set_slice_number(1);
        img.set_row(0,0,vec![(25,130,99)]);
        let hist = img.get_histogram();
        assert_eq!(hist.get(130).unwrap(),3)
    }

    #[test]
    fn test_ImageStack_get_standard_deviation_rgb(){
        let mut img =ColorStack::create_color_stack(2,2,2);
        img.set_row(0,0,vec![(255,130,130)]);
        img.set_slice_number(1);
        img.set_row(0,0,vec![(25,130,99)]);
        assert_eq!(img.get_standard_deviation(),(9, 64, 48));
    }
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