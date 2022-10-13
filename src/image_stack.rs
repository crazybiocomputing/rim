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


//size
//vecgteur d'image processor
//int
//w, h, cs, T avoir les infos sur le type d'image dans le stack


use crate :: image_processor::ImageProcessor;
use std::cell::RefCell;
use std::cell::RefMut;

pub struct ImageStack<T>{
    size: u32,
    data: RefCell<Vec<ImageProcessor<T>>>,
    focus_slice: RefCell<i32>,
}

impl<T> ImageStack<T>{
    
    pub fn create_stack(size: u32, data: RefCell<Vec<ImageProcessor<T>>>,focus_slice: RefCell<i32>) -> ImageStack<T>{
        return ImageStack{
            size : size,
            data: data,
            focus_slice: focus_slice
        }
    }

    pub fn create_byte_stack(width: u32, height: u32, size: u32) -> ImageStack<u8> {
        let data = RefCell::new(vec![ImageProcessor::<u8>::create_byte_processor(width,height);size.try_into().unwrap()]);
        let focus_slice = RefCell::new(1);
        return ImageStack::<u8>::create_stack(size,data,focus_slice)   
    }

    pub fn create_float_stack(width: u32, height: u32, size: u32) -> ImageStack<f32> {
        let data = RefCell::new(vec![ImageProcessor::<f32>::create_float_processor(width,height);size.try_into().unwrap()]);
        let focus_slice = RefCell::new(1);
        return ImageStack::<f32>::create_stack(size,data,focus_slice)   
    }

    pub fn create_color_stack(width: u32, height: u32, size: u32) -> ImageStack<u8> {
        let data = RefCell::new(vec![ImageProcessor::<(u8,u8,u8)>::create_byte_processor(width,height);size.try_into().unwrap()]);
        let focus_slice = RefCell::new(1);
        return ImageStack::<u8>::create_stack(size,data,focus_slice)   
    }
}

