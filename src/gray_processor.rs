//
//  RIM - Rust Image
//  Copyright (C) 2022  Jean-Christophe Taveau.
//
//  This file is part of RIM
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (&self,at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with RIM.  If not, see <http://www.gnu.org/licenses/>.

use crate::grayscale::Gray;
use crate::image_processor::ImageProcessor;
use crate::image_traits::Access;
use crate::operator::Operator;
use crate::pixel::PixelType;

///
/// Gray-Level Image Processor
///

//
// Accessors get/set pixel.s
//
impl<T: PixelType + std::clone::Clone> Access<T> for ImageProcessor<T, Gray<T>> {
    type Output = T;

    /// Check index bounds
    fn get_pixel(&self, index: usize) -> Result<Self::Output, &str> {
        match index {
            x if x < self.get_size() => Ok(self.data()[index].to_value()),
            _ => Err("Out of bounds"),
        }
    }
    fn get_pixel_at(&self, x: u32, y: u32) -> Result<Self::Output, &str> {
        Self::get_pixel(self, (x + self.get_width() * y) as usize)
    }

    fn get(&self, index: usize) -> Self::Output {
        // Should be faster without bounds checking
        self.data()[index].to_value()
    }
    fn getf(&self, index: usize) -> f32 {
        // Should be faster without bounds checking
        self.data()[index].to_f32()
    }

    // TODO must add check
    fn set_pixel(&mut self, index: usize, v: T) {
        self.data[index] = v;
    }

    // TODO must add check
    fn set_pixel_at(&mut self, x: u32, y: u32, v: T) {
        let w = self.get_width();
        self.data[(x + w * y) as usize] = v;
    }

    /// Set 1 pixel at a specific index, without check
    fn set(&mut self, index: usize, value: T) {
        self.data[index] = value;
    }

    /// Set 1 pixel at a specific index, without check
    fn set_at(&mut self, x: u32, y: u32, v: T) {
        let w = self.get_width();
        self.data[(x + w * y) as usize] = v;
    }

    /// Get a row of pixel, starting from a specific x y position
    fn get_row(&self, x: u32, y: u32) -> Vec<T> {
        // TODO
        let start = x + self.get_width() * y;
        let end = start + (self.get_width() - start);
        self.data[start as usize..end as usize].to_vec()
    }
    /// Get a column of pixel, starting from a specific x y position
    fn get_column(&self, x: u32, y: u32) -> Vec<T> {
        // TODO
        self.data()[0..2].to_vec()
    }

    /// Fill a row of pixel, starting from a specific x y position, with a vector of pixels.
    fn set_row(&mut self, x: u32, y: u32, data: Vec<T>) {
        // TODO
    }
    /// Fill a column of pixel, starting from a specific x y position, with a vector of pixels.
    fn set_column(&mut self, x: u32, y: u32, data: Vec<T>) {
        // TODO
    }
}

//
// Operators Implementation
//
impl<T: PixelType> Operator<T> for ImageProcessor<T, Gray<T>> {
    type Output = T;

    fn macro_op(&mut self, f: fn(T, f32, f32, f32, u32, u32, f32, f32) -> T) {
        self.data = self
            .data
            .iter()
            .enumerate()
            .map(|(i, v)| {
                // Compute v,x,y,z,w,h,a,d
                let x = (i as u32 % self.get_width()) as f32;
                let y = (i as f32 / self.get_width() as f32).floor();
                let z = 0f32;
                let a = y.atan2(x); // In radians
                let cx = (self.get_width() as f32 / 2.0).floor();
                let cy = (self.get_height() as f32 / 2.0).floor();
                let d = ((x - cx) * (x - cx) + (y - cy) * (y - cy)).sqrt();
                // Call f(..)
                f(
                    v.to_value(),
                    x,
                    y,
                    z,
                    self.get_width(),
                    self.get_height(),
                    a,
                    d,
                )
            })
            .collect();
    }
    fn macro_scalar(&mut self, scalar: T, f: fn(T, f32) -> T) {
        self.data = self
            .data
            .iter()
            .map(|v| {
                // Call f(..)
                f(v.to_value(), scalar.to_f32())
            })
            .collect();
    }

    fn add(&mut self, scalar: T) {
        self.macro_scalar(
            scalar,
            <ImageProcessor<T, Gray<T>> as Operator<T>>::add_func,
        );
    }

    fn subtract(&mut self, scalar: T) {
        self.macro_scalar(
            scalar,
            <ImageProcessor<T, Gray<T>> as Operator<T>>::sub_func,
        );
    }

    fn multiply(&mut self, scalar: T) {
        self.macro_scalar(
            scalar,
            <ImageProcessor<T, Gray<T>> as Operator<T>>::mul_func,
        );
    }

    fn divide(&mut self, scalar: T) {
        self.macro_scalar(
            scalar,
            <ImageProcessor<T, Gray<T>> as Operator<T>>::div_func,
        );
    }

    fn ceil(&mut self, scalar: T) {
        self.data = self
            .data
            .iter()
            .map(|v| {
                if v.to_f32() > scalar.to_f32() {
                    scalar.to_value()
                } else {
                    v.to_value()
                }
            })
            .collect();
    }

    fn floor(&mut self, scalar: T) {
        self.data = self
            .data
            .iter()
            .map(|v| {
                if v.to_f32() < scalar.to_f32() {
                    scalar.to_value()
                } else {
                    v.to_value()
                }
            })
            .collect();
    }

    fn and(&mut self, scalar: T) {
        self.macro_scalar(
            scalar,
            <ImageProcessor<T, Gray<T>> as Operator<T>>::and_func,
        );
    }

    fn or(&mut self, scalar: T) {
        self.macro_scalar(scalar, <ImageProcessor<T, Gray<T>> as Operator<T>>::or_func);
    }

    fn xor(&mut self, scalar: T) {
        self.macro_scalar(
            scalar,
            <ImageProcessor<T, Gray<T>> as Operator<T>>::xor_func,
        );
    }

    fn noise(&mut self, standard_deviation: f64) {
        // self.macro_scalar(scalar, <ImageProcessor<T, Gray<T>> as Operator<T>>::noise_func);
        // TODO
    }

    fn abs(&mut self) {
        self.data = self
            .data
            .iter()
            .map(|v| <T as PixelType>::clamp_pixel(v.to_f32().abs()))
            .collect();
    }

    fn exp(&mut self) {
        self.data = self
            .data
            .iter()
            .map(|v| <T as PixelType>::clamp_pixel(v.to_f32().exp()))
            .collect();
    }

    fn sqrt(&mut self) {
        self.data = self
            .data
            .iter()
            .map(|v| <T as PixelType>::clamp_pixel(v.to_f32().sqrt()))
            .collect();
    }

    fn ln(&mut self) {
        self.data = self
            .data
            .iter()
            .map(|v| <T as PixelType>::clamp_pixel(v.to_f32().log(2.0)))
            .collect();
    }

    fn log(&mut self) {
        self.data = self
            .data
            .iter()
            .map(|v| <T as PixelType>::clamp_pixel(v.to_f32().log(10.0)))
            .collect();
    }

    /// Performs gamma correction of the image or ROI.
    fn gamma(&mut self, scalar: T) {
        //self.macro_scalar(scalar, <ImageProcessor<T, Gray<T>> as Operator<T>>::gamma_func);
    }
}
