//
//  RIM - Rust Image
//  Copyright (&self,C) 2022  Jean-Christophe Taveau, Nicolas Maurice, Bluwen Guidoux.
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
use std::cmp::min;

use crate::image_processor::*;
use crate::image_stack::*;

pub trait Access<T> {
    type Output;
    /// Get 1 pixel at a specific index
    fn get_pixel(&self, index: u32) -> Self::Output;
    /// Get 1 pixel at a specific x y position
    fn get_pixel_at(&self,x: u32, y: u32) -> Self::Output;
    /// Get 1 pixel at a specific index, without check
    fn get(&self,index: usize) -> Self::Output;

    /// Set 1 pixel at a specific index
    fn set_pixel(&mut self,index: u32, value: Self::Output);
    /// Set 1 pixel at a specific x y position
    fn set_pixel_at(&mut self,x: u32, y: u32, value: Self::Output);
    /// Set 1 pixel at a specific index, without check
    fn set(&mut self,index: u32, value: Self::Output);  

    /// Get a row of pixel, starting from a specific x y position
    fn get_row(&self,x: u32, y: u32) -> Vec<Self::Output>;
    /// Get a column of pixel, starting from a specific x y position
    fn get_column(&self,x: u32, y: u32) -> Vec<Self::Output>;

    /// Fill a row of pixel, starting from a specific x y position, with a vector of pixels. 
    fn set_row(&mut self,x: u32, y: u32, data: Vec<Self::Output>);
    /// Fill a column of pixel, starting from a specific x y position, with a vector of pixels. 
    fn set_column(&mut self,x: u32, y: u32, data: Vec<Self::Output>);

    /// Set the slice number of a stack 
    fn set_slice_number(&self,slice: u32);
}



impl<T> Access<T> for ImageProcessor<T> where T:Copy{
    type Output = T;

    ///// Get 1 pixel /////
    fn get_pixel(&self, index: u32) -> Self::Output{
        if u32::from(index) >= self.get_width()*self.get_height(){
            panic!("Pixel out of bounds ! index = {}, data length : {}",index ,self.get_width()*self.get_height());
        } 
        return self.get_data()[usize::try_from(index).unwrap()]; 
    }
    
    fn get_pixel_at(&self,x: u32, y: u32) -> Self::Output{
        if x >= self.get_width(){
            panic!("Pixel out of bounds ! x={}, width={}",x,self.get_width());
        }
        if y >= self.get_height(){
            panic!("Pixel out of bounds ! y={}, height={}",y,self.get_height());
        }
        return self.get_pixel(y*self.get_width()+x)
    }
    // No check, faster, but prone to errors
    fn get(&self,index: usize) -> Self::Output{
        return self.get_data()[index];
    }

    
    ///// set 1 Pixel /////
    fn set_pixel(&mut self,index: u32, value: Self::Output){
        if u32::from(index) >= self.get_width()*self.get_height(){
            panic!("Pixel out of bounds ! index = {}, data length : {}",index ,self.get_width()*self.get_height());
        }
        
        self.get_data()[usize::try_from(index).unwrap()] = value;
    }
    fn set_pixel_at(&mut self,x: u32, y: u32, value: Self::Output){
        if x >= self.get_width(){
            panic!("Pixel out of bounds ! x={}, width={}",x,self.get_width());
        }
        if y >= self.get_height(){
            panic!("Pixel out of bounds ! y={}, height={}",y,self.get_height());
        }
        self.set_pixel(y*self.get_width()+x,value);
    }
    // No check, faster, but prone to errors
    fn set(&mut self,index: u32, value: Self::Output){
        self.get_data()[usize::try_from(index).unwrap()] = value;
    }
    
    fn get_row(&self,x: u32, y: u32) -> Vec<Self::Output>{
        let mut out : Vec<Self::Output> = Vec::new();
        let width = self.get_width();
        for local_x in x..width{
            out.push(self.get(usize::try_from(y*width+local_x).unwrap()));
        }
        return out;
    } 
    fn get_column(&self,x: u32, y: u32) -> Vec<Self::Output>{
        let mut out : Vec<Self::Output> = Vec::new();
        let width = self.get_width();
        let height = self.get_height();
        for local_y in y..height{
            out.push(self.get(usize::try_from(local_y*width+x).unwrap()));
        }
        return out;
    }

