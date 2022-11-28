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

use nalgebra::Matrix3;
use math::round::floor;
use std::f64::consts::PI;

pub fn rotation_X(angle: f64)-> Matrix3<f64> {
    let angle_rad = angle * PI / 180.0;
    let rotX = Matrix3::new(1.0,0.0,.0.0,
                                0.0, angle_rad.cos(), -angle_rad.sin(),
                                0.0, angle_rad.sin(), angle_rad.cos());
    return rotX
}

pub fn rotation_Y(angle: f64)-> Matrix3<f64> {
    let angle_rad = angle * PI / 180.0;
    let rotY = Matrix3::new(angle_rad.cos(), 0.0,angle_rad.sin(),
                                 0.0, 1.0, 0.0,
                                -angle_rad.sin(), 0.0, angle_rad.cos());
    return rotY
}

pub fn rotation_Z(angle: f64)-> Matrix3<f64> {
    let angle_rad = angle * PI / 180.0;
    let rotZ = Matrix3::new(angle_rad.cos(), -angle_rad.sin(), 0.0,
                                angle_rad.sin(), angle_rad.cos(), 0.0,
                                0.0, 0.0, 1.0);
    return rotZ
}

pub fn rotation(rot_list : $Vec<Matrix3>)-> Matrix3<F64> {
     let rotation: Matrix3<N> = rot_list[0]*rot_list[1]*rot[2];
     return rotation

}

pub fn bilinear_interpol(x: f64, y:f64, n_projections: &Image_Stack<X,T>)-> f64 {
    let x_bis: i64 = math::round::floor(x);
    let y_bis: i64 = math::round::floor(y);

    let X: f64 = (y_bis+1-y)*n_projection.data()[y_bis][x_bis] + (y -y_bis)*n_projection.data()[y_bis+1][x_bis];
    let Y: f64 = (y_bis+1-y)*n_projection.data()[y_bis][x_bis+1] + (y -y_bis)*n_projection.data()[y_bis+1][x_bis+1];
    let pixel_value: f64 = (x_bis+1-x)*X + (x-x_bis)*Y;
    return pixel_value;
}



