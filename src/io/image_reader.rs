//
//  RIM - Rust IMage
//  Copyright (&self,C) 2022  Jean-Christophe Taveau, Allain Anaelle, Texier Louis.
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


#![allow(non_snake_case)]
#![allow(unused)]

use crate::io::raw_reader::*;
use crate::image_stack::*;
use crate::image_traits::*;
use crate::image_processor::*;

/// Restructure to separate the different slices
    ///
    /// # arguments
    ///
    /// * `data` - The vector of raw data
    /// * `nb_slice` - The number of images in the stack
    ///
pub fn separate_slices(data:Vec<u8>, nb_slice:u32) ->Vec<Vec<u8>>
{
    let mut nb_pix_per_slice = data.len()/ nb_slice as usize;
    let mut newbuffer:Vec<Vec<u8>> = vec![];
    for i in (0..data.len()).step_by(nb_pix_per_slice.try_into().unwrap())
    {
        let mut temp:Vec<u8> = vec![];
        for j in i..(nb_pix_per_slice+i)
        {

            let mut u = (j) as usize;
            temp.push(data[u]);
        }
        newbuffer.push(temp);
    }
    return newbuffer;
}

/// Restructure the vector into a vector of tuple (u8,u8,u8)
    ///
    /// # arguments
    ///
    /// * `nb_slice` - The number of images in the stack
    /// * `image` - The vector with slices separated
    ///
fn slice_rgb(nb_slice:u32, image:Vec<Vec<u8>>) -> Vec<Vec<(u8,u8, u8)>>
{


    let sum = image.len() * image[0].len() as usize;
    let mut nb_pix_per_slice = sum/ nb_slice as usize;
    let mut buffer:Vec<Vec<(u8,u8, u8)>> = vec![];
    for i in 0..nb_slice
    {
        let mut temp:Vec<(u8, u8, u8)> = vec![];
        for j in (0..nb_pix_per_slice).step_by(3)
        {
            let iu = i as usize;
            let ju = j as usize;
            let mut tup = (image[iu][ju], image[iu][ju+1], image[iu][ju+2]);
            temp.push(tup);
        }
        buffer.push(temp);
    }
    return buffer;
}

/// Read a raw file and put he data into an image stack of 8bit
    ///
    /// # arguments
    ///
    /// * `height` - The height of the images
    /// * `width` - The width of the images
    /// * `slice` - The number of images in the stack
    /// * `filename` - The name of the raw file
    ///
pub fn read_byte_stack(height:u32,width:u32,slice:u32,filename:&String) -> ByteStack{
  let buffer = get_file_as_byte_vec(filename);
  let data = separate_slices(buffer,slice);

  let mut stack = ImageStack::new(width,height,data,Gray8::new());
  return stack;
}

/// Read a raw file and put he data into an image stack of 16bit (16bit not implemented)
    ///
    /// # arguments
    ///
    /// * `height` - The height of the images
    /// * `width` - The width of the images
    /// * `slice` - The number of images in the stack
    /// * `filename` - The name of the raw file
    ///
    pub fn read_16_stack(height:u32,width:u32,slice:u32,filename:&String){
      let buffer = get_file_as_byte_vec(filename);
      let mut new_buffer:Vec<Vec<u16>> = vec![];
      let data = separate_slices(buffer,slice);
      for j in 0..data.len(){
        let mut slide:Vec<u16> = vec![];
        for i in (0..data[j].len()).step_by(2){
          let short = u16::from_be_bytes([data[j][i],data[j][i+1]]);
          slide.push(short);
        }
        new_buffer.push(slide);
      }
      let mut stack = ImageStack::new(width,height,new_buffer,C);
      return stack;

      /*
      let mut stack = ByteStack::create_byte_stack(height,width,slice);
    
      for i in 0..data.len() {
        stack.set_slice_number(i.try_into().unwrap());
        for j in 0..data[i].len() {
          stack.set(j.try_into().unwrap(),data[i][j]);
        }
      }
      return stack;
      */
    }

