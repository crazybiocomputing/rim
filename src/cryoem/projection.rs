//
//  RIM - Rust Image
//  Copyright (C) 2022  Jean-Christophe Taveau.
//
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
// Authors: Luna Meyer, Louis Montagne


use crate::image_stack::Image_Stack;
use crate::grayscale::Gray;
use crate::vecmath::*;
use std::f64::consts::PI;
use nalgebra;

pub fn back_projection3D(phi: f32, theta: f32, psi: f32, n_projections: &Image_Stack<T,C>) -> VolumeProcessor<T,C> {
    //Create new cube filled with O as pixel values
    let data = vec![vec![vec![0 as f64;n_projections.get_height()];n_projections.get_width()]; n_projections.n_slices()];
    let dim: usize = n_projections.get_width();
    let mut vol = VolumeProcessor(dim, dim, dim, data, Gray::<f32>::new());

    //Rotation matrix
    let rot_mats: Vec<Matrix3> = vec![rotation_Z(phi), rotation_Y(theta),rotation_Z(psi)];
    let rotation: Matrix3<f64> = rotation(rot_mats);
    let invert: Matrix3<f64> = rotation.try_inverse().unwrap();

    for n in 0..n_projections.n_slices(){
        for z in 0..dim {
            for y in 0..dim {
                for x in 0..dim{
                    let mut position: Vec<f64> = vec![x,y,z]*invert;
                    //Interpolation
                    vol.data()[z][y][x]+=  bilinear_interpol(position[0], position[1], n_projections);
                }
            }
        }
    }
    return vol;
}
