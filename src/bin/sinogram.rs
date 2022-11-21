use rim::cryoem::sinogram::*;
use rim::float_processor::FloatProcessor;
use rim::color_space::ColorSpace;
use rim::grayscale::Gray;
use std::env;
use rim::io::image_writer::*;
use rim::io::file_info::*;
use rim::io::image_reader::*;
use rim::io::text_reader::*;

// Comamnde : cargo run --bin sinogram -- -i "path_image" -d 256x256 -b 32 -p "path_ficher_angle" -o "path_sortie_image"

// Exemle:  cargo run --bin sinogram -- -i ./src/sino.bin -d 256x256 -b 32 -p ./src/angles.csv -o ./test

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input = String::new();
    let mut output = String::new();
    let mut params = String::new();
    let mut meta : (u32,u32,usize) = (0,0,0);
    
    // Loop over the arguments
    match args.len() {
        1 => {
            println!("Arguments required.");
           
        },
        11 => {
            for i in (1..args.len()).step_by(2) {
                let arg = &args[i];
                let val = &args[i+1];
                // Parse 
                match &arg[..] {
                "-i" => input = val.to_string(),
                "-d" | "--dim" => {
                    let words : Vec<&str> = val.split('x').collect();
                    meta.0 = words[0].parse::<u32>().unwrap() as u32;
                    meta.1 = words[1].parse::<u32>().unwrap() as u32;
                },
                "-b" | "--bpp" => meta.2 = val.parse::<u32>().unwrap() as usize,
                "-p" => params = val.to_string(),
                "-o" => output = val.to_string(),
                _ => panic!("Unknown argument")
                }
            }
            let ip =import_image(&input,meta.0,meta.1,meta.2,params,&output);
            let ip2 = Sinogram::new_in_range(&ip,0.0,180.0,45.0);
            let op =OutputProcessor::FloatProcessor(ip2);
            FileSaver::save_processor(&output, FileInfo::GRAY32_FLOAT, op);

        }
        _ => {
            println!("Missing arguments. {}",args.len());
          
        }
    }
}



