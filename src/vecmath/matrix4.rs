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

use crate::vecmath::vector3::Vector3;
use std::fmt::{self, Display, Formatter};

///
/// f64 4 by 4 Matrix like those used in openGL and variants.
///
/// Adapted from JS package
/// https://github.com/mattdesl/vecmath

const EPSILON: f64 = 0.000001;

#[derive(Clone, Copy)]
pub struct Matrix4 {
    val: [f64; 16],
}

impl Matrix4 {
    ///
    /// Create a zero-filled 4 by 4 Matrix
    ///
    pub fn new() -> Self {
        Matrix4 { val: [0.0f64; 16] }
    }


    ///
    /// Create a 4 by 4 matrix from a 16-element Vec
    ///
    pub fn from_vec(v: Vec<f64>) -> Self {
        let a = v.try_into().unwrap_or_else(|v: Vec<f64>| {
            panic!("Expected a Vec of length 16 but it was {}", v.len())
        });
        Matrix4::from_array(a)
    }
    ///
    ///
    ///
    pub fn from_array(a: [f64; 16]) -> Self {
        let mut out = [0.0f64; 16];

        out[0] = a[0];
        out[1] = a[1];
        out[2] = a[2];
        out[3] = a[3];
        out[4] = a[4];
        out[5] = a[5];
        out[6] = a[6];
        out[7] = a[7];
        out[8] = a[8];
        out[9] = a[9];
        out[10] = a[10];
        out[11] = a[11];
        out[12] = a[12];
        out[13] = a[13];
        out[14] = a[14];
        out[15] = a[15];

        Matrix4 { val: out }
    }

    ///
    /// Create an Identity Matrix
    ///
    pub fn identity() -> Self {
        let mut out = [0.0f64; 16];

        out[0] = 1.0;
        out[5] = 1.0;
        out[10] = 1.0;
        out[15] = 1.0;
        Matrix4 { val: out }
    }

    ///
    ///
    ///
    pub fn build(&self) -> Matrix4 {
        Matrix4 { val: self.val }
    }

    ///
    /// The first element of the first row.
    ///
    pub fn m00(&self) -> f64 {
      self.val[0]
    }

    ///
    /// The second element of the first row.
    ///
    pub fn m01(&self) -> f64 {
      self.val[1]
    }

    ///
    /// The third element of the first row.
    ///
    pub fn m02(&self) -> f64 {
      self.val[2]
    }

    ///
    /// The fourth element of the first row.
    ///
    pub fn m03(&self) -> f64 {
      self.val[3]
    }

    ///
    /// The first element of the second row.
    ///
    pub fn m10(&self) -> f64 {
      self.val[4]
    }

    ///
    /// The second element of the second row.
    ///
    pub fn m11(&self) -> f64 {
      self.val[5]
    }

    ///
    /// The third element of the second row.
    ///
    pub fn m12(&self) -> f64 {
      self.val[6]
    }

    ///
    /// The fourth element of the second row.
    ///
    pub fn m13(&self) -> f64 {
      self.val[7]
    }

    ///
    /// The first element of the third row.
    ///
    pub fn m20(&self) -> f64 {
      self.val[8]
    }

    ///
    /// The second element of the third row.
    ///
    pub fn m21(&self) -> f64 {
      self.val[9]
    }

    ///
    /// The third element of the third row.
    ///
    pub fn m22(&self) -> f64 {
      self.val[10]
    }

    ///
    /// The fourth element of the third row.
    ///
    pub fn m23(&self) -> f64 {
      self.val[11]
    }

    ///
    /// The first element of the fourth row.
    ///
    pub fn m30(&self) -> f64 {
      self.val[12]
    }

    ///
    /// The second element of the fourth row.
    ///
    pub fn m31(&self) -> f64 {
      self.val[13]
    }

    ///
    /// The third element of the fourth row.
    ///
    pub fn m32(&self) -> f64 {
      self.val[14]
    }

    ///
    /// The fourth element of the fourth row.
    ///
    pub fn m33(&self) -> f64 {
      self.val[15]
    }

    ///
    /// Get the matrix values as a 1D array.
    ///
    pub fn values(&self) -> &[f64; 16] {
        &self.val
    }


    ///
    /// Transpose
    ///
    pub fn transpose(&mut self) -> &mut Self {
        // Temporary values
        let a01 = self.val[1];
        let a02 = self.val[2];
        let a03 = self.val[3];
        let a12 = self.val[6];
        let a13 = self.val[7];
        let a23 = self.val[11];

        self.val[1] = self.val[4];
        self.val[2] = self.val[8];
        self.val[3] = self.val[12];
        self.val[4] = a01;
        self.val[6] = self.val[9];
        self.val[7] = self.val[13];
        self.val[8] = a02;
        self.val[9] = a12;
        self.val[11] = self.val[14];
        self.val[12] = a03;
        self.val[13] = a13;
        self.val[14] = a23;
        self
    }

