use std::env;

use rim::io::file_info::*;
use rim::io::image_reader::*;
use rim::io::text_reader::*;
// use rim::image_processor::*;
use rim::image_traits::Access;

fn help() {
   println!("usage:
rim-tool -i <input-file.bin> -s <width>x<height> -b <bpp> -p <angle-file.csv> -o <output-file.bin");
}

fn run(ifile: String,w: u32,h: u32,bpp: usize,csv: String,ofile: String) -> FloatProcessor{

    let typ : u32 = match bpp {
      8 => FileInfo::GRAY8,
      16 => FileInfo::GRAY16_UNSIGNED,
      32 => FileInfo::GRAY32_FLOAT,
      _ => FileInfo::UNKNOWN
    };

    let proc = FileOpener::open_processor(&ifile[..], w, h, typ);
    match proc {
        OutputProcessor::FloatProcessor(ip) => {
            // TODO
            println!("IP Information: {} {} {}",ip.get_width(),ip.get_height(),ip.get_bit_depth());
            for i in 0..ip.get_size() {
              println!("{:.2}",ip.get_pixel(i).unwrap());
            }
        }
        _ => panic!("Wrong type"),

    }ip

}

