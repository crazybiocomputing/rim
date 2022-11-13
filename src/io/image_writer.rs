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


/// Save the data of an image stack of 8bit into raw file
///
/// # arguments
///
/// * `stack` - The ByteStack containing the data
/// * `filename` - Name of the created file
///
pub fn save_byte_stack(stack: ImageStack<u8, Gray8>, filename: &String) {
    let mut raw_data: Vec<u8> = vec![];

    let slices = stack.data();

    for i in 0..slices.len() {
        for j in 0..slices[i].len() {
            raw_data.push(slices[i][j]);
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
pub fn save_float_stack(stack: ImageStack<f32, Gray32>, filename: &String) {
    let mut raw_data: Vec<u8> = vec![];

    let slices = stack.data();

    for i in 0..slices.len() {
        for j in 0..slices[i].len() {
            let long = slices[i][j].to_be_bytes();
            for k in 0..long.len() {
                raw_data.push(long[k]);
            }
        }
    }
    save_raw_file(filename, raw_data);
}
/*
/// Save the data of an image stack of RGB into raw file
    ///
    /// # arguments
    ///
    /// * `stack` - The ByteStack containing the data
    /// * `filename` - Name of the created file
    ///
    pub fn save_color_stack(stack:ImageStack<T,C>, filename:&String){
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

*/