    ///
    /// Invert
    ///
    pub fn invert(&mut self) -> &mut Self {
        let a00 = self.val[0];
        let a01 = self.val[1];
        let a02 = self.val[2];
        let a03 = self.val[3];
        let a10 = self.val[4];
        let a11 = self.val[5];
        let a12 = self.val[6];
        let a13 = self.val[7];
        let a20 = self.val[8];
        let a21 = self.val[9];
        let a22 = self.val[10];
        let a23 = self.val[11];
        let a30 = self.val[12];
        let a31 = self.val[13];
        let a32 = self.val[14];
        let a33 = self.val[15];

        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;

        // Calculate the determinant
        let mut det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;

        if det == 0.0 {
            panic!("Determinant equal to 0.0");
        }
        det = 1.0 / det;

        self.val[0] = (a11 * b11 - a12 * b10 + a13 * b09) * det;
        self.val[1] = (a02 * b10 - a01 * b11 - a03 * b09) * det;
        self.val[2] = (a31 * b05 - a32 * b04 + a33 * b03) * det;
        self.val[3] = (a22 * b04 - a21 * b05 - a23 * b03) * det;
        self.val[4] = (a12 * b08 - a10 * b11 - a13 * b07) * det;
        self.val[5] = (a00 * b11 - a02 * b08 + a03 * b07) * det;
        self.val[6] = (a32 * b02 - a30 * b05 - a33 * b01) * det;
        self.val[7] = (a20 * b05 - a22 * b02 + a23 * b01) * det;
        self.val[8] = (a10 * b10 - a11 * b08 + a13 * b06) * det;
        self.val[9] = (a01 * b08 - a00 * b10 - a03 * b06) * det;
        self.val[10] = (a30 * b04 - a31 * b02 + a33 * b00) * det;
        self.val[11] = (a21 * b02 - a20 * b04 - a23 * b00) * det;
        self.val[12] = (a11 * b07 - a10 * b09 - a12 * b06) * det;
        self.val[13] = (a00 * b09 - a01 * b07 + a02 * b06) * det;
        self.val[14] = (a31 * b01 - a30 * b03 - a32 * b00) * det;
        self.val[15] = (a20 * b03 - a21 * b01 + a22 * b00) * det;

        self
    }

    ///
    /// Calculate Determinant
    ///
    pub fn determinant(&mut self) -> f64 {
        let a00 = self.val[0];
        let a01 = self.val[1];
        let a02 = self.val[2];
        let a03 = self.val[3];
        let a10 = self.val[4];
        let a11 = self.val[5];
        let a12 = self.val[6];
        let a13 = self.val[7];
        let a20 = self.val[8];
        let a21 = self.val[9];
        let a22 = self.val[10];
        let a23 = self.val[11];
        let a30 = self.val[12];
        let a31 = self.val[13];
        let a32 = self.val[14];
        let a33 = self.val[15];

        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;

        // Calculate the determinant
        b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06
    }

   ///
   /// Sets the value of this matrix to the matrix sum of matrices m1 and m2. 
   /// 
   /// # Arguments
   /// * `m1` the first matrix 
   /// * `m2` the second matrix 
   ///
    pub fn add(&mut self, m1: Matrix4, m2: Matrix4) {
        // note this is alias safe.
        self.set(
            m1.m00() + m2.m00(),
            m1.m01() + m2.m01(),
            m1.m02() + m2.m02(),
            m1.m03() + m2.m03(),
            m1.m10() + m2.m10(),
            m1.m11() + m2.m11(),
            m1.m12() + m2.m12(),
            m1.m13() + m2.m13(),
            m1.m20() + m2.m20(),
            m1.m21() + m2.m21(),
            m1.m22() + m2.m22(),
            m1.m23() + m2.m23(),
            m1.m30() + m2.m30(),
            m1.m31() + m2.m31(),
            m1.m32() + m2.m32(),
            m1.m33() + m2.m33()
            );
    }

    ///
    /// Sets the value of this matrix to sum of itself and matrix m1. 
    /// 
    /// # Arguments
    /// * `m1` the other matrix 
    ///
    pub fn add_mut(&mut self, m1: Matrix4) -> &mut Self {
        self.val[0] += m1.m00(); self.val[0] += m1.m01(); self.val[0] += m1.m02(); self.val[0] += m1.m03();
        self.val[0] += m1.m10(); self.val[0] += m1.m11(); self.val[0] += m1.m12(); self.val[0] += m1.m13();
        self.val[0] += m1.m20(); self.val[0] += m1.m21(); self.val[0] += m1.m22(); self.val[0] += m1.m23();
        self.val[0] += m1.m30(); self.val[0] += m1.m31(); self.val[0] += m1.m32(); self.val[0] += m1.m33();
        self
    }

    ///
    /// Sets the value of this matrix to the matrix difference
    /// of matrices m1 and m2. 
    ///
    /// # Arguments
    /// * `m1` the first matrix 
    /// * `m2` the second matrix 
    ///
    pub fn subtract(&mut self, m1: Matrix4, m2: Matrix4) {
      // note this is alias safe.
      self.set(
          m1.m00() - m2.m00(),
          m1.m01() - m2.m01(),
          m1.m02() - m2.m02(),
          m1.m03() - m2.m03(),
          m1.m10() - m2.m10(),
          m1.m11() - m2.m11(),
          m1.m12() - m2.m12(),
          m1.m13() - m2.m13(),
          m1.m20() - m2.m20(),
          m1.m21() - m2.m21(),
          m1.m22() - m2.m22(),
          m1.m23() - m2.m23(),
          m1.m30() - m2.m30(),
          m1.m31() - m2.m31(),
          m1.m32() - m2.m32(),
          m1.m33() - m2.m33()
      );
    }

