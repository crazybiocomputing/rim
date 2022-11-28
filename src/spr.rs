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

use std::f64::consts::PI;
use std::process::exit;
use nalgebra::{Matrix, Matrix3, Rotation3};
use rim::grayscale::Gray16;
use rim::image_processor::ImageProcessor;
use rim::image_stack::ImageStack;
use rim::io::file_info::FileInfo;
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
//     for img_stack in ist_stack {
//         all_img_projection.push(image_projection(img_stack);
//     }
//     return all_img_projection;
// }

pub struct SPR {}

impl SPR {
    fn read_angles() -> ResultsTable {
        return TextReader::open_csv("samples/psi-theta-phi-50.csv", Option::Some(',')).unwrap();
    }

    pub fn start() -> () {
        let angles: ResultsTable = Self::read_angles();
        let proc = FileOpener::open_processor(
                "./samples/T1_head_128x128x128.tif",
                128, 128,
                FileInfo::GRAY16_UNSIGNED
            );
        if let OutputProcessor::ShortStack(ip) = proc {
            println!("{}x{}",ip.get_width(),ip.get_height());
        }

        let phi_table: Vec<f64> = angles.get_column_as_floats(String::from("Phi")).unwrap();
        let psi_table: Vec<f64> = angles.get_column_as_floats(String::from("Psi")).unwrap();
        let theta_table: Vec<f64> = angles.get_column_as_floats(String::from("Theta")).unwrap();

        for i in 0..phi_table.len() {
            let phi = phi_table[i]*(PI/180.0); // Rotation around Z
            let theta = theta_table[i]*(PI/180.0); // Rotation around Y
            let psi = psi_table[i]*(PI/180.0); // Rotation around Z
            // Step #1 - compute the resulting rotation matrix (RZ * RY * RZ)
            /*
                       Z                            Y                               Z
            [ cos(psi) sin(psi) 0 ]     [ cos(theta) 0 -sin(theta) ]    [ cos(phi) sin(phi) 0 ]
            [-sin(psi) cos(psi) 0 ]     [    0       1      0      ]    [-sin(phi) cos(phi) 0 ]
            [    0        0     1 ]     [ sin(theta) 0      1      ]    [    0        0     1 ]
             */

            let mat_psi: Matrix3<f64> = Matrix3::new(
                psi.cos(),      psi.sin(), 0.0,
                -psi.sin(),     psi.cos(), 0.0,
                0.0, 0.0, 1.0
            );

            let mat_psi: Rotation3<f64> = Rotation3::from_matrix(&mat_psi);

            let mat_theta: Matrix3<f64> = Matrix3::new(
                theta.cos(), 0.0, -theta.sin(),
                0.0, 1.0, 0.0,
                theta.sin(), 0.0, 1.0
            );

            let mat_phi: Matrix3<f64> = Matrix3::new(
                phi.cos(), phi.sin(), 0.0,
                -phi.sin(), phi.cos(), 0.0,
                0.0, 0.0, 1.0
            );

            let mat = mat_psi * mat_theta * mat_phi;
            // let rot: Rotation3::from_matrix(mat);
            let inv = mat.try_inverse().unwrap();
            // Step #2 -
            // let proj = stack.get_slice(i);
            // Step #3 - Backprojection
            
            // Step #4 - Apply rotation to backprojection
            // Step #5 - Add rotated backprojection to output
        }
        // Step #6 - Return output

    }
}