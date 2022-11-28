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
//
// Authors: Luna Meyer, Louis Montagne 
 
 
 
/// VolumeProcessor is an ImageProcesssor only supporting 3D operations...



#![allow(non_camel_case_types)]
#![allow(unused)]

use crate::color_space::ColorSpace;
use crate::grayscale::Gray;
use crate::image_processor::ImageProcessor;
use crate::pixel::PixelType;

pub struct VolumeProcessor<T: PixelType, C: ColorSpace> {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub data: Vec<Vec<Vec<T>>>,
    pub cs: C,// metadata:MetaData
}

impl<T: PixelType, C: ColorSpace> VolumeProcessor<T, C> {
    //Constructor
    pub fn new(w: u32, h: u32, d: u32, pixel_volume: Vec<Vec<Vec<T>>>, cs: C) -> self{
        width: w,
        height: h,
        depth: d,
        data: pixel_volume,
        cs: cs,
    }
}

 // Accessors
 pub fn get_width(&self) -> u32 {
    self.width
}
pub fn get_height(&self) -> u32 {
    self.height
}
pub fn depth(&self) -> u32 {
    self.depth
}

pub fn data(&self) -> &Vec<Vec<T>> {
    &self.data
}
pub fn get_slice_size(&self) -> usize {
    (self.width * self.height) as usize
}

impl Transformable3D for VolumeProcessor {
   fn translate(tx:f32,ty: f32, tz: f32);

   fn rotate(rx: f32, ry:f32, rz: f32);
   fn apply_transform(mat: Vec<f32>);

}