    ///
    /// Sets the value of this matrix to the matrix difference of itself
    /// and matrix m1 (this = this - m1). 
    ///
    /// # Arguments
    /// * `m1` the other matrix 
    ///
    pub fn subtract_mut(&mut self,  m1: Matrix4) -> &mut Self {
        self.val[0] -= m1.m00(); self.val[0] -= m1.m01(); self.val[0] -= m1.m02(); self.val[0] -= m1.m03();
        self.val[0] -= m1.m10(); self.val[0] -= m1.m11(); self.val[0] -= m1.m12(); self.val[0] -= m1.m13();
        self.val[0] -= m1.m20(); self.val[0] -= m1.m21(); self.val[0] -= m1.m22(); self.val[0] -= m1.m23();
        self.val[0] -= m1.m30(); self.val[0] -= m1.m31(); self.val[0] -= m1.m32(); self.val[0] -= m1.m33();
        self
    }
    
    ///
    /// Sets the value of this matrix to the result of multiplying
    /// the two argument matrices together. 
    /// * `m1` the first matrix 
    /// * `m2` the second matrix 
    ///
    pub fn multiply(&mut self, m1: Matrix4, m2: Matrix4) -> &mut Self {
        // alias-safe way
        self.val[0]  = m1.m00() *m2.m00() + m1.m01() *m2.m10() + m1.m02() *m2.m20() + m1.m03() *m2.m30();
        self.val[1]  = m1.m00() *m2.m01() + m1.m01() *m2.m11() + m1.m02() *m2.m21() + m1.m03() *m2.m31();
        self.val[2]  = m1.m00() *m2.m02() + m1.m01() *m2.m12() + m1.m02() *m2.m22() + m1.m03() *m2.m32();
        self.val[3]  = m1.m00() *m2.m03() + m1.m01() *m2.m13() + m1.m02() *m2.m23() + m1.m03() *m2.m33();

        self.val[4]  = m1.m10() *m2.m00() + m1.m11() *m2.m10() + m1.m12() *m2.m20() + m1.m13() *m2.m30();
        self.val[5]  = m1.m10() *m2.m01() + m1.m11() *m2.m11() + m1.m12() *m2.m21() + m1.m13() *m2.m31();
        self.val[6]  = m1.m10() *m2.m02() + m1.m11() *m2.m12() + m1.m12() *m2.m22() + m1.m13() *m2.m32();
        self.val[7]  = m1.m10() *m2.m03() + m1.m11() *m2.m13() + m1.m12() *m2.m23() + m1.m13() *m2.m33();

        self.val[8]  = m1.m20() *m2.m00() + m1.m21() *m2.m10() + m1.m22() *m2.m20() + m1.m23() *m2.m30();
        self.val[9]  = m1.m20() *m2.m01() + m1.m21() *m2.m11() + m1.m22() *m2.m21() + m1.m23() *m2.m31();
        self.val[10]  = m1.m20() *m2.m02() + m1.m21() *m2.m12() + m1.m22() *m2.m22() + m1.m23() *m2.m32();
        self.val[11]  = m1.m20() *m2.m03() + m1.m21() *m2.m13() + m1.m22() *m2.m23() + m1.m23() *m2.m33();

        self.val[12]  = m1.m30() *m2.m00() + m1.m31() *m2.m10() + m1.m32() *m2.m20() + m1.m33() *m2.m30();
        self.val[13]  = m1.m30() *m2.m01() + m1.m31() *m2.m11() + m1.m32() *m2.m21() + m1.m33() *m2.m31();
        self.val[14]  = m1.m30() *m2.m02() + m1.m31() *m2.m12() + m1.m32() *m2.m22() + m1.m33() *m2.m32();
        self.val[15]  = m1.m30() *m2.m03() + m1.m31() *m2.m13() + m1.m32() *m2.m23() + m1.m33() *m2.m33();
        self
    }
    
    
    ///
    /// Matrix Multiplication `this` = `other` * `this`
    ///
    /// # Arguments
    /// * `other_mat` Another Matrix4
    ///
    /// # Example
    ///
    pub fn multiply_post(&mut self, other_mat: Matrix4) -> &mut Self {
        let b = other_mat.val;
        let a00 = self.val[0];
        let a01 = self.val[1];
        let a02 = self.val[2];
        let a03 = self.val[3];
        let a10 = self.val[4];
        let a11 = self.val[5];
        let a12 = self.val[6];
        let a13 = self.val[7];
        let a20 = self.val[8];
        let a21 = self.val[9];
        let a22 = self.val[10];
        let a23 = self.val[11];
        let a30 = self.val[12];
        let a31 = self.val[13];
        let a32 = self.val[14];
        let a33 = self.val[15];

        // Cache only the current line of the second matrix
        let mut b0 = b[0];
        let mut b1 = b[1];
        let mut b2 = b[2];
        let mut b3 = b[3];
        self.val[0] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
        self.val[1] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
        self.val[2] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
        self.val[3] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

        b0 = b[4];
        b1 = b[5];
        b2 = b[6];
        b3 = b[7];
        self.val[4] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
        self.val[5] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
        self.val[6] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
        self.val[7] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

        b0 = b[8];
        b1 = b[9];
        b2 = b[10];
        b3 = b[11];
        self.val[8] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
        self.val[9] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
        self.val[10] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
        self.val[11] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

        b0 = b[12];
        b1 = b[13];
        b2 = b[14];
        b3 = b[15];
        self.val[12] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
        self.val[13] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
        self.val[14] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
        self.val[15] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;
        self
    }

