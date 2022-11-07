//
//  RIM - Rust Image
//  Copyright (C) 2022  Jean-Christophe Taveau,
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
// Authors:  Nicolas Maurice, Bluwen Guidoux.

use crate::pixel::PixelType;

pub trait Statistics<T: PixelType> {
    type Output;
    type Output_f32;
    
    fn update_stats(&mut self);
    /// Returns the minimum value in the image or stack.
    ///
    /// For rgb, separate minimum for red, blue and green
    fn min_value(&self) -> f64;
    /// Returns the maximum value in the image or stack.
    ///
    /// For rgb, separate minimum for red, blue and green
    fn max_value(&self) -> f64;
    ///Returns the average value in the image or stack.
    ///
    ///For rgb, separate averages for red, blue and green
    fn mean(&self) -> f64;
    ///Returns the histogram of the values in the image or stack.
    ///
    ///Even for rgb, one histogram
    fn histogram(&self) -> &Vec<u32>;
    /// Returns the standard deviation of the values in the image or stack.
    ///
    /// For rgb, separate standard deviations for red, blue and green
    fn standard_deviation(&self) -> f64;
    
    /*

    */
}