/// Read a raw file and put he data into an image stack of 32bit
    ///
    /// # arguments
    ///
    /// * `height` - The height of the images
    /// * `width` - The width of the images
    /// * `slice` - The number of images in the stack
    /// * `filename` - The name of the raw file
    ///
    pub fn read_float_stack(height:u32,width:u32,slice:u32,filename:&String) -> FloatStack{
      let buffer = get_file_as_byte_vec(filename);
      let mut new_buffer:Vec<Vec<f32>> = vec![];
      let data = separate_slices(buffer,slice);
      for j in 0..data.len(){
        let mut slide:Vec<f32> = vec![];
        for i in (0..data[j].len()).step_by(4){
          let short = f32::from_be_bytes([data[j][i],data[j][i+1],data[j][i+2],data[j][i+3]]);
          slide.push(short);
        }
        new_buffer.push(slide);
      }

      

    
      let mut stack = ImageStack::new(width,height,new_buffer,C);
      return stack;
    }

/// Read a raw file and put he data into an image stack of RGB
    ///
    /// # arguments
    ///
    /// * `height` - The height of the images
    /// * `width` - The width of the images
    /// * `slice` - The number of images in the stack
    /// * `filename` - The name of the raw file
    ///
pub fn read_stack_RGB(height:u32,width:u32,slice:u32,filename:&String) -> ColorStack{
  let buffer = get_file_as_byte_vec(filename);
  let data = separate_slices(buffer,slice);
  let data_cut = slice_rgb(slice,data);

  let mut stack = ImageStack::new(width,height,data,C);
  return stack;

}

/// Save the data of an image stack of 8bit into raw file
    ///
    /// # arguments
    ///
    /// * `stack` - The ByteStack containing the data
    /// * `filename` - Name of the created file
    ///
pub fn save_byte_stack(stack:ImageStack, filename:&String){
  let mut raw_data:Vec<u8> = vec![];

  let slices = stack.get_data_stack();

  for i in 0..slices.len() {
    let mut image_data = slices[i].borrow();
    let image_data_borrow = image_data.get_data();
    for j in 0..image_data_borrow.len() {
      raw_data.push(image_data_borrow[j]);
    }
  }
  save_raw_file(filename, raw_data);

}

/// Save the data of an image stack of 32bit into raw file
    ///
    /// # arguments
    ///
    /// * `stack` - The ByteStack containing the data
    /// * `filename` - Name of the created file
    ///
    pub fn save_float_stack(stack:ImageStack, filename:&String){
      let mut raw_data:Vec<u8> = vec![];
    
      let slices = stack.get_data_stack();
    
      for i in 0..slices.len() {
        let mut image_data = slices[i].borrow();
        let image_data_borrow = image_data.get_data();
        for j in 0..image_data_borrow.len() {
          let long = image_data_borrow[j].to_be_bytes();
          for k in 0..long.len(){
          raw_data.push(long[k]);
          }
        }
      }
      save_raw_file(filename, raw_data);
    
    }


/// Save the data of an image stack of RGB into raw file
    ///
    /// # arguments
    ///
    /// * `stack` - The ByteStack containing the data
    /// * `filename` - Name of the created file
    ///
    pub fn save_color_stack(stack:ImageStack, filename:&String){
        let mut raw_data:Vec<u8> = vec![];
      
        let slices = stack.get_data_stack();
      
        for i in 0..slices.len() {
          let mut image_data = slices[i].borrow();
          let image_data_borrow = image_data.get_data();
          for j in 0..image_data_borrow.len() {
            raw_data.push(image_data_borrow[j].0);
            raw_data.push(image_data_borrow[j].1);
            raw_data.push(image_data_borrow[j].2);
          }
        }
        save_raw_file(filename, raw_data);
      
      }

