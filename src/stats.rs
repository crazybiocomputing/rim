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
use histogram::Histogram;

pub trait Stats {
    type Output;
    fn get_min_value(&self) -> Self::Output;
    fn get_max_value(&self) -> Self::Output;
    fn get_mean(&self) -> Self::Output;
    fn get_histogram(&self) -> Histogram;
    //fn get standard_deviation(&self) -> Self::Output;

    //unsafe fn get_histogram(&self) -> Histogram;//<Self::Output>;//where <Self as Stats>::Output: Counter;
    // get histograms specified bins
    // get histograms autobins
    
    //get mean_value
    //get standard deviation
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
        let size = (self.get_height() * self.get_width()) as u8;
        let mut average = self.get(usize::try_from(0).unwrap())/size;
        for i in 1..size {
            average = average + (self.get(usize::try_from(i).unwrap())/size);
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

}


#[cfg(test)]
mod test{
    use crate :: image_processor::*;
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