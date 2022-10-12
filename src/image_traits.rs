//
//  RIM - Rust Image
//  Copyright (&self,C) 2022  Jean-Christophe Taveau.
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

use crate::image_processor::ImageProcessor;
 
pub trait Access<T> {
    type Output;
    // Get 1 pixel
    fn get_pixel(&self, index: u32) -> Self::Output;
    fn get_pixel_at(&self,x: u32, y: u32) -> Self::Output;
    fn get(&self,index: usize) -> Self::Output;

    // Set 1 pixel
    //fn put_pixel​(&self,u32 x, u32 y, Self::Output);
    fn set(&mut self,index: u32, value: Self::Output);
    //fn set_at​(&self,u32 x, u32 y, value: Self::Output);
    
    
}

impl<T> Access<T> for ImageProcessor<T> where T:Copy{
    type Output = T;

    ///// Get 1 pixel /////
    fn get_pixel(&self, index: u32) -> Self::Output{
        if u32::from(index) >= self.get_width()*self.get_height(){
            panic!("Pixel out of bounds  ! index = {}, data length : {}",index ,self.get_width()*self.get_height());
        }
        return self.data()[usize::try_from(index).unwrap()];
    }
    fn get_pixel_at(&self,x: u32, y: u32) -> Self::Output{
        if x >= self.get_width(){
            panic!("Pixel out of bounds ! x={}, width={}",x,self.get_width());
        }
        if y >= self.get_height(){
            panic!("Pixel out of bounds  ! x={}, height={}",y,self.get_height());
        }
        return self.get_pixel(y*self.get_width()+x)
    }
    // No check, faster, but prone to errors
    fn get(&self,index: usize) -> Self::Output{
        return self.data()[index];
    }

    /*
    ///// set 1 Pixel /////
    //fn put_pixel​(&self,i32 x, i32 y, i32 value) {}

    /// Sets the value of the pixel at (&self,x,y) to 'value'.
    /// Sets the value of the pixel at (&self,x,y) to 'value'. 
    /// Does no bounds checking or clamping, making it faster than put_Pixel(&self,). 
    /// Due to the lack of bounds checking, (&self,x,y) coordinates outside the image may cause an exception. 
    /// Due to the lack of clamping, values outside the 0-255 range (for byte) or 0-65535 range (for short) 
    /// are not handled correctly.
    */
    fn set(&mut self,index: u32, value: Self::Output){
        self.data()[usize::try_from(index).unwrap()] = value;
    }
    
    
    //fn set_at​(&self,i32 x, i32 y, value: f32);

}



    /*
TODO
    /// Returns the pixel values down the column starting at (&self,x,y).
    float[] get_column​(&self,i32 x, i32 y, float[] data, i32 length);

    /// Returns the pixel values down the column starting at (&self,x,y).
    fn get_column​(&self,i32 x, i32 y, i32[] data, i32 length);
    
    /// Inserts the pixels contained in 'data' i32o a column starting at (&self,x,y).
    fn put_column​(&self,i32 x, i32 y, float[] data, i32 length) {}

    /// Inserts the pixels contained in 'data' i32o a column starting at (&self,x,y).
    fn put_column​(&self,i32 x, i32 y, i32[] data, i32 length) {}

    /// Stores the specified value at (&self,x,y).
    abstract fn put_pixel​(&self,i32 x, i32 y, i32 value) {}

    /// Sets a pixel in the image using an i32 array of samples.
    fn put_pixel​(&self,i32 x, i32 y, i32[] i_Array) {}

    /// Stores the specified value at (&self,x,y).
    abstract fn put_Pixel_Value​(&self,i32 x, i32 y, f64 value) {}

    /// Inserts the pixels contained in 'data' i32o a horizontal line starting at (&self,x,y).
    fn put_row​(&self,i32 x, i32 y, float[] data, i32 length) {}

    /// Inserts the pixels contained in 'data' i32o a horizontal line starting at (&self,x,y).
    fn put_row​(&self,i32 x, i32 y, i32[] data, i32 length) {}


    /// Sets the value of the pixel at (&self,x,y) to 'value'.
    /// Sets the value of the pixel at (&self,x,y) to 'value'. 
    /// Does no bounds checking or clamping, making it faster than put_Pixel(&self,). 
    /// Due to the lack of bounds checking, (&self,x,y) coordinates outside the image may cause an exception. 
    /// Due to the lack of clamping, values outside the 0-255 range (for byte) or 0-65535 range (for short) 
    /// are not handled correctly.
    fn setf​(&self,i32 index, value: f32);
    
    fn setf_at​(&self,i32 x, i32 y, value: f32);

    /// Replaces the pixel data with contents of the specified 2D float array.
    fn set_float_array​(&self,float[][] a);

    /// Replaces the pixel data with contents of the specified 2D i32 array.
    fn set_int_array​(&self,i32[][] a);
    
    /// Sets the pixels (&self,of one color channel for RGB images) from a Float_Processor.
    fn set_pixels​(&self,channel_number: i32, Float_Processor fp);

    /// Sets a new pixel array for the image.
    fn set_pixels​(&self,java.lang.Object pixels);

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