    ///
    /// Matrix translation 
    ///
    /// ```math
    /// \begin{bmatrix}
    /// 1 & 0 & 0 & X \\
    /// 0 & 1 & 0 & Y \\
    /// 0 & 0 & 1 & Z \\
    /// 0 & 0 & 0 & 1 \\
    /// \end{bmatrix}
    /// ```
    /// 
    pub fn translate(&mut self, v: Vector3) -> &mut Self {
        let x = v.x;
        let y = v.y;
        let z = v.z;

        self.val[12] = self.val[0] * x + self.val[4] * y + self.val[8] * z + self.val[12];
        self.val[13] = self.val[1] * x + self.val[5] * y + self.val[9] * z + self.val[13];
        self.val[14] = self.val[2] * x + self.val[6] * y + self.val[10] * z + self.val[14];
        self.val[15] = self.val[3] * x + self.val[7] * y + self.val[11] * z + self.val[15];
        self
    }

    pub fn scale(&mut self, v: Vector3) -> &mut Self {
        let x = v.x;
        let y = v.y;
        let z = v.z;

        self.val[0] = self.val[0] * x;
        self.val[1] = self.val[1] * x;
        self.val[2] = self.val[2] * x;
        self.val[3] = self.val[3] * x;
        self.val[4] = self.val[4] * y;
        self.val[5] = self.val[5] * y;
        self.val[6] = self.val[6] * y;
        self.val[7] = self.val[7] * y;
        self.val[8] = self.val[8] * z;
        self.val[9] = self.val[9] * z;
        self.val[10] = self.val[10] * z;
        self.val[11] = self.val[11] * z;
        // self.val[12] = self.val[12];
        // self.val[13] = self.val[13];
        // self.val[14] = self.val[14];
        // self.val[15] = self.val[15];

        self
    }

    pub fn rotate(&mut self, rad: f64, axis: Vector3) -> &mut Self {
        let mut x = axis.x;
        let mut y = axis.y;
        let mut z = axis.z;
        let mut len = (x * x + y * y + z * z).sqrt();

        if len.abs() < EPSILON {
            panic!("Len too small");
        }

        len = 1.0 / len;
        x *= len;
        y *= len;
        z *= len;

        let s = rad.sin();
        let c = rad.cos();
        let t = 1.0 - c;

        let a00 = self.val[0];
        let a01 = self.val[1];
        let a02 = self.val[2];
        let a03 = self.val[3];
        let a10 = self.val[4];
        let a11 = self.val[5];
        let a12 = self.val[6];
        let a13 = self.val[7];
        let a20 = self.val[8];
        let a21 = self.val[9];
        let a22 = self.val[10];
        let a23 = self.val[11];

        // Construct the elements of the rotation matrix
        let b00 = x * x * t + c;
        let b01 = y * x * t + z * s;
        let b02 = z * x * t - y * s;
        let b10 = x * y * t - z * s;
        let b11 = y * y * t + c;
        let b12 = z * y * t + x * s;
        let b20 = x * z * t + y * s;
        let b21 = y * z * t - x * s;
        let b22 = z * z * t + c;

        // Perform rotation-specific matrix multiplication
        self.val[0] = a00 * b00 + a10 * b01 + a20 * b02;
        self.val[1] = a01 * b00 + a11 * b01 + a21 * b02;
        self.val[2] = a02 * b00 + a12 * b01 + a22 * b02;
        self.val[3] = a03 * b00 + a13 * b01 + a23 * b02;
        self.val[4] = a00 * b10 + a10 * b11 + a20 * b12;
        self.val[5] = a01 * b10 + a11 * b11 + a21 * b12;
        self.val[6] = a02 * b10 + a12 * b11 + a22 * b12;
        self.val[7] = a03 * b10 + a13 * b11 + a23 * b12;
        self.val[8] = a00 * b20 + a10 * b21 + a20 * b22;
        self.val[9] = a01 * b20 + a11 * b21 + a21 * b22;
        self.val[10] = a02 * b20 + a12 * b21 + a22 * b22;
        self.val[11] = a03 * b20 + a13 * b21 + a23 * b22;
        self
    }

    pub fn rotate_x(&mut self, rad: f64) -> &mut Self {
        let s = rad.sin();
        let c = rad.cos();
        let a10 = self.val[4];
        let a11 = self.val[5];
        let a12 = self.val[6];
        let a13 = self.val[7];
        let a20 = self.val[8];
        let a21 = self.val[9];
        let a22 = self.val[10];
        let a23 = self.val[11];

        // Perform axis-specific matrix multiplication
        self.val[4] = a10 * c + a20 * s;
        self.val[5] = a11 * c + a21 * s;
        self.val[6] = a12 * c + a22 * s;
        self.val[7] = a13 * c + a23 * s;
        self.val[8] = a20 * c - a10 * s;
        self.val[9] = a21 * c - a11 * s;
        self.val[10] = a22 * c - a12 * s;
        self.val[11] = a23 * c - a13 * s;
        self
    }

    pub fn rotate_y(&mut self, rad: f64) -> &mut Self {
        let s = rad.sin();
        let c = rad.cos();
        let a00 = self.val[0];
        let a01 = self.val[1];
        let a02 = self.val[2];
        let a03 = self.val[3];
        let a20 = self.val[8];
        let a21 = self.val[9];
        let a22 = self.val[10];
        let a23 = self.val[11];

        // Perform axis-specific matrix multiplication
        self.val[0] = a00 * c - a20 * s;
        self.val[1] = a01 * c - a21 * s;
        self.val[2] = a02 * c - a22 * s;
        self.val[3] = a03 * c - a23 * s;
        self.val[8] = a00 * s + a20 * c;
        self.val[9] = a01 * s + a21 * c;
        self.val[10] = a02 * s + a22 * c;
        self.val[11] = a03 * s + a23 * c;
        self
    }

