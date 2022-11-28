//
//  RIM - Rust Image
//  Copyright (C) 2022  Jean-Christophe Taveau.
//
//  self file is part of RIM
//
// self program is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// self program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with RIM.  If not, see <http://www.gnu.org/licenses/>.

/// Adapted from JS package
/// https://github.com/mattdesl/vecmath

const EPSILON: f64 = 0.000001;

#[derive(Clone, Copy)]
pub struct Matrix3 {
    val: [f64; 9],
}

impl Matrix3 {
    ///
    ///
    ///
    pub fn from_vec(v: Vec<f64>) -> Self {
        let a = v.try_into().unwrap_or_else(|v: Vec<f64>| {
            panic!("Expected a Vec of length 9 but it was {}", v.len())
        });
        Matrix3::from_array(a)
    }
    ///
    ///
    ///
    pub fn from_array(a: [f64; 9]) -> Self {
        let mut out = [0.0f64; 9];

        out[0] = a[0];
        out[1] = a[1];
        out[2] = a[2];
        out[3] = a[3];
        out[4] = a[4];
        out[5] = a[5];
        out[6] = a[6];
        out[7] = a[7];
        out[8] = a[8];

        Matrix3 { val: out }
    }

    ///
    /// Create an Identity Matrix
    ///
    pub fn identity() -> Self {
        let mut out = [0.0f64; 9];

        out[0] = 1.0;
        out[4] = 1.0;
        out[8] = 1.0;
        Matrix3 { val: out }
    }

    ///
    ///
    ///
    pub fn build(&self) -> Matrix3 {
        Matrix3 { val: self.val }
    }

    ///
    ///
    ///
    pub fn values(&self) -> &[f64; 9] {
        &self.val
    }
}
