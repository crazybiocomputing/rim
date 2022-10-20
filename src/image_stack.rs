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


use crate :: image_processor::*;
use crate::color_space::ColorSpace;
use std::cell::RefCell;
use std::cell::RefMut;
use std::cell::Cell;
use std::borrow::BorrowMut;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct ImageStack<T>{
    width : u32,
    height : u32,
    size : Cell<u32>,
    data : RefCell<Vec<RefCell<ImageProcessor<T>>>>,
    cs : ColorSpace<T>,
    focus_slice : Cell<u32>,
}

pub type ByteStack = ImageStack<u8>;
pub type FloatStack = ImageStack<f32>;
pub type ColorStack = ImageStack<(u8,u8,u8)>;

impl<T> ImageStack<T> where T: Copy {
    
    pub fn create_stack(width: u32, height: u32, size: Cell<u32>, data: RefCell<Vec<RefCell<ImageProcessor<T>>>>,cs : ColorSpace<T>,focus_slice: Cell<u32>) -> ImageStack<T>{
        return ImageStack{
            width : width,
            height : height,
            size : size,
            data: data,
            cs : cs,
            focus_slice: focus_slice
        }
    }

    pub fn create_byte_stack(width: u32, height: u32, size: u32) -> ByteStack {
        let cs : ColorSpace<u8> = ColorSpace::<u8>::Gray8();
        let data = RefCell::new(vec![RefCell::new(ByteProcessor::create_byte_processor(width,height));size.try_into().unwrap()]);
        let focus_slice = Cell::new(0);
        let size = Cell:: new(size);
        return ImageStack::<u8>::create_stack(width,height,size,data,cs,focus_slice)   
    }

    pub fn create_float_stack(width: u32, height: u32, size: u32) -> FloatStack {
        let cs : ColorSpace<f32> = ColorSpace::<f32>::Grayf32();
        let data = RefCell::new(vec![RefCell::new(FloatProcessor::create_float_processor(width,height));size.try_into().unwrap()]);
        let focus_slice = Cell::new(0);
        let size = Cell:: new(size);
        return ImageStack::<f32>::create_stack(width,height,size,data,cs,focus_slice)   
    }

    pub fn create_color_stack(width: u32, height: u32, size: u32) -> ColorStack {
        let cs : ColorSpace<(u8,u8,u8)> = ColorSpace::<(u8,u8,u8)>::Rgb24();
        let data = RefCell::new(vec![RefCell::new(ColorProcessor::create_color_processor(width,height));size.try_into().unwrap()]);
        let focus_slice = Cell::new(0);
        let size = Cell:: new(size);
        return ImageStack::<(u8,u8,u8)>::create_stack(width,height,size,data,cs,focus_slice)   
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
    pub fn get_data_stack(&self) -> RefMut<Vec<RefCell<ImageProcessor<T>>>>{
        return self.data.borrow_mut()
    }
    /*
    pub fn get_one_slice(&self)-> RefMut<ImageProcessor<T>>{
        return self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut()
    }
    */

    pub fn get_one_slice_copy(&self)-> ImageProcessor<T>{
        return self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow().clone()
    }


///Set///

    pub fn set_size(&self){
        self.size.set(self.size.get()+1);
    }

    ///push data stack
    pub fn set_data_stack(&self,img: ImageProcessor<T>){
        if img.get_width() != self.get_width_stack() {
            panic!("Width out of bounds ! width stack={}, width image={}",self.get_width_stack(),img.get_width());
        }    
        if img.get_height() != self.get_height_stack(){
            panic!("Heigth out of bounds ! height stack={}, height image={}",self.get_height_stack(),img.get_height());
        }
        self.get_data_stack().push(RefCell::new(img));
        self.set_size();
    }
    //à modifier pour ne pas débasser le nombre d'image contenue
    pub fn set_slice_number(&self,slice:u32) {
        if (slice>=self.get_size()) & (self.get_size() !=0) {
            panic!("Slice out of bounds ! Slices range from 0 to {} and you asked for slice numbre {}", (self.get_size()-1),slice);
        } 
        self.focus_slice.set(slice);
        println!("Focus slice : {}", self.get_focus_slice());
        self.get_one_slice_copy().debug();
    }
}


#[cfg(test)]
mod test{
    //use super ::super::ImageStack::{*};
    //use super :: super:: ImageProcessor::{*};
    use crate::image_stack::ImageStack;
    use crate::image_stack::ImageProcessor;
    use crate::image_stack::ByteStack;
    use crate::image_stack::ByteProcessor;
    use crate::image_stack::FloatProcessor;
    use crate::image_stack::FloatStack;
    use crate::color_space::ColorSpace;
    use crate::image_stack::ColorStack;
    use crate::image_stack::ColorProcessor;
    use core::cell::RefCell;
    use core::cell::Cell;