// TODO
/*
    Tests unitaires
    Préparer pour les 16bits et 32bits avec notre propre API

*/
//
//  RIM - Rust IMage
//  Copyright (&self,C) 2022  Jean-Christophe Taveau, Allain Anaelle, Texier Louis.
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


#![allow(non_snake_case)]
#![allow(unused)]

use crate::io::raw_reader::*;
use crate::image_stack::*;
use crate::image_traits::*;
use crate::image_processor::*;

/// Restructure to separate the different slices
    ///
    /// # arguments
    ///
    /// * `data` - The vector of raw data
    /// * `nb_slice` - The number of images in the stack
    ///
pub fn separate_slices(data:Vec<u8>, nb_slice:u32) ->Vec<Vec<u8>>
{
    let mut nb_pix_per_slice = data.len()/ nb_slice as usize;
    let mut newbuffer:Vec<Vec<u8>> = vec![];
    for i in (0..data.len()).step_by(nb_pix_per_slice.try_into().unwrap())
    {
        let mut temp:Vec<u8> = vec![];
        for j in i..(nb_pix_per_slice+i)
        {

            let mut u = (j) as usize;
            temp.push(data[u]);
        }
        newbuffer.push(temp);
    }
    return newbuffer;
}

/// Restructure the vector into a vector of tuple (u8,u8,u8)
    ///
    /// # arguments
    ///
    /// * `nb_slice` - The number of images in the stack
    /// * `image` - The vector with slices separated
    ///
fn slice_rgb(nb_slice:u32, image:Vec<Vec<u8>>) -> Vec<Vec<(u8,u8, u8)>>
{


    let sum = image.len() * image[0].len() as usize;
    let mut nb_pix_per_slice = sum/ nb_slice as usize;
    let mut buffer:Vec<Vec<(u8,u8, u8)>> = vec![];
    for i in 0..nb_slice
    {
        let mut temp:Vec<(u8, u8, u8)> = vec![];
        for j in (0..nb_pix_per_slice).step_by(3)
        {
            let iu = i as usize;
            let ju = j as usize;
            let mut tup = (image[iu][ju], image[iu][ju+1], image[iu][ju+2]);
            temp.push(tup);
        }
        buffer.push(temp);
    }
    return buffer;
}

/// Read a raw file and put he data into an image stack of 8bit
    ///
    /// # arguments
    ///
    /// * `height` - The height of the images
    /// * `width` - The width of the images
    /// * `slice` - The number of images in the stack
    /// * `filename` - The name of the raw file
    ///
pub fn read_byte_stack(height:u32,width:u32,slice:u32,filename:&String) -> ByteStack{
  let buffer = get_file_as_byte_vec(filename);
  let data = separate_slices(buffer,slice);

  let mut stack = ImageStack::new(width,height,data,C);
  return stack;
}

/// Read a raw file and put he data into an image stack of 16bit (16bit not implemented)
    ///
    /// # arguments
    ///
    /// * `height` - The height of the images
    /// * `width` - The width of the images
    /// * `slice` - The number of images in the stack
    /// * `filename` - The name of the raw file
    ///
    pub fn read_16_stack(height:u32,width:u32,slice:u32,filename:&String){
      let buffer = get_file_as_byte_vec(filename);
      let mut new_buffer:Vec<Vec<u16>> = vec![];
      let data = separate_slices(buffer,slice);
      for j in 0..data.len(){
        let mut slide:Vec<u16> = vec![];
        for i in (0..data[j].len()).step_by(2){
          let short = u16::from_be_bytes([data[j][i],data[j][i+1]]);
          slide.push(short);
        }
        new_buffer.push(slide);
      }
      let mut stack = ImageStack::new(width,height,new_buffer,C);
      return stack;

      /*
      let mut stack = ByteStack::create_byte_stack(height,width,slice);
    
      for i in 0..data.len() {
        stack.set_slice_number(i.try_into().unwrap());
        for j in 0..data[i].len() {
          stack.set(j.try_into().unwrap(),data[i][j]);
        }
      }
      return stack;
      */
    }