    fn set_row(&mut self,x: u32, y: u32, data: Vec<Self::Output>){
        let width = self.get_width();
        for local_x in x..min(width, y+u32::try_from(data.len()).unwrap()){
            self.set(u32::try_from(y*width+local_x).unwrap(), data[usize::try_from(local_x-x).unwrap()]);
        }
    }
    fn set_column(&mut self,x: u32, y: u32, data: Vec<Self::Output>){
        let width = self.get_width();
        let height = self.get_height();
        for local_y in y..min(height, y+u32::try_from(data.len()).unwrap()){
            self.set(u32::try_from(local_y*width+x).unwrap(), data[usize::try_from(local_y-y).unwrap()]);
        }
    }

    fn set_slice_number(&self,slice: u32){
        panic!("You cannot set the slice number for a single image");
    }
}



impl<T> Access<T> for ImageStack<T> where T:Copy{
    type Output = T;

    ///// Get 1 pixel /////
    fn get_pixel(&self, index: u32) -> Self::Output{
        return self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut().get_pixel(index)
    }
    
    fn get_pixel_at(&self,x: u32, y: u32) -> Self::Output{
        if x >= self.get_width_stack(){
            panic!("Pixel out of bounds ! x={}, width={}",x,self.get_width_stack());
        }
        if y >= self.get_height_stack(){
            panic!("Pixel out of bounds ! y={}, height={}",y,self.get_height_stack());
        }
        return self.get_pixel(y*self.get_width_stack()+x)
    }
    // No check, faster, but prone to errors
    fn get(&self,index: usize) -> Self::Output{
        return self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut().get_data()[index];
    }

    
    ///// set 1 Pixel /////
    fn set_pixel(&mut self,index: u32, value: Self::Output){
        if u32::from(index) >= self.get_width_stack()*self.get_height_stack(){
            panic!("Pixel out of bounds ! index = {}, data length : {}",index ,self.get_width_stack()*self.get_height_stack());
        }
        
        self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut().get_data()[usize::try_from(index).unwrap()] = value;
    }
    fn set_pixel_at(&mut self,x: u32, y: u32, value: Self::Output){
        if x >= self.get_width_stack(){
            panic!("Pixel out of bounds ! x={}, width={}",x,self.get_width_stack());
        }
        if y >= self.get_height_stack(){
            panic!("Pixel out of bounds ! y={}, height={}",y,self.get_height_stack());
        }
        self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut().set_pixel(y*self.get_width_stack()+x,value);
    }
    // No check, faster, but prone to errors
    fn set(&mut self,index: u32, value: Self::Output){
        self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut().get_data()[usize::try_from(index).unwrap()] = value;
    }
    
    fn get_row(&self,x: u32, y: u32) -> Vec<Self::Output>{
        let mut out : Vec<Self::Output> = Vec::new();
        let width = self.get_width_stack();
        for local_x in x..width{
            out.push(self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut().get(usize::try_from(y*width+local_x).unwrap()));
        }
        return out;
    } 
    fn get_column(&self,x: u32, y: u32) -> Vec<Self::Output>{
        let mut out : Vec<Self::Output> = Vec::new();
        let width = self.get_width_stack();
        let height = self.get_height_stack();
        for local_y in y..height{
            out.push(self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut().get(usize::try_from(local_y*width+x).unwrap()));
        }
        return out;
    }

    fn set_row(&mut self,x: u32, y: u32, data: Vec<Self::Output>){
        self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut().set_row(x,y,data);
    }
    fn set_column(&mut self,x: u32, y: u32, data: Vec<Self::Output>){
        self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut().set_column(x,y,data);
    }

    fn set_slice_number(&self,slice: u32){
        self.set_slice_number(slice);
    }
}


#[cfg(test)]
mod test{
    //use super ::super::ImageStack::{*};
    //use super :: super:: ImageProcessor::{*};
    use crate :: image_processor::*;
    use crate::image_stack::*;
    /*use crate::image_stack::ImageProcessor;
    use crate::image_stack::ByteStack;
    use crate::image_stack::ByteProcessor;
    use crate::image_stack::FloatProcessor;
    use crate::image_stack::FloatStack;*/
    use crate::color_space::ColorSpace;
    
    use crate::image_traits::Access;
    use core::cell::RefCell;
    use core::cell::Cell;

