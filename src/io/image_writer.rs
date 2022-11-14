//
//  RIM - Rust IMage
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
//
// Authors:  Allain Anaelle, Texier Louis

use std::fs::metadata;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

use crate::color_space::ColorSpace;
use crate::gray_processor::ip_gray;
use crate::grayscale::*;
use crate::image_processor::*;
use crate::image_stack::*;
use crate::image_traits::*;
use crate::io::file_info::*;
use crate::meta_data::MetaData;
use crate::pixel::PixelType;
use crate::io::image_reader::OutputProcessor;


///
/// FileSaver is a utility class to saw raw binary file without header.
///
/// # Example
///
/// ```rust
///     let proc = FileOpener::open_processor(
///         "./samples/chessboard_u8_8x8.bin",
///         8, 8,
///         FileInfo::GRAY8
///     );
///     
///     FileSaver::save_processor("./samples/test", FileInfo::GRAY8, proc);
///     }
/// `

pub struct FileSaver {}

impl FileSaver {
    /// Save a _3D ImageProcessor_ from Raw|Binary File.
    /// There is no header in this file
    ///
    /// # Arguments
    ///
    /// * `filename` The output file name
    /// * `ty` The image Type among the `FileInfo` constants.
    /// * `image` The OutputProcessor struct containing the image
    ///

    pub fn save_volume(filename: &str, ty: u32, image: OutputProcessor) {
        match ty {
            FileInfo::GRAY8 => {
                FileSaver::save_byte_processor(image, filename);
            }
            FileInfo::GRAY16_UNSIGNED => {
                FileSaver::save_u16_processor(image, filename)
            }
            FileInfo::GRAY32_FLOAT => {
                FileSaver::save_float_processor(image, filename)
            }
            _ => println!("Type not found !"),
        }
    }

    /// Save an ImageProcessor from Raw|Binary File.
    /// There is no header in this file
    ///
    /// # Arguments
    ///
    /// * `filename` The output file name
    /// * `ty` The image Type among the `FileInfo` constants.
    /// * `image` The OutputProcessor struct containing the image
    ///
    pub fn save_processor(filename: &str, ty: u32, image: OutputProcessor) {
        FileSaver::save_volume(filename, ty, image)
    }

    /// Save an ImageStack from Raw|Binary File.
    /// There is no header in this file
    ///
    /// # Arguments
    ///
    /// * `filename` The output file name
    /// * `ty` The image Type among the `FileInfo` constants.
    /// * `image` The OutputProcessor struct containing the image
    ///
    pub fn save_stack(filename: &str, ty: u32, image: OutputProcessor) {
        match ty {
            FileInfo::GRAY8 => {
                FileSaver::save_byte_stack(image, filename);
            }
            FileInfo::GRAY16_UNSIGNED => {
                FileSaver::save_u16_stack(image, filename)
            }
            FileInfo::GRAY32_FLOAT => {
                FileSaver::save_float_stack(image, filename)
            }
            _ => println!("Type not found !"),
        }
    }



    /// Save the data of an image processor of 8bit into raw file
    ///
    /// # arguments
    ///
    /// * `stack` - The ByteProcessor containing the data
    /// * `filename` - Name of the created file
    ///
    pub fn save_byte_processor(stack: OutputProcessor, filename: &str) {
        let mut raw_data: Vec<u8> = vec![];
        if let OutputProcessor::ByteProcessor(ip) = &stack {
            let proc = ip.clone();

            let slices = proc.data();

            for i in 0..slices.len() {
                raw_data.push(slices[i]);
            }
            FileSaver::save_raw_file(filename, raw_data);
        }
    }

    /// Save the data of an image stack of 16bit into raw file
    ///
    /// # arguments
    ///
    /// * `stack` - The ByteStack containing the data
    /// * `filename` - Name of the created file
    ///
    pub fn save_u16_processor(stack: OutputProcessor, filename: &str) {
        let mut raw_data: Vec<u8> = vec![];
        if let OutputProcessor::ShortProcessor(ip) = &stack {
            let proc = ip.clone();

            let slices = proc.data();

            for i in 0..slices.len() {
                let long = slices[i].to_be_bytes();
                for k in 0..long.len() {
                    raw_data.push(long[k]);
                }
            }
        
            FileSaver::save_raw_file(filename, raw_data);
        }
    }


    /// Save the data of an image processor of 32bit into raw file
    ///
    /// # arguments
    ///
    /// * `stack` - The ByteStack containing the data
    /// * `filename` - Name of the created file
    ///
    pub fn save_float_processor(stack: OutputProcessor, filename: &str) {
        let mut raw_data: Vec<u8> = vec![];
        if let OutputProcessor::FloatProcessor(ip) = &stack {

            let proc = ip.clone();

            let slices = proc.data();

            for i in 0..slices.len() {
                let long = slices[i].to_be_bytes();
                for k in 0..long.len() {
                    raw_data.push(long[k]);
                }
            }
            FileSaver::save_raw_file(filename, raw_data);
        }
    }

    /// Save the data of an image stack of 8bit into raw file
    ///
    /// # arguments
    ///
    /// * `stack` - The ByteStack containing the data
    /// * `filename` - Name of the created file
    ///
    pub fn save_byte_stack(stack: OutputProcessor, filename: &str) {
        let mut raw_data: Vec<u8> = vec![];
        if let OutputProcessor::ByteStack(ip) = &stack {

            let proc = ip.clone();

            let slices = proc.data();

            for i in 0..slices.len() {
                for j in 0..slices[i].len() {
                    raw_data.push(slices[i][j]);
                }
            }
            FileSaver::save_raw_file(filename, raw_data);
        }
    }

    /// Save the data of an image stack of 16bit into raw file
    ///
    /// # arguments
    ///
    /// * `stack` - The ByteStack containing the data
    /// * `filename` - Name of the created file
    ///
    pub fn save_u16_stack(stack: OutputProcessor, filename: &str) {
        let mut raw_data: Vec<u8> = vec![];
        if let OutputProcessor::ShortStack(ip) = &stack {

            let proc = ip.clone();

            let slices = proc.data();

            for i in 0..slices.len() {
                for j in 0..slices[i].len() {
                    let long = slices[i][j].to_be_bytes();
                    for k in 0..long.len() {
                        raw_data.push(long[k]);
                    }
                }
            }
            FileSaver::save_raw_file(filename, raw_data);
        }
    }


    /// Save the data of an image stack of 32bit into raw file
    ///
    /// # arguments
    ///
    /// * `stack` - The ByteStack containing the data
    /// * `filename` - Name of the created file
    ///
    pub fn save_float_stack(stack: OutputProcessor, filename: &str) {
        let mut raw_data: Vec<u8> = vec![];
        if let OutputProcessor::FloatStack(ip) = &stack {

            let proc = ip.clone();

            let slices = proc.data();

            for i in 0..slices.len() {
                for j in 0..slices[i].len() {
                    let long = slices[i][j].to_be_bytes();
                    for k in 0..long.len() {
                        raw_data.push(long[k]);
                    }
                }
            }
            FileSaver::save_raw_file(filename, raw_data);
        }
    }


    /// Write the raw data in a raw file
    ///
    /// # arguments
    ///
    /// * `name` - Name of the final file
    /// * `buffer` - Vector of data for writting in the file
    ///
    pub fn save_raw_file(name: &str, buffer: Vec<u8>){
        let filename = format!("{}.bin", name);
        let file = File::create(filename);
        file.expect("REASON").write_all(&buffer);
    }


}