    #[test]
    fn test_create_byte_stack(){
        let stack = ByteStack {width: 10, 
            height: 15, 
            size: Cell:: new(12),
            data: RefCell::new(vec![RefCell::new(ByteProcessor::create_byte_processor(10,15));12]),
            cs : ColorSpace::<u8>::Gray8(),
            focus_slice: Cell::new(0)
            };
        assert_eq!(ByteStack::create_byte_stack(10,15,12), stack);
    }

    #[test]
    fn test_create_float_stack(){
        let stack = FloatStack {width: 10, 
            height: 15, 
            size: Cell:: new(12),
            data: RefCell::new(vec![RefCell::new(FloatProcessor::create_float_processor(10,15));12]),
            cs : ColorSpace::<f32>::Grayf32(),
            focus_slice: Cell::new(0)
            };
        assert_eq!(FloatStack::create_float_stack(10,15,12), stack);
    }


    #[test]
    fn test_create_color_stack(){
        let stack = ColorStack {width: 10, 
            height: 15, 
            size: Cell:: new(12),
            data: RefCell::new(vec![RefCell::new(ColorProcessor::create_color_processor(10,15));12]),
            cs : ColorSpace::<(u8,u8,u8)>::Rgb24(),
            focus_slice: Cell::new(0)
            };
        assert_eq!(ColorStack::create_color_stack(10,15,12), stack);
    }

    #[test]
    fn test_get_size(){
        let stack = ByteStack {width: 10, 
            height: 15, 
            size: Cell:: new(12),
            data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
            cs : ColorSpace::<u8>::Gray8(),
            focus_slice: Cell::new(0)
            };
        assert_eq!(stack.get_size(),12);
    }

    #[test]
    fn test_get_width_stack(){
        let stack = ByteStack {width: 10, 
            height: 15, 
            size: Cell:: new(12),
            data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
            cs : ColorSpace::<u8>::Gray8(),
            focus_slice: Cell::new(0)
            };
        assert_eq!(stack.get_width_stack(),10);
    }

    #[test]
    fn test_get_height_stack(){
        let stack = ByteStack {width: 10, 
            height: 15, 
            size: Cell:: new(12),
            data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
            cs : ColorSpace::<u8>::Gray8(),
            focus_slice: Cell::new(0)
            };
        assert_eq!(stack.get_height_stack(),15);
    }

    #[test]
    //should_get_focus_slice_equal_to_0
    fn test_get_focus_slice(){
        let stack = ByteStack {width: 10, 
            height: 15, 
            size: Cell:: new(12),
            data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
            cs : ColorSpace::<u8>::Gray8(),
            focus_slice: Cell::new(0)
            };
        assert_eq!(stack.get_focus_slice(),0);
    }
    
    #[test]
    fn test_get_nb_channels_stacks(){
        let stack = ByteStack {width: 10, 
            height: 15, 
            size: Cell:: new(12),
            data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
            cs : ColorSpace::<u8>::Gray8(),
            focus_slice: Cell::new(0)
            };
        assert_eq!(stack.get_nb_channels_stacks(),1);
    }