    pub fn rotate_z(&mut self, rad: f64) -> &mut Self {
        let s = rad.sin();
        let c = rad.cos();
        let a00 = self.val[0];
        let a01 = self.val[1];
        let a02 = self.val[2];
        let a03 = self.val[3];
        let a10 = self.val[4];
        let a11 = self.val[5];
        let a12 = self.val[6];
        let a13 = self.val[7];

        // Perform axis-specific matrix multiplication
        self.val[0] = a00 * c + a10 * s;
        self.val[1] = a01 * c + a11 * s;
        self.val[2] = a02 * c + a12 * s;
        self.val[3] = a03 * c + a13 * s;
        self.val[4] = a10 * c - a00 * s;
        self.val[5] = a11 * c - a01 * s;
        self.val[6] = a12 * c - a02 * s;
        self.val[7] = a13 * c - a03 * s;
        self
    }
    
    ///
    /// Multiplies each element of this matrix by a scalar.
    /// 
    /// # Arguments
    /// * `scalar` The scalar multiplier.
    ///
    pub fn mul_by_scalar(&mut self, scalar: f64) -> &mut Self {
        self.val[0] *= scalar; self.val[1] *= scalar;  self.val[2]  *= scalar; self.val[3]  *= scalar;
        self.val[4] *= scalar; self.val[5] *= scalar;  self.val[6]  *= scalar; self.val[7]  *= scalar;
        self.val[8] *= scalar; self.val[9] *= scalar;  self.val[10]  *= scalar; self.val[11]  *= scalar;
        self.val[12] *= scalar; self.val[13] *= scalar;  self.val[14]  *= scalar; self.val[15]  *= scalar;
        self
     }
     
     
     