/// Read a raw file and put he data into an image stack of 32bit
    ///
    /// # arguments
    ///
    /// * `height` - The height of the images
    /// * `width` - The width of the images
    /// * `slice` - The number of images in the stack
    /// * `filename` - The name of the raw file
    ///
    pub fn read_float_stack(height:u32,width:u32,slice:u32,filename:&String) -> FloatStack{
      let buffer = get_file_as_byte_vec(filename);
      let mut new_buffer:Vec<Vec<f32>> = vec![];
      let data = separate_slices(buffer,slice);
      for j in 0..data.len(){
        let mut slide:Vec<f32> = vec![];
        for i in (0..data[j].len()).step_by(4){
          let short = f32::from_be_bytes([data[j][i],data[j][i+1],data[j][i+2],data[j][i+3]]);
          slide.push(short);
        }
        new_buffer.push(slide);
      }

      

    
      let mut stack = ImageStack::new(width,height,new_buffer,C);
      return stack;
    }

/// Read a raw file and put he data into an image stack of RGB
    ///
    /// # arguments
    ///
    /// * `height` - The height of the images
    /// * `width` - The width of the images
    /// * `slice` - The number of images in the stack
    /// * `filename` - The name of the raw file
    ///
pub fn read_stack_RGB(height:u32,width:u32,slice:u32,filename:&String) -> ColorStack{
  let buffer = get_file_as_byte_vec(filename);
  let data = separate_slices(buffer,slice);
  let data_cut = slice_rgb(slice,data);

  let mut stack = ImageStack::new(width,height,data,C);
  return stack;

}

/// Save the data of an image stack of 8bit into raw file
    ///
    /// # arguments
    ///
    /// * `stack` - The ByteStack containing the data
    /// * `filename` - Name of the created file
    ///
pub fn save_byte_stack(stack:ImageStack, filename:&String){
  let mut raw_data:Vec<u8> = vec![];

  let slices = stack.get_data_stack();

  for i in 0..slices.len() {
    let mut image_data = slices[i].borrow();
    let image_data_borrow = image_data.get_data();
    for j in 0..image_data_borrow.len() {
      raw_data.push(image_data_borrow[j]);
    }
  }
  save_raw_file(filename, raw_data);

}

/// Save the data of an image stack of 32bit into raw file
    ///
    /// # arguments
    ///
    /// * `stack` - The ByteStack containing the data
    /// * `filename` - Name of the created file
    ///
    pub fn save_float_stack(stack:ImageStack, filename:&String){
      let mut raw_data:Vec<u8> = vec![];
    
      let slices = stack.get_data_stack();
    
      for i in 0..slices.len() {
        let mut image_data = slices[i].borrow();
        let image_data_borrow = image_data.get_data();
        for j in 0..image_data_borrow.len() {
          let long = image_data_borrow[j].to_be_bytes();
          for k in 0..long.len(){
          raw_data.push(long[k]);
          }
        }
      }
      save_raw_file(filename, raw_data);
    
    }


/// Save the data of an image stack of RGB into raw file
    ///
    /// # arguments
    ///
    /// * `stack` - The ByteStack containing the data
    /// * `filename` - Name of the created file
    ///
    pub fn save_color_stack(stack:ImageStack, filename:&String){
        let mut raw_data:Vec<u8> = vec![];
      
        let slices = stack.get_data_stack();
      
        for i in 0..slices.len() {
          let mut image_data = slices[i].borrow();
          let image_data_borrow = image_data.get_data();
          for j in 0..image_data_borrow.len() {
            raw_data.push(image_data_borrow[j].0);
            raw_data.push(image_data_borrow[j].1);
            raw_data.push(image_data_borrow[j].2);
          }
        }
        save_raw_file(filename, raw_data);
      
      }