    #[test]
    fn test_get_bit_depth_stack(){
        let stack = ByteStack {width: 10, 
            height: 15, 
            size: Cell:: new(12),
            data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
            cs : ColorSpace::<u8>::Gray8(),
            focus_slice: Cell::new(0)
            };
        assert_eq!(stack.get_bit_depth_stack(),8);
    }

    /*#[test]
    fn test_get_data_stack(){
        let stack = ByteStack {width: 10, 
            height: 15, 
            size: Cell:: new(12),
            data: RefCell::new(vec![ImageProcessor::<u8>::create_byte_processor(10,15);12]),
            cs : ColorSpace::Gray8(),
            focus_slice: Cell::new(0)
            };
        assert_eq!(stack.get_data_stack(),vec![ImageProcessor::<u8>::create_byte_processor(10,15);12]);
    }*/

    #[test]
    fn test_get_one_slice(){
        let stack = ByteStack {width: 10, 
            height: 15, 
            size: Cell:: new(12),
            data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
            cs : ColorSpace::<u8>::Gray8(),
            focus_slice: Cell::new(0)
            };
            let img =ImageProcessor::<u8>::create_byte_processor(10, 15);
        assert_eq!(stack.get_one_slice_copy(),img);
    }

    
    #[test]
    fn test_set_size(){
        let stack = ByteStack {width: 10, 
            height: 15, 
            size: Cell:: new(12),
            data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
            cs : ColorSpace::<u8>::Gray8(),
            focus_slice: Cell::new(0)
            };
        stack.set_size();
        assert_eq!(stack.get_size(),13);
    }

    #[test]
    fn test_set_data_stack(){
        let stack = ByteStack {width: 10, 
            height: 15, 
            size: Cell:: new(12),
            data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
            cs : ColorSpace::<u8>::Gray8(),
            focus_slice: Cell::new(0)
            };
        let img =ImageProcessor::<u8>::create_byte_processor(10, 15);
        stack.set_data_stack(img);
        assert_eq!(stack.get_size(),13);
    }


    #[test]
    #[should_panic(expected = "Width out of bounds ! width stack=10, width image=1")]
    fn test_set_data_stack_panic_width(){
        let stack = ByteStack {width: 10, 
            height: 15, 
            size: Cell:: new(12),
            data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
            cs : ColorSpace::<u8>::Gray8(),
            focus_slice: Cell::new(0)
            };
        let img =ImageProcessor::<u8>::create_byte_processor(1, 15);
        stack.set_data_stack(img);
    }

    #[test]
    #[should_panic(expected = "Heigth out of bounds ! height stack=15, height image=1")]
    fn test_set_data_stack_panic_heigth(){
        let stack = ByteStack {width: 10, 
            height: 15, 
            size: Cell:: new(12),
            data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
            cs : ColorSpace::<u8>::Gray8(),
            focus_slice: Cell::new(0)
            };
        let img =ImageProcessor::<u8>::create_byte_processor(10, 1);
        stack.set_data_stack(img);
    }

    #[test]
    fn test_set_slice_number(){
        let stack = ByteStack {width: 10, 
            height: 15, 
            size: Cell:: new(12),
            data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
            cs : ColorSpace::<u8>::Gray8(),
            focus_slice: Cell::new(0)
            };
        stack.set_slice_number(11);
        assert_eq!(stack.get_focus_slice(),11);
    }

    #[test]
    #[should_panic(expected = "Slice out of bounds ! Slices range from 0 to 11 and you asked for slice numbre 12")]
    fn test_set_slice_number_panic(){
        let stack = ByteStack {width: 10, 
            height: 15, 
            size: Cell:: new(12),
            data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
            cs : ColorSpace::<u8>::Gray8(),
            focus_slice: Cell::new(0)
            };
        stack.set_slice_number(12);
    }
}