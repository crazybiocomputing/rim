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
 
 
pub trait Transform {

    static f64 cubic​( x: f64) {}
    
    /// Flips the image or ROI horizontally.
    fn flip_horizontal(&self) {};

    /// Flips the image or ROI vertically.
    fn flip_vertical(&self) {};
    
    /// This method is from Chapter 16 of "Digital Image Processing: An Algorithmic introduction Using Java" by Burger and Burge 
    /// (http://www.imagingbook.com/).
    fn f64 get_bicubic_interpolated_pixel​(f64 x0, f64 y0, Image_Processor ip2) {}

    /// Returns the value of the interpolate field.
    fn bool get_interpolate() {};

    /// Uses the current i32erpolation method (bilinear or bicubic) to find the pixel value at real coordinates (x,y).
    fn f64 get_interpolated_pixel​(f64 x, f64 y) {};

    /// Uses the current interpolation method to find the pixel value at real coordinates (x,y).
    fn i32 get_Pixel_interpolated​(f64 x, f64 y) {}

    /// Uses bilinear i32erpolation to find the pixel value at real coordinates (x,y).
    fn f64 geti32erpolated_Value​(f64 x, f64 y) {};

    /// Returns the current i32erpolation method (NONE, BILINEAR or BICUBIC).
    fn i32 get_interpolation_method() {};

    static java.lang.String[] get_interpolation_methods() {};

    /// Returns a new Image_Processor containing a scaled copy of this image or ROI.
    /// Image_Processor resize​(i32 dst_Width, i32 dst_Height, bool use_Averging) {}
    /// Returns a new Image_Processor containing a scaled copy of this image or ROI.
    fn resize​(i32 dst_Width, i32 dst_Height) -> Image_Processor {};

    /// Use linear interpolation to resize images that have a width or height of one.
    fn Image_Processor resize_linearly​(i32 width2, i32 height2) {};

    /// Rotates the image or selection 'angle' degrees clockwise.
    /// Image_Processor rotate_Left() {}
    /// Rotates the entire image 90 degrees counter-clockwise.
    /// Image_Processor rotate_Right() {}
    /// Rotates the entire image 90 degrees clockwise.
    fn rotate​(&self,f64 angle) {};

    /// Scales the image by the specified factors.
    ///n scale_And_Set_Threshold​(f64 lower, f64 upper, i32 lut_Update) {}
    ///et the threshold using a 0-255 range.
    ///n set​(f64 value) {}
    /// Assigns 'value' to each pixel in the image or ROI.
    fn scale​(&self,f64 x_Scale, f64 y_Scale) {};

    fn set_interpolate​(bool interpolate) {};
    /// This method has been replaced by seti32erpolation_Method().

    fn set_interpolation_method​(i32 method) {};;
    /// Use this method to set the i32erpolation method (NONE, BILINEAR or BICUBIC) used by scale(), resize() and rotate().

    static fn set_use_bicubic​(bool b) {};

    /// Moves the image or selection vertically or horizontally by a specified number of pixels.
    fn translate​(&self,f64 x_offset, f64 y_offset) {};

}

pub trait Transform3D {
  fn translate(tx:f32,ty: f32, tz: f32) {};
  
  fn rotate(rx: f32, ry:f32, rz: f32) {};

  fn apply_transform(mat: Vec<f32>) {};


}
 
