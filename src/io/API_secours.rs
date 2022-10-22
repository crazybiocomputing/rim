//  RIM - Rust IMage
//  Copyright (&self,C) 2022  Jean-Christophe Taveau, Anaëlle Allain, Louis Texier.
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


use std::*;
mod classe;
use classe::*;
mod raw_reader;
use raw_reader::*;

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


/// Restructure to separate the different slices
///
/// # arguments
///
/// * `file` - Image object
///
fn cut_by_slice(file:&ImageReader) ->Vec<Vec<u8>>
{
    let nb_pix_per_slice = file.len()/ file.slice();
    let mut newbuffer:Vec<Vec<u8>> = vec![];
    for i in (0..file.len()).step_by(nb_pix_per_slice.try_into().unwrap())
    {
        let mut temp:Vec<u8> = vec![];
        for j in i..(nb_pix_per_slice+i)
        {
            let mut u = j as usize;
            temp.push(file.buffer()[u].clone());
        }
        newbuffer.push(temp);
    }
    return newbuffer;
}



/// Separate the pixels for 16bits images
///
/// # arguments
///
/// * `file` - Image object
/// * `image` - Data that will be modified
///
fn cut_by_pixels_16(file:&ImageReader, image:Vec<Vec<u8>>) -> Vec<Vec<(u8,u8)>>
{
    let nb_pix_per_slice = file.len()/ file.slice();
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



/// Separate the pixels for 32bits images
///
/// # arguments
///
/// * `file` - Image object
/// * `image` - Data that will be modified
///
fn cut_by_pixels_32(file:&ImageReader, image:Vec<Vec<u8>>) -> Vec<Vec<(u8,u8, u8, u8)>>
{
    let nb_pix_per_slice = file.len()/ file.slice();
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



/// Separate the pixels for RGB images
///
/// # arguments
///
/// * `file` - Image object
/// * `image` - Data that will be modified
///
fn cut_pixels_rgb(file:&ImageReader, image:Vec<Vec<u8>>) -> Vec<Vec<(u8,u8,u8)>>
{
    let nb_pix_per_slice = file.len()/ file.slice();
    let mut buffer:Vec<Vec<(u8,u8,u8)>> = vec![];
    for i in 0..file.slice()
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
        fn main() {
            
            let args: Vec<String> = env::args().collect();
            let filename = &args[1].to_string();
            let test = get_file_as_byte_vec(filename);
            let cut = ImageReader::new (2, 2, 2, &"8bit".to_string(), test);
            let testcut = sorting_images(&cut);
            let bidule = get_enimage8(testcut);
            println!("bidule {:?}", bidule);
        }
        
    }
    return buffer;
}




/// Return a result depending on the type of images and separate each pixels
///
/// # arguments
///
/// * `file` - Image object
///
fn sorting_images (file:&ImageReader) -> results//-> Box<dyn Fn() + Send + 'static>
{
    let mut image:Vec<Vec<u8>> = vec![file.buffer().clone()];
    let imageclone:Vec<Vec<u8>> = image.clone();
    if file.slice() > 1 
    {
        image = cut_by_slice(&file);
        //println!("cut by slice {:?}", image);
    }

    let mut image_result = results::enimage8(imageclone);

    if file.fi() == "16bit"
    {
        let cutimage16 = cut_by_pixels_16(&file, image);
        println!("cut image 16 {:?}", &cutimage16);
        image_result = results::enimage16(cutimage16);
        return image_result;
        
    }

    else if file.fi() == "32bit"
    {
        let cutimage32 = cut_by_pixels_32(&file, image);
        println!("cut image 32 {:?}", cutimage32);
        image_result = results::enimage32(cutimage32);
        return image_result;

    }

    else if file.fi() == "rgb"
    {
        let cutimagergb = cut_pixels_rgb(&file, image);
        println!("cut image rgb {:?}", cutimagergb);
        image_result = results::enimagergb(cutimagergb);
        return image_result;
    }

    return image_result;

}


enum results
{
    enimage8(Vec<Vec<u8>>),
    enimage16(Vec<Vec<(u8,u8)>>), 
    enimage32(Vec<Vec<(u8,u8, u8, u8)>>), 
    enimagergb(Vec<Vec<(u8,u8,u8)>>),
}


/// Get the vector of data for 8 bits after sorting
///
/// # arguments
///
/// * `enu` - Result of the sorting
///
fn get_enimage8(enu:results) -> Vec<Vec<u8>>
{
    if let results::enimage8(data) = &enu{
        let image = data.clone();
        return image;
    }
    let empty:Vec<Vec<u8>>=vec![];
    return empty;
}




/// Get the vector of data for 16 bits after sorting
///
/// # arguments
///
/// * `enu` - Result of the sorting
///
fn get_enimage16(enu:results) -> Vec<Vec<(u8,u8)>>
{
    if let results::enimage16(data) = &enu{
        let image = data.clone();
        return image;
    }
    let empty:Vec<Vec<(u8,u8)>>=vec![];
    return empty;
}



/// Get the vector of data for 32 bits after sorting
///
/// # arguments
///
/// * `enu` - Result of the sorting
///
fn get_enimage32(enu:results) -> Vec<Vec<(u8,u8,u8,u8)>>
{
    if let results::enimage32(data) = &enu{
        let image = data.clone();
        return image;
    }
    let empty:Vec<Vec<(u8,u8,u8,u8)>>=vec![];
    return empty;
}


/// Get the vector of data for RGB after sorting
///
/// # arguments
///
/// * `enu` - Result of the sorting
///
fn get_enimagergb(enu:results) -> Vec<Vec<(u8, u8, u8)>>
{
    if let results::enimagergb(data) = &enu{
        let image = data.clone();
        return image;
    }
    let empty:Vec<Vec<(u8, u8, u8)>>=vec![];
    return empty;
}
