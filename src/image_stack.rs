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
use crate::color_space::ColorSpace;
use std::cell::RefCell;
use std::cell::RefMut;
use std::cell::Cell;
use std::borrow::BorrowMut;


pub struct ImageStack<T>{
    width : u32,
    height : u32,
    size : Cell<u32>,
    data : RefCell<Vec<ImageProcessor<T>>>,
    cs : ColorSpace,
    focus_slice : Cell<u32>,
}

impl<T> ImageStack<T> where T: Clone {
    
    pub fn create_stack(width: u32, height: u32, size: Cell<u32>, data: RefCell<Vec<ImageProcessor<T>>>,cs : ColorSpace,focus_slice: Cell<u32>) -> ImageStack<T>{
        return ImageStack{
            width : width,
            height : height,
            size : size,
            data: data,
            cs : cs,
            focus_slice: focus_slice
        }
    }

    pub fn create_byte_stack(width: u32, height: u32, size: u32) -> ImageStack<u8> {
        let cs : ColorSpace = ColorSpace::Gray8();
        let data = RefCell::new(vec![ImageProcessor::<u8>::create_byte_processor(width,height);size.try_into().unwrap()]);
        let focus_slice = Cell::new(0);
        let size = Cell:: new(size);
        return ImageStack::<u8>::create_stack(width,height,size,data,cs,focus_slice)   
    }

    pub fn create_float_stack(width: u32, height: u32, size: u32) -> ImageStack<f32> {
        let cs : ColorSpace = ColorSpace::Grayf32();
        let data = RefCell::new(vec![ImageProcessor::<f32>::create_float_processor(width,height);size.try_into().unwrap()]);
        let focus_slice = Cell::new(0);
        let size = Cell:: new(size);
        return ImageStack::<f32>::create_stack(width,height,size,data,cs,focus_slice)   
    }

    pub fn create_color_stack(width: u32, height: u32, size: u32) -> ImageStack<u8> {
        let cs : ColorSpace = ColorSpace::Rgb24();
        let data = RefCell::new(vec![ImageProcessor::<(u8,u8,u8)>::create_byte_processor(width,height);size.try_into().unwrap()]);
        let focus_slice = Cell::new(0);
        let size = Cell:: new(size);
        return ImageStack::<u8>::create_stack(width,height,size,data,cs,focus_slice)   
    }

///Debug///
pub fn debug_stack(&self){
    println!("ImageStacks : Dimensions : {}x{}x{} px, Bit depth : {}, data length : {}", self.get_width_stack(), self.get_height_stack(), self.get_size(), self.get_bit_depth_stack(), self.get_data_stack().len());
    println!("Focus slice : {}", self.get_focus_slice());
}

///Get///

    pub fn get_size(&self) -> u32 {
        return self.size.get()
    }

    pub fn get_width_stack(&self) -> u32{
        return self.width
    }

    pub fn get_height_stack(&self) -> u32{
        return self.height
    }

    pub fn get_focus_slice(&self) -> u32{
        return self.focus_slice.get()
    }

    pub fn get_nb_channels_stacks(&self) -> u8{
        return self.cs.get_nb_channels()
    }

    pub fn get_bit_depth_stack(&self) -> u8{
        return self.cs.get_bit_depth()
    }

    //retourne le vecteur contenant les vecteurs d'images
    pub fn get_data_stack(&self) -> RefMut<Vec<ImageProcessor<T>>>{
        return self.data.borrow_mut()
    }

    pub fn get_one_slice(&self)-> ImageProcessor<T>{
        return self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].clone()
    }



///Set///

    pub fn set_size(&self){
        self.size.set(self.size.get()+1);
    }

    pub fn set_data_stack(&self,img: ImageProcessor<T>){
        if img.get_width() != self.get_width_stack() {
            panic!("Width out of bounds ! width stack={}, width image={}",self.get_width_stack(),img.get_width());
        }    
        if img.get_height() != self.get_height_stack(){
            panic!("Heigth out of bounds ! height stack={}, height image={}",self.get_height_stack(),img.get_height());
        }
        self.get_data_stack().push(img);
        self.set_size();
    }

    pub fn set_slice_number(&self,slice:u32) {
        self.focus_slice.set(slice);
        println!("Focus slice : {}", self.get_focus_slice());
        self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].debug();
    }
}

