//
//  RIM - Rust IMage
//  Copyright (C) 2022  Jean-Christophe Taveau,.
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
// Authors:  Allain Anaelle, Texier Louis

#![allow(non_snake_case)]
#![allow(unused)]

use std::fs::metadata;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use crate::color_space::ColorSpace;
use crate::gray_processor::ip_gray;
use crate::grayscale::*;
use crate::image_processor::*;
use crate::image_stack::*;
use crate::image_traits::*;
use crate::io::file_info::*;
use crate::meta_data::MetaData;
use crate::pixel::PixelType;

pub enum OutputProcessor {
    ByteProcessor(ImageProcessor<u8, Gray8>),
    ShortProcessor(ImageProcessor<u16, Gray16>),
    UIntProcessor(ImageProcessor<u32, Gray32>),
    FloatProcessor(ImageProcessor<f32, Gray32>),
    ByteStack(ImageStack<u8, Gray8>),
    ShortStack(ImageStack<u16, Gray16>),
    UIntStack(ImageStack<u32, Gray32>),
    FloatStack(ImageStack<f32, Gray32>),
    Unknown(String),
}

///
/// FileOpener is a utility class to load raw binary file without header.
///
/// # Example
///
/// ```rust
///     let proc = FileOpener::open_processor(
///         "./samples/chessboard_u8_8x8.bin",
///         8, 8,
///         FileInfo::GRAY8
///     );
///     if let OutputProcessor::ByteProcessor(ip) == proc {
///         println!("{}x{}",ip.get_width(),ip.get_height());
///     }
/// ```

pub struct FileOpener {}

impl FileOpener {
    /// Open a _3D ImageProcessor_ from Raw|Binary File.
    /// There is no header in this file
    ///
    /// # Arguments
    ///
    /// * `filename` The file name
    /// * `w` The image width
    /// * `h` The image height
    /// * `d` The image depth
    /// * `ty` The image Type among the `FileInfo` constants.
    ///
    pub fn open_volume(filename: &str, w: u32, h: u32, d: u32, ty: u32) -> OutputProcessor {
        let length = (w * h * d) as usize;
        // Read data
        let buffer = FileOpener::get_file_as_byte_vec(&filename.to_string());

        // Create corresponding ImageProcessor depending of content
        match ty {
            FileInfo::GRAY8 => {
                let ip = ImageProcessor::<u8, Gray8>::new_volume(w, h, d, buffer, Gray8::new());
                OutputProcessor::ByteProcessor(ip)
            }
            FileInfo::GRAY16_UNSIGNED => {
                let mut data16 = Vec::<u16>::new();
                for i in (0..buffer.len()).step_by(2) {
                    // TODO: Check endianness????
                    let short = u16::from_be_bytes([buffer[i], buffer[i + 1]]);
                    data16.push(short);
                }
                let ip = ImageProcessor::<u16, Gray16>::new_volume(w, h, d, data16, Gray16::new());
                OutputProcessor::ShortProcessor(ip)
            }
            FileInfo::GRAY32_FLOAT => {
                let mut dataf32 = Vec::<f32>::new();
                let file = File::open(&filename.to_string()).expect("no file found");
                let mut reader = BufReader::new(file);
                let mut buffer = [0u8; 4];
                for i in 0..length {
                    if let Err(e) = reader.read_exact(&mut buffer) {
                        // if you know how many bytes are expected, then it's better not to rely on `UnexpectedEof`!
                        panic!("{}", e);
                    }
                    // or use `from_le_bytes()` depending on the byte-order
                    let float = f32::from_be_bytes(buffer);
                    dataf32.push(float);
                }
                let ip = ImageProcessor::<f32, Gray<f32>>::new_volume(
                    w,
                    h,
                    d,
                    dataf32,
                    Gray::<f32>::new(),
                );
                OutputProcessor::FloatProcessor(ip)
            }
            _ => OutputProcessor::Unknown(String::from("Unknown File Format")),
        }
    }

    /// Open a _ImageProcessor_ from Raw Binary File.
    /// There is no header in this file
    ///
    /// # Arguments
    ///
    /// * `filename` The file name
    /// * `w` The image width
    /// * `h` The image height
    /// * `ty` The image Type among the `FileInfo` constants.
    ///
    pub fn open_processor(filename: &str, w: u32, h: u32, ty: u32) -> OutputProcessor {
        FileOpener::open_volume(filename, w, h, 1, ty)
    }