    //
    // Private function to set 16 values
    //
    pub fn set(&mut self, 
      m00: f64,  m01: f64,  m02: f64,  m03: f64, 
      m10: f64,  m11: f64,  m12: f64,  m13: f64,
      m20: f64,  m21: f64,  m22: f64,  m23: f64,
      m30: f64,  m31: f64,  m32: f64,  m33: f64) {
        self.val[0] = m00; self.val[1] = m01; self.val[2] = m02; self.val[3] = m03;
        self.val[4] = m10; self.val[5] = m11; self.val[6] = m12; self.val[7] = m13;
        self.val[8] = m20; self.val[9] = m21; self.val[10] = m22; self.val[11] = m23;
        self.val[12] = m30; self.val[13] = m31; self.val[14] = m32; self.val[15] = m33;
    }
    /*


        pub fn copy(&self,other_mat: Matrix4) -> Matrix4 {
            let mut out = [0.0f64;16];
            out.clone_from_slice(&self.val);
            let a = other_mat.val;
            out[0] = a[0];
            out[1] = a[1];
            out[2] = a[2];
            out[3] = a[3];
            out[4] = a[4];
            out[5] = a[5];
            out[6] = a[6];
            out[7] = a[7];
            out[8] = a[8];
            out[9] = a[9];
            out[10] = a[10];
            out[11] = a[11];
            out[12] = a[12];
            out[13] = a[13];
            out[14] = a[14];
            out[15] = a[15];
            Matrix4 {val:out}
        }



        pub fn adjoint(&mut self) -> &mut Self {
            let a = self.val;
            let a00 = a[0], a01 = a[1], a02 = a[2], a03 = a[3],
                a10 = a[4], a11 = a[5], a12 = a[6], a13 = a[7],
                a20 = a[8], a21 = a[9], a22 = a[10], a23 = a[11],
                a30 = a[12], a31 = a[13], a32 = a[14], a33 = a[15];

            a[0]  =  (a11 * (a22 * a33 - a23 * a32) - a21 * (a12 * a33 - a13 * a32) + a31 * (a12 * a23 - a13 * a22));
            a[1]  = -(a01 * (a22 * a33 - a23 * a32) - a21 * (a02 * a33 - a03 * a32) + a31 * (a02 * a23 - a03 * a22));
            a[2]  =  (a01 * (a12 * a33 - a13 * a32) - a11 * (a02 * a33 - a03 * a32) + a31 * (a02 * a13 - a03 * a12));
            a[3]  = -(a01 * (a12 * a23 - a13 * a22) - a11 * (a02 * a23 - a03 * a22) + a21 * (a02 * a13 - a03 * a12));
            a[4]  = -(a10 * (a22 * a33 - a23 * a32) - a20 * (a12 * a33 - a13 * a32) + a30 * (a12 * a23 - a13 * a22));
            a[5]  =  (a00 * (a22 * a33 - a23 * a32) - a20 * (a02 * a33 - a03 * a32) + a30 * (a02 * a23 - a03 * a22));
            a[6]  = -(a00 * (a12 * a33 - a13 * a32) - a10 * (a02 * a33 - a03 * a32) + a30 * (a02 * a13 - a03 * a12));
            a[7]  =  (a00 * (a12 * a23 - a13 * a22) - a10 * (a02 * a23 - a03 * a22) + a20 * (a02 * a13 - a03 * a12));
            a[8]  =  (a10 * (a21 * a33 - a23 * a31) - a20 * (a11 * a33 - a13 * a31) + a30 * (a11 * a23 - a13 * a21));
            a[9]  = -(a00 * (a21 * a33 - a23 * a31) - a20 * (a01 * a33 - a03 * a31) + a30 * (a01 * a23 - a03 * a21));
            a[10] =  (a00 * (a11 * a33 - a13 * a31) - a10 * (a01 * a33 - a03 * a31) + a30 * (a01 * a13 - a03 * a11));
            a[11] = -(a00 * (a11 * a23 - a13 * a21) - a10 * (a01 * a23 - a03 * a21) + a20 * (a01 * a13 - a03 * a11));
            a[12] = -(a10 * (a21 * a32 - a22 * a31) - a20 * (a11 * a32 - a12 * a31) + a30 * (a11 * a22 - a12 * a21));
            a[13] =  (a00 * (a21 * a32 - a22 * a31) - a20 * (a01 * a32 - a02 * a31) + a30 * (a01 * a22 - a02 * a21));
            a[14] = -(a00 * (a11 * a32 - a12 * a31) - a10 * (a01 * a32 - a02 * a31) + a30 * (a01 * a12 - a02 * a11));
            a[15] =  (a00 * (a11 * a22 - a12 * a21) - a10 * (a01 * a22 - a02 * a21) + a20 * (a01 * a12 - a02 * a11));
            self
        }




        pub fn fromRotationTranslation (q, v) {
            // Quaternion math
            var out = self.val,
                x = q.x, y = q.y, z = q.z, w = q.w,
                x2 = x + x,
                y2 = y + y,
                z2 = z + z,

                xx = x * x2,
                xy = x * y2,
                xz = x * z2,
                yy = y * y2,
                yz = y * z2,
                zz = z * z2,
                wx = w * x2,
                wy = w * y2,
                wz = w * z2;

            out[0] = 1 - (yy + zz);
            out[1] = xy + wz;
            out[2] = xz - wy;
            out[3] = 0;
            out[4] = xy - wz;
            out[5] = 1 - (xx + zz);
            out[6] = yz + wx;
            out[7] = 0;
            out[8] = xz + wy;
            out[9] = yz - wx;
            out[10] = 1 - (xx + yy);
            out[11] = 0;
            out[12] = v.x;
            out[13] = v.y;
            out[14] = v.z;
            out[15] = 1;
            self
        }

        pub fn fromQuat(q: Quaternion) {
            var out = self.val,
                x = q.x, y = q.y, z = q.z, w = q.w,
                x2 = x + x,
                y2 = y + y,
                z2 = z + z,

                xx = x * x2,
                xy = x * y2,
                xz = x * z2,
                yy = y * y2,
                yz = y * z2,
                zz = z * z2,
                wx = w * x2,
                wy = w * y2,
                wz = w * z2;

            out[0] = 1 - (yy + zz);
            out[1] = xy + wz;
            out[2] = xz - wy;
            out[3] = 0;

            out[4] = xy - wz;
            out[5] = 1 - (xx + zz);
            out[6] = yz + wx;
            out[7] = 0;

            out[8] = xz + wy;
            out[9] = yz - wx;
            out[10] = 1 - (xx + yy);
            out[11] = 0;

            out[12] = 0;
            out[13] = 0;
            out[14] = 0;
            out[15] = 1;

            self
        }


        ///
        /// Generates a frustum matrix with the given bounds
        ///
        /// @param {Number} left Left bound of the frustum
        /// @param {Number} right Right bound of the frustum
        /// @param {Number} bottom Bottom bound of the frustum
        /// @param {Number} top Top bound of the frustum
        /// @param {Number} near Near bound of the frustum
        /// @param {Number} far Far bound of the frustum
        /// @returns {Matrix4} self for chaining
        ////
        pub fn frustum = function (left, right, bottom, top, near, far) {
            var out = self.val,
                rl = 1 / (right - left),
                tb = 1 / (top - bottom),
                nf = 1 / (near - far);
            out[0] = (near * 2) * rl;
            out[1] = 0;
            out[2] = 0;
            out[3] = 0;
            out[4] = 0;
            out[5] = (near * 2) * tb;
            out[6] = 0;
            out[7] = 0;
            out[8] = (right + left) * rl;
            out[9] = (top + bottom) * tb;
            out[10] = (far + near) * nf;
            out[11] = -1;
            out[12] = 0;
            out[13] = 0;
            out[14] = (far * near * 2) * nf;
            out[15] = 0;
            self
        }


        ///
        /// Generates a perspective projection matrix with the given bounds
        ///
        /// @param {number} fovy Vertical field of view in radians
        /// @param {number} aspect Aspect ratio. typically viewport width/height
        /// @param {number} near Near bound of the frustum
        /// @param {number} far Far bound of the frustum
        /// @returns {Matrix4} self for chaining
        ////
        pub fn perspective = function (fovy, aspect, near, far) {
            var out = self.val,
                f = 1.0 / Math.tan(fovy / 2),
                nf = 1 / (near - far);
            out[0] = f / aspect;
            out[1] = 0;
            out[2] = 0;
            out[3] = 0;
            out[4] = 0;
            out[5] = f;
            out[6] = 0;
            out[7] = 0;
            out[8] = 0;
            out[9] = 0;
            out[10] = (far + near) * nf;
            out[11] = -1;
            out[12] = 0;
            out[13] = 0;
            out[14] = (2 * far * near) * nf;
            out[15] = 0;
            self
        }

        ///
        /// Generates a orthogonal projection matrix with the given bounds
        ///
        /// @param {number} left Left bound of the frustum
        /// @param {number} right Right bound of the frustum
        /// @param {number} bottom Bottom bound of the frustum
        /// @param {number} top Top bound of the frustum
        /// @param {number} near Near bound of the frustum
        /// @param {number} far Far bound of the frustum
        /// @returns {Matrix4} self for chaining
        ////
        pub fn ortho = function (left, right, bottom, top, near, far) {
            var out = self.val,
                lr = left-right,
                bt = bottom-top,
                nf = near-far;

            //avoid division by zero
            lr = lr===0 ? lr : 1/lr;
            bt = bt===0 ? bt : 1/bt;
            nf = nf===0 ? nf : 1/nf;

            out[0] = -2 * lr;
            out[1] = 0;
            out[2] = 0;
            out[3] = 0;
            out[4] = 0;
            out[5] = -2 * bt;
            out[6] = 0;
            out[7] = 0;
            out[8] = 0;
            out[9] = 0;
            out[10] = 2 * nf;
            out[11] = 0;
            out[12] = (left + right) * lr;
            out[13] = (top + bottom) * bt;
            out[14] = (far + near) * nf;
            out[15] = 1;
            self
        }

        ///
        /// Generates a look-at matrix with the given eye position, focal point, and up axis
        ///
        /// @param {Vector3} eye Position of the viewer
        /// @param {Vector3} center Point the viewer is looking at
        /// @param {Vector3} up vec3 pointing up
        /// @returns {Matrix4} self for chaining
        ////
        pub fn lookAt = function (eye, center, up) {
            var out = self.val,

                x0, x1, x2, y0, y1, y2, z0, z1, z2, len,
                eyex = eye.x,
                eyey = eye.y,
                eyez = eye.z,
                upx = up.x,
                upy = up.y,
                upz = up.z,
                centerx = center.x,
                centery = center.y,
                centerz = center.z;

            if (Math.abs(eyex - centerx) < EPSILON &&
                Math.abs(eyey - centery) < EPSILON &&
                Math.abs(eyez - centerz) < EPSILON) {
                return self.identity();
            }

            z0 = eyex - centerx;
            z1 = eyey - centery;
            z2 = eyez - centerz;

            len = 1 / Math.sqrt(z0 * z0 + z1 * z1 + z2 * z2);
            z0 *= len;
            z1 *= len;
            z2 *= len;

            x0 = upy * z2 - upz * z1;
            x1 = upz * z0 - upx * z2;
            x2 = upx * z1 - upy * z0;
            len = Math.sqrt(x0 * x0 + x1 * x1 + x2 * x2);
            if (!len) {
                x0 = 0;
                x1 = 0;
                x2 = 0;
            } else {
                len = 1 / len;
                x0 *= len;
                x1 *= len;
                x2 *= len;
            }

            y0 = z1 * x2 - z2 * x1;
            y1 = z2 * x0 - z0 * x2;
            y2 = z0 * x1 - z1 * x0;

            len = Math.sqrt(y0 * y0 + y1 * y1 + y2 * y2);
            if (!len) {
                y0 = 0;
                y1 = 0;
                y2 = 0;
            } else {
                len = 1 / len;
                y0 *= len;
                y1 *= len;
                y2 *= len;
            }

            out[0] = x0;
            out[1] = y0;
            out[2] = z0;
            out[3] = 0;
            out[4] = x1;
            out[5] = y1;
            out[6] = z1;
            out[7] = 0;
            out[8] = x2;
            out[9] = y2;
            out[10] = z2;
            out[11] = 0;
            out[12] = -(x0 * eyex + x1 * eyey + x2 * eyez);
            out[13] = -(y0 * eyex + y1 * eyey + y2 * eyez);
            out[14] = -(z0 * eyex + z1 * eyey + z2 * eyez);
            out[15] = 1;

            self
        }


        pub fn mul = pub fn multiply;

        pub fn idt = pub fn identity;

        //self is handy for Pool utilities, to "reset" a
        //shared object to its default state
        pub fn reset = pub fn idt;
    */
}