    #[test]
    fn test_ImageProcessor_get_pixel(){
       let img= ByteProcessor::create_byte_processor(10,15);
        assert_eq!(img.get_pixel(0),0);
    }

    #[test]
    #[should_panic(expected = "Pixel out of bounds ! index = 200, data length : 200")]
    fn test_ImageProcessor_get_pixel_panic(){
       let img= ByteProcessor::create_byte_processor(10,20);
       img.get_pixel(200);
    }

    #[test]
    fn test_ImageProcessor_get_pixel_at(){
       let img= ByteProcessor::create_byte_processor(10,15);
        assert_eq!(img.get_pixel_at(0,1),0);
    }

    #[test]
    #[should_panic(expected = "Pixel out of bounds ! x=10, width=10")]
    fn test_ImageProcessor_get_pixel_at_panic_width(){
       let img= ByteProcessor::create_byte_processor(10,15);
        img.get_pixel_at(10,0);
    }

    #[test]
    #[should_panic(expected = "Pixel out of bounds ! y=15, height=15")]
    fn test_ImageProcessor_get_pixel_at_panic_height(){
       let img= ByteProcessor::create_byte_processor(10,15);
        img.get_pixel_at(0,15);
    }

    #[test]
    fn test_ImageProcessor_get(){
        let img= ByteProcessor::create_byte_processor(10,15);
         assert_eq!(img.get(0),0);
    }

    #[test]
    fn test_ImageProcessor_set_pixel(){
        let mut img= ByteProcessor::create_byte_processor(2,2);
        img.set_pixel(0,10);
        assert_eq!(img.get_pixel(0),10);
    }

    #[test]
    #[should_panic(expected = "Pixel out of bounds ! index = 4, data length : 4")]
    fn test_ImageProcessor_set_pixel_panic(){
        let mut img= ByteProcessor::create_byte_processor(2,2);
        img.set_pixel(4,10);
    }

    #[test]
    fn test_ImageProcessor_set_pixel_color(){
        let mut img= ColorProcessor::create_color_processor(2,2);
        img.set_pixel(0,(10,0,16));
        assert_eq!(img.get_pixel(0),(10,0,16));
    }

    #[test]
    fn test_ImageProcessor_set_pixel_at(){
        let mut img= ByteProcessor::create_byte_processor(2,2);
        img.set_pixel_at(0,0,10);
        assert_eq!(img.get_pixel(0),10);
    }

    #[test]
    #[should_panic(expected = "Pixel out of bounds ! x=4, width=2")]
    fn test_ImageProcessor_set_pixel_at_panic_width(){
        let mut img= ByteProcessor::create_byte_processor(2,2);
        img.set_pixel_at(4,0,10);
    }

    #[test]
    #[should_panic(expected = "Pixel out of bounds ! y=4, height=2")]
    fn test_ImageProcessor_set_pixel_at_panic_height(){
        let mut img= ByteProcessor::create_byte_processor(2,2);
        img.set_pixel_at(0,4,10);
    }

    #[test]
    fn test_ImageProcessor_set_pixel_at_color(){
        let mut img= ColorProcessor::create_color_processor(2,2);
        img.set_pixel_at(0,0,(10,0,16));
        assert_eq!(img.get_pixel(0),(10,0,16));
    }

    #[test]
    fn test_ImageProcessor_set_pixel_at_float(){
        let mut img= FloatProcessor::create_float_processor(2,2);
        img.set_pixel_at(0,0,10.2);
        assert_eq!(img.get_pixel(0),10.2);
    }

    #[test]
    fn test_ImageProcessor_set_column(){
        let mut img= ColorProcessor::create_color_processor(2,2);
        let vec = vec![(1,1,1),(14,0,20)];
        img.set_column(0,0,vec![(1,1,1),(14,0,20)]);
        let row = img.get_column(0,0);
        assert_eq!(row,vec);
    }

    #[test]
    #[should_panic(expected = "You cannot set the slice number for a single image")]
    fn test_ImageProcessor_set_slice_number(){
        let mut img= ColorProcessor::create_color_processor(2,2);
        img.set_slice_number(1);
    }

    #[test]
    fn test_ImageStack_get_pixel(){
        let stack= ByteStack::create_byte_stack(10,15,12);
        assert_eq!(stack.get_pixel(0),0);
    }