    /// Open an Image Stack from Raw Binary File.
    /// There is no header in this file
    ///
    /// # Arguments
    ///
    /// * `filename` The file name
    /// * `w` The image width
    /// * `h` The image height
    /// * `ty` The image Type among the `FileInfo` constants.
    ///
    pub fn open_stack(filename: &str, w: u32, h: u32, ty: u32) -> OutputProcessor {
        // Read data
        let buffer = FileOpener::get_file_as_byte_vec(&filename.to_string());

        // Create corresponding ImageStack depending of content
        match ty {
            FileInfo::GRAY8 => {
                let stck = FileOpener::read_byte_stack(w, h, &buffer);
                OutputProcessor::ByteStack(stck)
            }
            FileInfo::GRAY16_UNSIGNED => {
                let mut data16 = Vec::<u16>::new();
                for i in (0..buffer.len()).step_by(2) {
                    // TODO: Check endianness????
                    let short = u16::from_be_bytes([buffer[i], buffer[i + 1]]);
                    data16.push(short);
                }
                let stck = FileOpener::read_u16_stack(w, h, &buffer);
                OutputProcessor::ShortStack(stck)
            }
            _ => OutputProcessor::Unknown(String::from("Unknown File Format")),
        }
    }

    // Restructure to separate the different slices
    //
    // # arguments
    //
    // * `data` - The vector of raw data
    // * `nb_slice` - The number of images in the stack
    //
    fn separate_slices(data: &Vec<u8>, nb_slice: usize) -> Vec<Vec<u8>> {
        let mut nb_pix_per_slice = data.len() / nb_slice as usize;
        let mut newbuffer: Vec<Vec<u8>> = vec![];
        for i in (0..data.len()).step_by(nb_pix_per_slice.try_into().unwrap()) {
            let mut temp: Vec<u8> = vec![];
            for j in i..(nb_pix_per_slice + i) {
                let mut u = (j) as usize;
                temp.push(data[u]);
            }
            newbuffer.push(temp);
        }
        return newbuffer;
    }

    // Restructure the vector into a vector of tuple (u8,u8,u8)
    //
    // # arguments
    //
    // * `nb_slice` - The number of images in the stack
    // * `image` - The vector with slices separated
    //
    fn slice_rgb(nb_slice: u32, image: Vec<Vec<u8>>) -> Vec<Vec<(u8, u8, u8)>> {
        let sum = image.len() * image[0].len() as usize;
        let mut nb_pix_per_slice = sum / nb_slice as usize;
        let mut buffer: Vec<Vec<(u8, u8, u8)>> = vec![];
        for i in 0..nb_slice {
            let mut temp: Vec<(u8, u8, u8)> = vec![];
            for j in (0..nb_pix_per_slice).step_by(3) {
                let iu = i as usize;
                let ju = j as usize;
                let mut tup = (image[iu][ju], image[iu][ju + 1], image[iu][ju + 2]);
                temp.push(tup);
            }
            buffer.push(temp);
        }
        return buffer;
    }

    // Read a raw file and put the data into a 8-bit image stack of
    //
    // # arguments
    //
    // * `height` - The height of the images
    // * `width` - The width of the images
    // * `slice` - The number of images in the stack
    // * `filename` - The name of the raw file
    //
    fn read_byte_stack(width: u32, height: u32, buffer: &Vec<u8>) -> ImageStack<u8, Gray8> {
        let nslices = buffer.len() / (width * height) as usize;
        //let data = FileOpener::separate_slices(buffer, nslices);
        let mut data = Vec::<Vec<u8>>::new();
        let mut start: usize = 0;
        let size: usize = (width * height) as usize;
        let mut end: usize = start + size;
        for i in 0..nslices {
            data.push(buffer[start..end].to_vec());
            start = end;
            end += size;
        }
        ImageStack::new(width, height, data, Gray8::new())
    }

    // Read a raw file and put he data into an image stack of 16bit (16bit not implemented)
    //
    // # arguments
    //
    // * `height` - The height of the images
    // * `width` - The width of the images
    // * `slice` - The number of images in the stack
    // * `filename` - The name of the raw file
    //
    fn read_u16_stack(height: u32, width: u32, buffer: &Vec<u8>) -> ImageStack<u16, Gray16> {
        let nslices = buffer.len() / (width * height * 2) as usize;
        let mut new_buffer: Vec<Vec<u16>> = vec![];
        let data = FileOpener::separate_slices(buffer, nslices);
        for j in 0..data.len() {
            let mut slice: Vec<u16> = vec![];
            for i in (0..data[j].len()).step_by(2) {
                let short = u16::from_be_bytes([data[j][i], data[j][i + 1]]);
                slice.push(short);
            }
            new_buffer.push(slice);
        }
        let mut stack = ImageStack::new(width, height, new_buffer, Gray16::new());
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

    // Read a raw file and put he data into an image stack of 32bit
    //
    // # arguments
    //
    // * `height` - The height of the images
    // * `width` - The width of the images
    // * `slice` - The number of images in the stack
    // * `filename` - The name of the raw file
    //
    fn read_float_stack(height: u32, width: u32, buffer: &Vec<u8>) -> ImageStack<f32, Gray32> {
        let nslices = buffer.len() / (width * height * 4) as usize;
        let mut new_buffer: Vec<Vec<f32>> = vec![];
        let data = FileOpener::separate_slices(buffer, nslices);
        for j in 0..data.len() {
            let mut slice: Vec<f32> = vec![];
            for i in (0..data[j].len()).step_by(4) {
                let short = f32::from_be_bytes([
                    data[j][i],
                    data[j][i + 1],
                    data[j][i + 2],
                    data[j][i + 3],
                ]);
                slice.push(short);
            }
            new_buffer.push(slice);
        }

        let mut stack = ImageStack::new(width, height, new_buffer, Gray32::new());
        return stack;
    }

    // Return a vector u8 containing the raw data of the image
    //
    // # arguments
    //
    // * `filename` - String containing the name of the raw file
    //
    fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
        let mut f = File::open(&filename).expect("no file found");
        let metadata = metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        buffer
    }
}

