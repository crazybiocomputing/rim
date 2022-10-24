//
//  RIM - Rust Image
//  Copyright (&self,C) 2022  Jean-Christophe Taveau
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


/// Authors , Nicolas Maurice, Bluwen Guidoux.

use crate::pixel::PixelType;

pub trait Access<T: PixelType> {
    type Output;

    /// Get a pixel at a specific index
    fn get_pixel(&self, index: usize) -> Result<Self::Output, &str>;
    
    /// Get a pixel at a specific x y position
    fn get_pixel_at(&self, x: u32, y: u32) -> Result<Self::Output, &str>;
    
    // Get a pixel at a specific index, without check and returns pixel value as f32.
    fn getf(&self, index: usize) -> f32;
    
    /// Get a pixel at a specific index, without check
    fn get(&self, index: usize) -> Self::Output;



    /// Set a pixel at a specific index
    fn set_pixel(&mut self, index: usize, v: T);
    
    /// Set a pixel at a specific x y position
    fn set_pixel_at(&mut self,x: u32, y: u32, value: Self::Output);
    
    /// Set 1 pixel at a specific index, without check
    fn set(&mut self,index: usize, value: Self::Output);  

    /// Set a pixel at a specific x y position, without check
    fn set_at(&mut self,x: u32, y: u32, value: Self::Output);
    

    /// Get a row of pixel, starting from a specific x y position
    fn get_row(&self,x: u32, y: u32) -> Vec<Self::Output>;
    /// Get a column of pixel, starting from a specific x y position
    fn get_column(&self,x: u32, y: u32) -> Vec<Self::Output>;

    /// Fill a row of pixel, starting from a specific x y position, with a vector of pixels. 
    fn set_row(&mut self,x: u32, y: u32, data: Vec<Self::Output>);
    /// Fill a column of pixel, starting from a specific x y position, with a vector of pixels. 
    fn set_column(&mut self,x: u32, y: u32, data: Vec<Self::Output>);

    // Set the slice number of a stack 
    // fn set_slice_number(&self,slice: u32);

}


/*
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

*/




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
