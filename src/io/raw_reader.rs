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

pub fn save_raw_file(name: &String, buffer: Vec<u8>){
    let filename = format!("{}.raw", name);
    let file = File::create(filename);
    file.expect("REASON").write_all(&buffer);
}

// TODO
/*
    essayer de read file en vecteur de float -> pas possible metadata seulement en u8
    read image depuis url (API ImageJ) -> format compliqué
    save sous format TIF -> trop compliqué à cause du format

*/