/*
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

}*
*/

#[cfg(test)]
mod tests {

    use super::*;

    use crate::image_processor::*;

    #[test]
    fn open_chessboard_u8_8x8_bin() {
        let proc =
            FileOpener::open_processor("./samples/chessboard_u8_8x8.bin", 8, 8, FileInfo::GRAY8);
        let answer = vec![
            255u8, 0u8, 255u8, 0u8, 255u8, 0u8, 255u8, 0u8, 0u8, 255u8, 0u8, 255u8, 0u8, 255u8,
            0u8, 255u8, 255u8, 0u8, 255u8, 0u8, 255u8, 0u8, 255u8, 0u8, 0u8, 255u8, 0u8, 255u8,
            0u8, 255u8, 0u8, 255u8, 255u8, 0u8, 255u8, 0u8, 255u8, 0u8, 255u8, 0u8, 0u8, 255u8,
            0u8, 255u8, 0u8, 255u8, 0u8, 255u8, 255u8, 0u8, 255u8, 0u8, 255u8, 0u8, 255u8, 0u8,
            0u8, 255u8, 0u8, 255u8, 0u8, 255u8, 0u8, 255u8,
        ];
        match proc {
            OutputProcessor::ByteProcessor(ip) => {
                assert!(ip.data().iter().zip(answer).all(|(a, b)| *a == b));
                //    assert!(ip.data().iter().all(|item| answer.contains(item)))
            }
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn open_chessboard_u16_8x8_bin() {
        let proc = FileOpener::open_processor(
            "./samples/chessboard_u16_8x8.bin",
            8,
            8,
            FileInfo::GRAY16_UNSIGNED,
        );
        let answer = vec![
            32768u16, 0u16, 32768u16, 0u16, 32768u16, 0u16, 32768u16, 0u16, 0u16, 32768u16, 0u16,
            32768u16, 0u16, 32768u16, 0u16, 32768u16, 32768u16, 0u16, 32768u16, 0u16, 32768u16,
            0u16, 32768u16, 0u16, 0u16, 32768u16, 0u16, 32768u16, 0u16, 32768u16, 0u16, 32768u16,
            32768u16, 0u16, 32768u16, 0u16, 32768u16, 0u16, 32768u16, 0u16, 0u16, 32768u16, 0u16,
            32768u16, 0u16, 32768u16, 0u16, 32768u16, 32768u16, 0u16, 32768u16, 0u16, 32768u16,
            0u16, 32768u16, 0u16, 0u16, 32768u16, 0u16, 32768u16, 0u16, 32768u16, 0u16, 32768u16,
        ];
        match proc {
            OutputProcessor::ShortProcessor(ip) => {
                assert!(ip.data().iter().zip(answer).all(|(a, b)| *a == b));
            }
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn open_chessboard_u8_8x8_as_u8stack_4x4x4() {
        let proc = FileOpener::open_stack("./samples/chessboard_u8_8x8.bin", 4, 4, FileInfo::GRAY8);
        let answer = vec![
            vec![
                255u8, 0u8, 255u8, 0u8, 255u8, 0u8, 255u8, 0u8, 0u8, 255u8, 0u8, 255u8, 0u8, 255u8,
                0u8, 255u8,
            ],
            vec![
                255u8, 0u8, 255u8, 0u8, 255u8, 0u8, 255u8, 0u8, 0u8, 255u8, 0u8, 255u8, 0u8, 255u8,
                0u8, 255u8,
            ],
            vec![
                255u8, 0u8, 255u8, 0u8, 255u8, 0u8, 255u8, 0u8, 0u8, 255u8, 0u8, 255u8, 0u8, 255u8,
                0u8, 255u8,
            ],
            vec![
                255u8, 0u8, 255u8, 0u8, 255u8, 0u8, 255u8, 0u8, 0u8, 255u8, 0u8, 255u8, 0u8, 255u8,
                0u8, 255u8,
            ],
        ];
        match proc {
            OutputProcessor::ByteStack(ip) => {
                assert_eq!(ip.n_slices(), 4);
                assert_eq!(ip.data().len(), 4);
                assert_eq!(ip.data()[0].len(), 16);
                for i in 0..4 {
                    assert!(ip.data()[i].iter().zip(&answer[i]).all(|(a, b)| *a == *b));
                }
            }
            _ => panic!("Wrong type"),
        }
    }
}