    #[test]
    fn test_ImageStack_get_pixel_at(){
        let stack= ByteStack::create_byte_stack(10,15,12);
        assert_eq!(stack.get_pixel_at(0,1),0);
    }

    #[test]
    #[should_panic(expected = "Pixel out of bounds ! x=20, width=10")]
    fn test_ImageStack_get_pixel_at_panic_width(){
        let stack= FloatStack::create_float_stack(10,15,12);
        stack.get_pixel_at(20,1);
    }

    #[test]
    #[should_panic(expected = "Pixel out of bounds ! y=30, height=15")]
    fn test_ImageStack_get_pixel_at_panic_height(){
        let stack= ColorStack::create_color_stack(10,15,12);
        stack.get_pixel_at(0,30);
    }

    #[test]
    fn test_ImageStack_get(){
        let stack= FloatStack::create_float_stack(10,15,12);
        assert_eq!(stack.get(0),0.0);
    }

    #[test]
    fn test_ImageStack_set_pixel(){
        let mut stack= FloatStack::create_float_stack(10,15,12);
        stack.set_pixel(0,20.5);
        assert_eq!(stack.get_pixel(0),20.5);
    }

    #[test]
    #[should_panic(expected = "Pixel out of bounds ! index = 300, data length : 150")]
    fn test_ImageStack_set_pixel_panic(){
        let mut stack= FloatStack::create_float_stack(10,15,12);
        stack.set_pixel(300,20.5);
    }

    #[test]
    fn test_ImageStack_set_pixel_at(){
        let mut stack= ColorStack::create_color_stack(10,15,12);
        stack.set_pixel_at(0,0,(10,50,60));
        assert_eq!(stack.get_pixel(0),(10,50,60));
    }

    #[test]
    #[should_panic(expected = "Pixel out of bounds ! x=10, width=10")]
    fn test_ImageStack_set_pixel_at_panic_width(){
        let mut stack= ColorStack::create_color_stack(10,15,12);
        stack.set_pixel_at(10,0,(10,50,60));
    }

    #[test]
    #[should_panic(expected = "Pixel out of bounds ! y=15, height=15")]
    fn test_ImageStack_set_pixel_at_panic_height(){
        let mut stack= ByteStack::create_byte_stack(10,15,12);
        stack.set_pixel_at(0,15,30);
    }

    #[test]
    fn test_ImageStack_set(){
        let mut stack= ByteStack::create_byte_stack(10,15,12);
        stack.set(0,30);
        assert_eq!(stack.get_pixel(0),30);
    }

    #[test]
    fn test_ImageStack_get_row(){
        let stack= ByteStack::create_byte_stack(2,2,12);
        let vec = vec![0,0];
        assert_eq!(stack.get_row(0,0),vec);
    }

    #[test]
    fn test_ImageStack_get_column(){
        let stack= ByteStack::create_byte_stack(2,2,12);
        let vec = vec![0,0];
        assert_eq!(stack.get_column(0,0),vec);
    }

    #[test]
    fn test_ImageStack_set_row(){
        let mut stack= ColorStack::create_color_stack(10,15,12);
        let vec = vec![(255,50,8)];
        stack.set_row(0,0,vec![(255,50,8)]);
        assert_eq!(stack.get_row(0,0)[0],vec[0]);
    }

    #[test]
    fn test_ImageStack_set_column(){
        let mut stack= ColorStack::create_color_stack(10,15,12);
        let vec = vec![(255,50,8)];
        stack.set_column(2,2,vec![(255,50,8)]);
        assert_eq!(stack.get_column(2,0)[2],vec[0]);
    }

    #[test]
    fn test_ImageStack_set_slice_number(){
        let mut stack= FloatStack::create_float_stack(10,15,12);
        stack.set_slice_number(11);
        assert_eq!(stack.get_focus_slice(),11);
    }

}
/*
TODO
    
    /// Plug_In_Filter_Runner uses this method to set the slice number.
    fn set_slice_number​(&self,i32 slice);
*/



/*
pub trait Resize {
    /// Returns a new processor containing an image that corresponds to the current ROI.
    fn crop(&self) -> Image_Processor<T> {}
    /// Returns a duplicate of this image.
    fn  duplicate(&self) -> ImageProcessor {}
}
*/