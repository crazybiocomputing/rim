//
//  RIM - Rust IMage
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

use crate::color_space::ColorSpace;
use crate::meta_data::MetaData;
use crate::pixel::PixelType;

///
/// ImageProcessor
///
pub struct ImageProcessor<T: PixelType, C: ColorSpace> {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub data: Vec<T>,
    pub cs: C,
    pub metadata: MetaData,
}

impl<T: PixelType, C: ColorSpace> ImageProcessor<T, C> {
    // Constructor
    pub fn new(w: u32, h: u32, pixels: Vec<T>, cs: C) -> Self {
        ImageProcessor {
            width: w,
            height: h,
            depth: 1,
            data: pixels,
            cs: cs,
            metadata: MetaData::new(w, h),
        }
    }
    // Constructor
    pub fn new_volume(w: u32, h: u32, d: u32, pixels: Vec<T>, cs: C) -> Self {
        ImageProcessor {
            width: w,
            height: h,
            depth: d,
            data: pixels,
            cs: cs,
            metadata: MetaData::new(w, h),
        }
    }
    pub fn with_pixels(w: u32, h: u32, pixels: Vec<T>) -> Self {
        ImageProcessor {
            width: w,
            height: h,
            depth: 1,
            data: pixels,
            cs: C::new(),
            metadata: MetaData::new(w, h),
        }
    }
    // Accessors
    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn get_height(&self) -> u32 {
        self.height
    }
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
    pub fn get_size(&self) -> usize {
        (self.width * self.height) as usize
    }
    pub fn get_bit_depth(&self) -> usize {
        const BITS_PER_BYTE: usize = 8;
        BITS_PER_BYTE * std::mem::size_of::<T>()
    }
}

/*

use crate::color_space::ColorSpace;
use crate::image_traits::Access;
use std::cell::RefCell;
use std::cell::RefMut;

// ImageProcessor
// Generic Struct for dedicated Processors:
// Byte_Processor, Short_Processor, Float_Processor, Color_Processor, Image_Plus, Image_Stack
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
/// A generic Struct for dedicated Processors:
/// Byte_Processor, Short_Processor, Float_Processor, Color_Processor, Image_Plus, Image_Stack
pub struct ImageProcessor<T>{
    /// The width of the processor in pixel
    width: u32,
    /// The height of the processor in pixel
    height: u32,
    /// The
    data: RefCell<Vec<T>>,
    // meta: MetaData, // Contains all the file info + lut : [u8; 256 * 3], etc.
    cs : ColorSpace<T>
}


pub type ByteProcessor = ImageProcessor<u8>;
pub type FloatProcessor = ImageProcessor<f32>;
pub type ColorProcessor = ImageProcessor<(u8,u8,u8)>;


impl<T> ImageProcessor<T> where T: Copy {
    //// Constructeur g??n??rique ////
    /// creates an image processor of dimentions width x height, containing an array and a color space of type T
    pub fn create_processor(width: u32, height: u32, data : RefCell<Vec<T>>, cs : ColorSpace<T>) -> ImageProcessor<T> {
        return ImageProcessor{
            width : width,
            height : height,
            data : data,
            cs : cs,
        }
    }

    //// Constructeurs sp??cialis??s ////
    /// creates a byte processor of dimentions width x height
    pub fn create_byte_processor(width: u32, height: u32) -> ByteProcessor {
        let cs : ColorSpace<u8> = ColorSpace::<u8>::Gray8();
        let data = RefCell::new(vec![0 as u8; (width*height) as usize]);
        return ImageProcessor::<u8>::create_processor(width, height, data, cs )
    }
    /// creates a float processor of dimentions width x height
    pub fn create_float_processor(width: u32, height: u32) -> FloatProcessor {
        let cs : ColorSpace<f32> = ColorSpace::<f32>::Grayf32();
        let data = RefCell::new(vec![0 as f32; (width*height) as usize]);
        return ImageProcessor::<f32>::create_processor(width, height, data, cs )
    }
    /// creates a color processor of dimentions width x height
    pub fn create_color_processor(width: u32, height: u32) -> ColorProcessor {
        let cs : ColorSpace<(u8,u8,u8)> = ColorSpace::<(u8,u8,u8)>::Rgb24();
        let data = RefCell::new(vec![(0 as u8,0 as u8,0 as u8) ; (width*height) as usize]);
        return ImageProcessor::<(u8,u8,u8)>::create_processor(width, height, data, cs )
    }


    //// Affichage ////
    /// Shows some basic information about the image processor
    pub fn debug(&self){
        println!("ImageProcessor : Dimensions : {}x{} px, Bit depth : {}, data length : {}", self.get_width(), self.get_height(), self.get_bit_depth(), self.get_data().len());
    }

    //// Getters ////
    /// returns the width of the processor
    pub fn get_width(&self) -> u32 {
        return self.width
    }
    /// returns the height of the processor
    pub fn get_height(&self) -> u32 {
        return self.height
    }
    /// returns the data of the processor
    pub fn get_data(&self) -> RefMut<Vec<T>> {
        return self.data.borrow_mut()
    }

    /// Returns the bit depth, 8, 16, 24 (RGB) or 32.
    pub fn get_bit_depth(&self) -> u8 {
        return self.cs.get_bit_depth();
    }

    /// Returns the number of color channels in the image (1 for grayscale)
    pub fn get_nb_channels(&self) -> u8 {
        return self.cs.get_nb_channels();
    }

    /// returns the mimimum possible value
    pub fn get_min_possible(&self) -> T{
        return self.cs.get_min()
    }

    /// returns the maximum possible value
    pub fn get_max_possible(&self) -> T{
        return self.cs.get_max()
    }

}
*/