use std::ops::Mul;

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Matrix4::new();
        *result.multiply(self,rhs)
    }
}

impl Display for Matrix4 {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        let a = self.val;
        write!(
            f,
            "Matrix4(
    {}, {}, {}, {},
    {}, {}, {}, {},
    {}, {}, {}, {},
    {}, {}, {}, {}
)",
            a[0],
            a[1],
            a[2],
            a[3],
            a[4],
            a[5],
            a[6],
            a[7],
            a[8],
            a[9],
            a[10],
            a[11],
            a[12],
            a[13],
            a[14],
            a[15]
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::f64::consts::PI;
    
    #[test]
    fn identity_4x4() {
        let id = Matrix4::identity();
        assert_eq!(id.val[0], 1.0);
    }

    #[test]
    fn from_vec() {
        let arr: Vec<f64> = (1..=16).map(|x| x as f64).collect();
        let mat = Matrix4::from_vec(arr);
        // Assert
        let answer: [f64; 16] = [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ];
        assert!(mat.val.iter().zip(answer).all(|(a, b)| *a == b));
    }

    #[test]
    fn from_array() {
        let arr: [f64; 16] = [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ];
        let mat = Matrix4::from_array(arr);
        // Assert
        let answer: [f64; 16] = [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ];
        assert!(mat.val.iter().zip(answer).all(|(a, b)| *a == b));
    }

