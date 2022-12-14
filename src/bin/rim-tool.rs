use std::env;

use rim::io::file_info::*;
use rim::io::image_reader::*;
use rim::io::text_reader::*;
// use rim::image_processor::*;
use rim::image_traits::Access;

fn help() {
    println!(
        "usage:
rim-tool -i <input-file.bin> -s <width>x<height> -b <bpp> -p <angle-file.csv> -o <output-file.bin"
    );
}

fn run(ifile: String, w: u32, h: u32, bpp: usize, csv: String, ofile: String) {
    let typ: u32 = match bpp {
        8 => FileInfo::GRAY8,
        16 => FileInfo::GRAY16_UNSIGNED,
        32 => FileInfo::GRAY32_FLOAT,
        _ => FileInfo::UNKNOWN,
    };

    let proc = FileOpener::open_processor(&ifile[..], w, h, typ);
    match proc {
        OutputProcessor::FloatProcessor(ip) => {
            // TODO
            println!(
                "IP Information: {} {} {}",
                ip.get_width(),
                ip.get_height(),
                ip.get_bit_depth()
            );
            for i in 0..ip.get_size() {
                println!("{:.2}", ip.get_pixel(i).unwrap());
            }
        }
        _ => panic!("Wrong type"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input = String::new();
    let mut output = String::new();
    let mut params = String::new();
    let mut meta: (u32, u32, usize) = (0, 0, 0);

    // Loop over the arguments
    match args.len() {
        1 => {
            println!("Arguments required.");
            help();
        }
        11 => {
            for i in (1..args.len()).step_by(2) {
                let arg = &args[i];
                let val = &args[i + 1];
                // Parse
                match &arg[..] {
                    "-i" => input = val.to_string(),
                    "-d" | "--dim" => {
                        let words: Vec<&str> = val.split('x').collect();
                        meta.0 = words[0].parse::<u32>().unwrap() as u32;
                        meta.1 = words[1].parse::<u32>().unwrap() as u32;
                    }
                    "-b" | "--bpp" => meta.2 = val.parse::<u32>().unwrap() as usize,
                    "-p" => params = val.to_string(),
                    "-o" => output = val.to_string(),
                    _ => panic!("Unknown argument"),
                }
            }
            run(input, meta.0, meta.1, meta.2, params, output);
        }
        _ => {
            println!("Missing arguments. {}", args.len());
            help();
        } // println!("{} {} {}",i,args[i], args[i+1]);
    }
}
