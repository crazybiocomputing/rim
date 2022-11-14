//
//  RIM - Rust Image
//  Copyright (C) 2022  Jean-Christophe Taveau
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
// Group : SPR in real space / frequency space (fftw3)
// Authors : Léonard Brindel, Océane Dorémus, Léo Gillet
//

#![warn(unused_imports)]
#![warn(unused)]
#![warn(dead_code)]

use std::process::exit;
use rim::grayscale::Gray16;
use rim::image_processor::ImageProcessor;
use rim::image_stack::ImageStack;
use rim::io::image_reader::{FileOpener, OutputProcessor};
use rim::io::text_reader::TextReader;
use rim::results_table::ResultsTable;

// fn image_projection(ImageStack ist) -> Vec<ImageStack> {
//    let img_projection_stack: Vec<ImageStack> = Vec::new();
//    for _ in 10 {
//         img_projection_stack.push(ist);
//    }
//    return img_projection_stack;
// }
//
// fn image_stack_projection(ImageStacks ist_stack) -> Vec<ImageStacks> {
//     let all_img_projection: Vec<ImageStacks> = Vec::new();
//     for img_stack in ist_stack{
//         all_img_projection.push(image_projection(img_stack);
//     }
//     return all_img_projection;
// }

pub struct SPR {}

impl SPR {
    fn read_angles() -> ResultsTable {
        return TextReader::open_csv("samples/psi-theta-phi-50.csv", Option::Some(',')).unwrap();
    }

    fn read_image() -> Option<&'static ImageProcessor<u16, Gray16>> {
        let processor =  FileOpener::open_stack("samples/T1_head_128x128x128.tif", 128, 128, 128);
        if let OutputProcessor::ShortProcessor(ip) = &processor {
            return Some(ip)
        }
        return None
    }

    pub fn start() -> () {
        let angles: ResultsTable = Self::read_angles();
        let processor: Option<&ImageProcessor<u16, Gray16>> = Self::read_image();
        let ip: &Vec<u16>;

        match processor {
            Some(x) => {
                ip = x.data();
            }
            None => {
                exit(1);
            }
        }

        for val in ip {
            print!("{}", val);
        }
    }
}