#![allow(non_snake_case)]
#![allow(unused)]

use crate::io::raw_reader::*;
use crate::image_stack::*;
use crate::image_traits::*;
use crate::image_processor::*;


/// A structure for the readed image
pub struct ImageReader {
    height: u32,
    width: u32,
    slice: u32,
    fi: String,
    buffer: Vec<u8>
}

impl ImageReader{
    /// Return a image with his parameters (Structure qui peut gérer les 8bits, 16bits, 32bits et RGB pour les testes, 16bits pas dans l'API et 32bits en float dan l'API et tuple de u8 pour les row)
    ///
    /// # arguments
    ///
    /// * `height` - A number u32 for the height of the image
    /// * `width` - A number u32 for the width of the image
    /// * `slice` - A number u32 for the number of slices in a stack, 1 for a single image
    /// * `fi` - String corresponding to the byte type of the image
    /// * `buffer` - Vector containing the raw data from the image
    ///
    pub fn new(height: u32, width: u32, slice: u32, fi: &String, buffer: Vec<u8>) -> Self {
        ImageReader {
        height: height,
        width: width,
        slice: slice,
        fi: fi.clone(),
        buffer: buffer,
        }
    }

    /// Return the height of the image
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Return the width of the image
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Return the number of slice of the stack
    pub fn slice(&self) -> u32 {
        self.slice
    }

    /// Return the vector of the raw data
    pub fn buffer(&self) -> &Vec<u8> {
        &self.buffer
    }

    /// Return the type of the image
    pub fn fi(&self) -> &str {
        &self.fi
    }

    /// Return the length of the vector of raw data
    pub fn len(&self) -> u32 {
        self.buffer.len() as u32
    }

    /// Modify the value of a pixel (utilisable que pour une image 8bit)
    ///
    /// # arguments
    ///
    /// * `index` - The index in the vector of the value to modify
    /// * `val` - The new value
    ///
    pub fn modifyPixel(&mut self, index: usize, val: u8){
        let newvec = &mut self.buffer().clone();
        newvec[index] = val;
        self.buffer = newvec.to_vec();
    }
}

/// Restructure to separate the different slices
    ///
    /// # arguments
    ///
    /// * `data` - The vector of raw data
    /// * `nb_slice` - The number of images in the stack
    ///
pub fn cut_by_slices(data:Vec<u8>, nb_slice:u32) ->Vec<Vec<u8>>
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
    //file.modifyBufferType(newbuffer);
    return newbuffer;
}

/*
/// Modify the value of a pixel (utilisable que pour une image 8bit)
    ///
    /// # arguments
    ///
    /// * `index` - The index in the vector of the value to modify
    /// * `val` - The new value
    ///
fn cut_by_pixels_16(file:&ImageReader, image:Vec<Vec<u8>>) -> Vec<Vec<(u8,u8)>>
{
    let mut nb_pix_per_slice = file.len()/ file.slice();
    let mut buffer:Vec<Vec<(u8,u8)>> = vec![];
    for i in 0..file.slice()
    {
        let mut temp:Vec<(u8, u8)> = vec![];
        for j in (0..nb_pix_per_slice).step_by(2)
        {
            let iu = i as usize;
            let ju = j as usize;
            let mut tup = (image[iu][ju], image[iu][ju+1]);
            temp.push(tup);
        }
        buffer.push(temp);
    }
    return buffer;
}

/// Modify the value of a pixel (utilisable que pour une image 8bit)
    ///
    /// # arguments
    ///
    /// * `index` - The index in the vector of the value to modify
    /// * `val` - The new value
    ///
fn cut_by_pixels_32(file:&ImageReader, image:Vec<Vec<u8>>) -> Vec<Vec<(u8,u8, u8, u8)>>
{
    let mut nb_pix_per_slice = file.len()/ file.slice();
    let mut buffer:Vec<Vec<(u8,u8, u8, u8)>> = vec![];
    for i in 0..file.slice()
    {
        let mut temp:Vec<(u8, u8, u8, u8)> = vec![];
        for j in (0..nb_pix_per_slice).step_by(4)
        {
            let iu = i as usize;
            let ju = j as usize;
            let mut tup = (image[iu][ju], image[iu][ju+1], image[iu][ju+2], image[iu][ju+3]);
            temp.push(tup);
        }
        buffer.push(temp);
    }
    return buffer;
}

*/


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
  let data = cut_by_slices(buffer,slice);

  let mut stack = ByteStack::create_byte_stack(height,width,slice);

  for i in 0..data.len() {
    stack.set_slice_number(i.try_into().unwrap());
    for j in 0..data[i].len() {
      stack.set(j.try_into().unwrap(),data[i][j]);
    }
  }
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
  let data = cut_by_slices(buffer,slice);
  let data_cut = slice_rgb(slice,data);

  let mut stack = ColorStack::create_color_stack(height,width,slice);


  for i in 0..data_cut.len() {
    stack.set_slice_number(i.try_into().unwrap());
    for j in 0..data_cut[i].len() {
      stack.set(j.try_into().unwrap(),data_cut[i][j]);
    }
  }
  return stack;
}

/// Save the data of an image stack of 8bit into raw file
    ///
    /// # arguments
    ///
    /// * `stack` - The ByteStack containing the data
    /// * `filename` - Name of the created file
    ///
pub fn save_byte_stack(stack:ByteStack, filename:&String){
  let mut raw_data:Vec<u8> = vec![];

  let slices = stack.get_data_stack();
  println!("{:?}",slices);

  for i in 0..slices.len() {
    let mut image_data = slices[i].borrow();
    let image_data_borrow = image_data.get_data();
    for j in 0..image_data_borrow.len() {
      raw_data.push(image_data_borrow[j]);
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
    pub fn save_color_stack(stack:ByteStack, filename:&String){
        let mut raw_data:Vec<u8> = vec![];
      
        let slices = stack.get_data_stack();
        println!("{:?}",slices);
      
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