/*
#[cfg(test)]
mod test{
    use crate::image_processor::ImageProcessor;
    use crate::image_processor::ByteProcessor;
    use crate::image_processor::FloatProcessor;
    use crate::color_space::ColorSpace;
    use crate::image_processor::ColorProcessor;
    use core::cell::RefCell;
    use core::cell::Cell;

    #[test]
    fn test_create_byte_processor(){
        let img = ByteProcessor{
            width : 10,
            height : 20,
            data : RefCell::new(vec![0 as u8; (10*20) as usize]),
            cs : ColorSpace::<u8>::Gray8(),
        };
        assert_eq!(ByteProcessor::create_byte_processor(10,20), img);
    }

    #[test]
    fn test_create_float_processor(){
        let img = FloatProcessor{
            width : 10,
            height : 20,
            data : RefCell::new(vec![0 as f32; (10*20) as usize]),
            cs : ColorSpace::<f32>::Grayf32(),
        };
        assert_eq!(FloatProcessor::create_float_processor(10,20), img);
    }

    #[test]
    fn test_create_color_processor(){
        let img = ColorProcessor{
            width : 10,
            height : 20,
            data : RefCell::new(vec![(0 as u8,0 as u8,0 as u8); (10*20) as usize]),
            cs : ColorSpace::<(u8,u8,u8)>::Rgb24(),
        };
        assert_eq!(ColorProcessor::create_color_processor(10,20), img);
    }

    #[test]
    fn test_debug(){
        let img = ColorProcessor{
            width : 10,
            height : 20,
            data : RefCell::new(vec![(0 as u8,0 as u8,0 as u8); (10*20) as usize]),
            cs : ColorSpace::<(u8,u8,u8)>::Rgb24(),
        };
        assert_eq!(img.debug(), println!("ImageProcessor : Dimensions : 10x20 px, Bit depth : 8, data length : 200)"));
    }

    #[test]
    fn test_get_width(){
        let img = ColorProcessor{
            width : 10,
            height : 20,
            data : RefCell::new(vec![(0 as u8,0 as u8,0 as u8); (10*20) as usize]),
            cs : ColorSpace::<(u8,u8,u8)>::Rgb24(),
        };
        assert_eq!(img.get_width(),10);
    }

    #[test]
    fn test_get_height(){
        let img = ColorProcessor{
            width : 10,
            height : 20,
            data : RefCell::new(vec![(0 as u8,0 as u8,0 as u8); (10*20) as usize]),
            cs : ColorSpace::<(u8,u8,u8)>::Rgb24(),
        };
        assert_eq!(img.get_height(),20);
    }
/*
    #[test]
    fn test_get_data(){
        let img = ColorProcessor{
            width : 10,
            height : 20,
            data : RefCell::new(vec![(0 as u8,0 as u8,0 as u8); (10*20) as usize]),
            cs : ColorSpace::<(u8,u8,u8)>::Rgb24(),
        };
        let vec =RefCell::new(vec![(0 as u8,0 as u8,0 as u8); (10*20) as usize]);
        assert_eq!(img.get_data(),vec);
    }
*/

    #[test]
    fn test_get_bit_depth(){
        let img = ColorProcessor{
            width : 10,
            height : 20,
            data : RefCell::new(vec![(0 as u8,0 as u8,0 as u8); (10*20) as usize]),
            cs : ColorSpace::<(u8,u8,u8)>::Rgb24(),
        };
        assert_eq!(img.get_bit_depth(),24);
    }


    #[test]
    fn test_get_nb_channels(){
        let img = ColorProcessor{
            width : 10,
            height : 20,
            data : RefCell::new(vec![(0 as u8,0 as u8,0 as u8); (10*20) as usize]),
            cs : ColorSpace::<(u8,u8,u8)>::Rgb24(),
        };
        assert_eq!(img.get_nb_channels(),3);
    }

    #[test]
    fn test_get_min_possible(){
        let img = ColorProcessor{
            width : 10,
            height : 20,
            data : RefCell::new(vec![(0 as u8,0 as u8,0 as u8); (10*20) as usize]),
            cs : ColorSpace::<(u8,u8,u8)>::Rgb24(),
        };
        assert_eq!(img.get_min_possible(),(0,0,0));
    }

    #[test]
    fn test_get_max_possible(){
        let img = ColorProcessor{
            width : 10,
            height : 20,
            data : RefCell::new(vec![(0 as u8,0 as u8,0 as u8); (10*20) as usize]),
            cs : ColorSpace::<(u8,u8,u8)>::Rgb24(),
        };
        assert_eq!(img.get_max_possible(),(255,255,255));
    }

}





  pub fn with_pixels(w: u32, h: u32, px: Vec<T>, cm: C) -> Self {};


    /// Transforms the image or ROI using a lookup table.
    fn apply_table???(&self,i32[] lut) {}

    /// Returns a shallow copy of this Image_Processor, where this image and the copy share pixel data.
    java.lang.Object clone(&self,) {}

    /// Image_Processor convert_To_Byte???(&self,bool do_Scaling) {}
    /// Returns an 8-bit version of this image as a Byte_Processor.
    /// Byte_Processor convert_To_Byte_Processor(&self,) {}
    /// Returns an 8-bit version of this image as a Byte_Processor.
    /// Byte_Processor convert_To_Byte_Processor???(&self,bool scale) {}
    /// Returns an 8-bit version of this image as a Byte_Processor.
    /// Color_Processor convert_To_Color_Processor(&self,) {}
    /// Returns an RGB version of this image as a Color_Processor.
    /// Image_Processor convert_To_Float(&self,) {}
    /// Returns a 32-bit float version of this image as a Float_Processor.
    /// Float_Processor convert_To_Float_Processor(&self,) {}
    /// Returns a 32-bit float version of this image as a Float_Processor.
    /// Image_Processor convert_To_RGB(&self,) {}
    /// Returns an RGB version of this image as a Color_Processor.
    /// Image_Processor convert_To_Short???(&self,bool do_Scaling) {}
    /// Returns a 16-bit version of this image as a Short_Processor.
    /// Short_Processor convert_To_Short_Processor(&self,) {}
    /// Returns a 16-bit version of this image as a Short_Processor.
    /// Short_Processor convert_To_Short_Processor???(&self,bool scale) {}
    /// Returns a 16-bit version of this image as a Short_Processor.

    /// Copies the image contained in 'ip' to (&self,xloc, yloc) using one of the transfer modes defined in the Blitter interface.
    fn copy_bits???(&self,Image_Processor ip, i32 xloc, i32 yloc, i32 mode) {}


    /// This is a faster version of get_Pixel(&self,) that does not do bounds checking.
    fn get???(&self, index: u32) -> Vec<T> {
      }

    /// This is a faster version of get_Pixel(&self,) that does not do bounds checking
    fn get???(&self,i32 x, i32 y) -> T {}

    /// Returns the LUT index that's the best match for this color.
    fn i32 get_best_index???(&self,java.awt.Color c) {}

     /// Returns the calibration table or null.
    float[] get_calibration_table(&self,) {}

    /// Returns the current color model, which may have been modified by set_Min_And_Max(&self,) or set_Threshold(&self,).
    java.awt.image.Color_Model get_Current_Color_Model(&self,) {}

    /// Returns the default grayscale Index_Color_Model.
    java.awt.image.Index_Color_Model get_default_color_model(&self,) {}

    /// Returns the value of the pixel at index `index` as a float.
    fn  get???(&self,i32 index) -> T {}

    /// Returns the value of the pixel at (&self,x,y) as a float.
    fn get_at(&self,i32 x, i32 y)  -> T {}

    /// Returns a copy of the pixel data as a 2D float array with dimensions [x=0..width-1][y=0..height-1].
    T[][] get_float_array(&self,) {}


    /// Returns the height of this image in pixels.
    pub fn get_height(&self,&self) -> u32 {
        self.height
    }


    /// Returns a copy of the pixel data as a 2D i32 array with dimensions [x=0..width-1][y=0..height-1].
    i32[][] get_i32_array(&self,) {}


    /// LUT get_Lut(&self,)

    /// Returns the LUT update mode, which can be RED_LUT, BLACK_AND_WHITE_LUT, OVER_UNDER_LUT or NO_LUT_UPDATE.
    /// Image_Processor get_Mask(&self,) {}
    /// For images with irregular ROIs, returns a mask, otherwise, returns null.
    i32 get_lut_update_mode(&self,) {}

    /// Returns a reference to the mask pixel array, or null if there is no mask.
    byte[] get_mask_array(&self,) {}

    /// Experimental
    /// Overlay get_Overlay(&self,)

    /// Returns the value of the pixel at (&self,x,y).
    fn i32 get_pixel???(&self,i32 x, i32 y) {}

    /// Returns the samples for the pixel at (&self,x,y) in an i32 array.
    i32[] get_pixel???(&self,i32 x, i32 y, i32[] i_Array) {}

    fn i32 get_pixel_count(&self) {
      self.width * self.height * self.depth
    }

    /// Returns a reference to this image's pixel array.
    fn java.lang.Object get_pixels(&self,) {}

    /// Returns a copy of the pixel data.
    fn java.lang.Object get_pixels_copy(&self,) {}

    /// Returns the value of the pixel at (&self,x,y).
    fn float get_pixel_value???(&self,i32 x, i32 y) {}

    protected i32 get_progress_increment???(&self,i32 w, i32 h)

    /// Returns the pixel values along the horizontal line starting at (&self,x,y).
    float[] get_row???(&self,i32 x, i32 y, float[] data, i32 length) {}

    /// Returns the pixel values along the horizontal line starting at (&self,x,y).
    fn get_row???(&self,i32 x, i32 y, i32[] data, i32 length) {}

    fn get_slice_number(&self,) -> i32 {}


    /// Returns the value of the pixel at (&self,x,y), a calibrated value from 8-bit and 16-bit images,
    /// an intensity value from RGB images and a f64 value from 32-bit images.
    pub fn get_value???(&self,i32 x, i32 y) -> f64 {}

    /// Returns the width of this image in pixels.
    pub fn get_width(&self,&self) -> u32 {
        self.width
    }

    /// Inserts the image contained in 'ip' at (&self,xloc, yloc).
    fn insert???(&self,Image_Processor ip, i32 xloc, i32 yloc) {}

    /// Returns 'true' if this is a binary image (&self,8-bit-image with only 0 and 255).
    bool is_binary(&self,) {}

    /// Returns true if this image uses a color LUT.
    bool is_color_lut(&self,) {}

    /// Returns true if the image is using the default grayscale LUT.
    bool is_default_lut(&self,) {}

    /// Returns 'true' if this is an image with a grayscale LUT or an RGB image with identical red, green and blue channels.
    bool is_grayscale(&self,) {}

    /// Returns true if this image uses an inverting LUT that displays zero as white and 255 as black.
    bool is_inverted_lut(&self,) {}

    /// Returns true if this image uses a pseudocolor or grayscale LUT, in other words, is this an image that can be filtered.
    bool is_pseudo_color_lut(&self,) {}

    /// Returns 'true' if this is a signed 16-bit image.
    bool is_signed16bit(&self,) {}

   // protected java.lang.String mask_Size_Error???(&self,Image_Processor mask)


    /// Restores the pixel data from the snapshot (&self,undo) buffer.
    abstract fn reset(&self,) {}

    /// Restores pixels from the snapshot buffer that are within the rectangular roi but not part of the mask.
    abstract fn reset???(&self,Image_Processor mask) {}

    /// For short and float images, recalculates the min and max image values needed to correctly display the image.
    fn reset_Min_And_Max(&self,) {}



    /// Sets the pixel at (&self,x,y) to the current fill/draw value.
    /// This is a faster version of put_Pixel(&self,) that does not clip out of range values and does not do bounds checking.
    abstract fn set???(&self,i32 index, i32 value)

    /// Sets the pixel at (&self,x,y) to the current fill/draw value.
    /// This is a faster version of put_Pixel(&self,) that does not clip out of range values and does not do bounds checking.
    abstract fn set???_at(&self,i32 x, i32 y, i32 value) {}

    /// Set a lookup table used by get_Pixel_Value(&self,), get_Line(&self,) and convert_To_Float(&self,) to calibrate pixel values.
    fn set_Calibration_Table???(&self,float[] c_Table) {}



    fn set_lut???(&self,LUT lut)
    fn set_lut_animation???(&self,bool lut_Animation) {}
    /// For 16 and 32 bit processors, set 'lut_Animation' true to have create_Image(&self,) use the cached 8-bit version of the image.

    fn set_min_and_max???(&self,f64 min, f64 max) {}
    /// This image will be displayed by mapping pixel values in the range min-max to screen values in the range 0-255.

    static fn set_over_color???(&self,i32 red, i32 green, i32 blue) {}
    /// Sets the upper Over/Under threshold color.

    fn set_overlay???(&self,Overlay overlay) {}
    /// This method is used to display virtual stack overlays.

    /// Sets the default fill/draw value.
    fn set_value???(&self,f64 value) {}

    /// Makes a copy of this image's pixel data that can be later restored using reset(&self,) or reset(&self,mask).
    fn snapshot(&self,) {}

    /// Swaps the pixel and snapshot (&self,undo) buffers.
    fn swap_pixel_arrays(&self,) {}

    /// Returns a Float_Processor with the image or one color channel thereof.
    fn  to_float???(&self,i32 channel_Number, Float_Processor fp) -> FloatProcessor {}


    /// This method is used by Composite_Image.update_Image(&self,) to create RGB images (&self,for display) of a multi-channel composite images.
    fn update_composite???(&self,i32[] rgb_Pixels, i32 mode) {}


}



impl fmt::Display for ImageProcessor {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "ImageProcessor {}x{}x{}", self.width, self.height, self.depth)
    }
}
*/