    #[test]
    fn chaining_rotate_methods() {
        let mut mat = Matrix4::identity();
        let mut result = mat.rotate_z(PI / 2.0).rotate_y(PI / 4.0).rotate_z(-PI / 2.0);
        // Assert
        let answer: [f64; 16] = [
            1.0, 0.0, 0.0, 0.0, 
            0.0, 0.707, -0.707, 0.0, 
            0.0, 0.707, 0.707, 0.0, 
            0.0, 0.0, 0.0, 1.0,
        ];

        assert!(result.val.iter().zip(answer).all(|(a, b)| (*a - b).abs() < 0.0005 ));
    }

    #[test]
    fn transpose() {
        let arr: Vec<f64> = (1..=16).map(|x| x as f64).collect();
        let mut mat = Matrix4::from_vec(arr);
        let tmat = mat.transpose();
        // Assert
        let answer: [f64; 16] = [
            1.0, 5.0, 9.0, 13.0, 
            2.0, 6.0, 10.0, 14.0, 
            3.0, 7.0, 11.0, 15.0, 
            4.0, 8.0, 12.0, 16.0,
        ];
        assert!(tmat.val.iter().zip(answer).all(|(a, b)| *a == b));
    }
    
    #[test]
    fn ops_trait_mul() {
        let vec1: Vec<f64> = vec![
          5.0,7.0,9.0,10.0,
          2.0,3.0,3.0,8.0,
          8.0,10.0,2.0,3.0,
          3.0,3.0,4.0,8.0
        ];
        let mut mat1 = Matrix4::from_vec(vec1);
        let vec2: Vec<f64> = vec![
          3.0,10.0,12.0,18.0,
          12.0,1.0,4.0,9.0,
          9.0,10.0,12.0,2.0,
          3.0,12.0,4.0,10.0
        ];
        let mut mat2 = Matrix4::from_vec(vec2);
        let mul_mat = mat1 * mat2;
        // Assert
        let answer: [f64; 16] = [
            210.0,267.0,236.0,271.0,
            93.0,149.0,104.0,149.0,
            171.0,146.0,172.0,268.0,
            105.0,169.0,128.0,169.0,
        ];
        let t_answer: [f64; 16] = [
            210.0,93.0,171.0,105.0,
            267.0,149.0,146.0,169.0,
            236.0,104.0,172.0,128.0,
            271.0,149.0,268.0,169.0,
        ];
        assert!(mul_mat.values().iter().zip(answer).all(|(a, b)| *a == b));
    }
    
      #[test]
    fn ops_multiply_a_la_opengl() {
        let vec1: Vec<f64> = vec![
          5.0,7.0,9.0,10.0,
          2.0,3.0,3.0,8.0,
          8.0,10.0,2.0,3.0,
          3.0,3.0,4.0,8.0
        ];
        let mut mat1 = Matrix4::from_vec(vec1);
        let vec2: Vec<f64> = vec![
          3.0,10.0,12.0,18.0,
          12.0,1.0,4.0,9.0,
          9.0,10.0,12.0,2.0,
          3.0,12.0,4.0,10.0
        ];
        let mut mat2 = Matrix4::from_vec(vec2);
        let result = mat2.multiply_post(mat1); // self = mat1 * self (post-multiplication)
        // Assert
        let answer: [f64; 16] = [
            210.0,267.0,236.0,271.0,
            93.0,149.0,104.0,149.0,
            171.0,146.0,172.0,268.0,
            105.0,169.0,128.0,169.0,
        ];
        let t_answer: [f64; 16] = [
            210.0,93.0,171.0,105.0,
            267.0,149.0,146.0,169.0,
            236.0,104.0,172.0,128.0,
            271.0,149.0,268.0,169.0,
        ];
        assert!(result.values().iter().zip(answer).all(|(a, b)| *a == b));
    }
    
    #[test]
    fn ops_mul() {
        let v1: Vec<f64> = vec![
          5.0,7.0,9.0,10.0,
          2.0,3.0,3.0,8.0,
          8.0,10.0,2.0,3.0,
          3.0,3.0,4.0,8.0
        ];
        let v2: Vec<f64> = vec![
          3.0,10.0,12.0,18.0,
          12.0,1.0,4.0,9.0,
          9.0,10.0,12.0,2.0,
          3.0,12.0,4.0,10.0
        ];
        let m1 = Matrix4::from_vec(v1);
        let m2 = Matrix4::from_vec(v2);
        let mut result = Matrix4::new();
        result = *result.multiply(m1,m2); // self = m1 * m2
        // Assert
        let answer: [f64; 16] = [
            210.0,267.0,236.0,271.0,
            93.0,149.0,104.0,149.0,
            171.0,146.0,172.0,268.0,
            105.0,169.0,128.0,169.0,
        ];
        let t_answer: [f64; 16] = [
            210.0,93.0,171.0,105.0,
            267.0,149.0,146.0,169.0,
            236.0,104.0,172.0,128.0,
            271.0,149.0,268.0,169.0,
        ];
        assert_eq!(result.val[0],210.0);
        assert!(result.values().iter().zip(answer).all(|(a, b)| *a == b));
    }
}
