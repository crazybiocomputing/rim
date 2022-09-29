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


pub trait Operator {
    /// Adds 'value' to each pixel in the image or ROI.
    fn add​(&self, value: i32);
    
    /// Adds 'value' to each pixel in the image or ROI.
    fn add​_float(&self, value: f64);

    /// Binary AND of each pixel in the image or ROI with 'value'.
    fn and​(&self,value: i32);

    /// If this is a 32-bit or signed 16-bit image, performs an absolute value transform, otherwise does nothing.
    fn abs(&self);

    /// Performs a exponential transform on the image or ROI.
    fn exp(&self);

    /// Performs gamma correction of the image or ROI.
    fn gamma​(value: f64);

    /// Does a natural logarithmic (base e) transform of the image or ROI.
    fn ln(&self);

    /// Does a logarithmic (base 10) transform of the image or ROI.
    fn log(&self);
 
    /// Pixels greater than 'value' are set to 'value'.
    fn max​(&self, value: f64);

    /// Pixels less than 'value' are set to 'value'.
    fn min​(&self,value: f64);

    /// Multiplies each pixel in the image or ROI by 'value'.
    fn multiply​(&self,value: f64);

    /// Adds pseudorandom, Gaussian ("normally") distributed values, 
    /// with mean 0.0 and the specified standard deviation, to this image or ROI.
    fn noise​(&self,standard_deviation: f64);

    /// Binary OR of each pixel in the image or ROI with 'value'.
    fn or​(&self,value: i32);

    static fn set_random_seed​(random_seed: f64);

    /// Performs a square transform on the image or ROI.
    fn sqr(&self);

    /// Performs a square root transform on the image or ROI.
    fn sqrt(&self);

    fn subtract​(&self,value: f64);
    /// Subtracts 'value' from each pixel in the image or ROI.

    /// Binary exclusive OR of each pixel in the image or ROI with 'value'.
    fn xor​(&self,value: i32);

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

}
