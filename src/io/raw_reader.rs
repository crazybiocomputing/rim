v//
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

use std::fs::File;
use std::io::Read;
use std::fs::metadata;
use std::io::Write;

/// Return a vector u8 containing the raw data of the image
///
/// # arguments
///
/// * `filename` - String containing the name of the raw file
///
pub fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

/// Write the raw data in a raw file
///
/// # arguments
///
/// * `name` - Name of the final file
/// * `buffer` - Vector of data for writting in the file
///
pub fn save_raw_file(name: &String, buffer: Vec<u8>){
    let filename = format!("{}.raw", name);
    let file = File::create(filename);
    file.expect("REASON").write_all(&buffer);
}//
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

use std::fs::File;
use std::io::Read;
use std::fs::metadata;
use std::io::Write;

/// Return a vector u8 containing the raw data of the image
///
/// # arguments
///
/// * `filename` - String containing the name of the raw file
///
pub fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

/// Write the raw data in a raw file
///
/// # arguments
///
/// * `name` - Name of the final file
/// * `buffer` - Vector of data for writting in the file
///
pub fn save_raw_file(name: &String, buffer: Vec<u8>){
    let filename = format!("{}.raw", name);
    let file = File::create(filename);
    file.expect("REASON").write_all(&buffer);
}
