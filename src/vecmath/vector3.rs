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

use rand::Rng;
use std::f64::consts::PI;
use std::fmt::{self, Display, Formatter};

use crate::vecmath::matrix3::Matrix3;
use crate::vecmath::matrix4::Matrix4;
use crate::vecmath::vector4::Vector4;

/// Adapted from JS package
/// https://github.com/mattdesl/vecmath

#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn from(other: &Vector3) -> Self {
        Vector3 {
            x: other.x,
            y: other.y,
            z: other.z,
        }
    }

    ///
    /// Builder pattern
    ///
    pub fn build(&self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn set(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }

    pub fn add(&mut self, v: &Vector3) -> &mut Self {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
        self
    }

    pub fn subtract(&mut self, v: &Vector3) -> &mut Self {
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
        self
    }

    pub fn multiply(&mut self, v: &Vector3) -> &mut Self {
        self.x *= v.x;
        self.y *= v.y;
        self.z *= v.z;
        self
    }

    pub fn scale(&mut self, s: f64) -> &mut Self {
        self.x *= s;
        self.y *= s;
        self.z *= s;
        self
    }

    pub fn divide(&mut self, v: &Vector3) -> &mut Self {
        self.x /= v.x;
        self.y /= v.y;
        self.z /= v.z;
        self
    }

    pub fn negate(&mut self) -> &mut Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }

    pub fn distance(&self, v: Vector3) -> f64 {
        let dx = v.x - self.x;
        let dy = v.y - self.y;
        let dz = v.z - self.z;
        // Return
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn distance_sq(&self, v: Vector3) -> f64 {
        let dx = v.x - self.x;
        let dy = v.y - self.y;
        let dz = v.z - self.z;
        dx * dx + dy * dy + dz * dz
    }

    pub fn length(&self) -> f64 {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        // return
        (x * x + y * y + z * z).sqrt()
    }

    pub fn length_sq(&self) -> f64 {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        return x * x + y * y + z * z;
    }

    pub fn normalize(&mut self) -> &mut Self {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let norm = x * x + y * y + z * z;
        if norm > 0.0 {
            let len = 1.0 / norm.sqrt();
            self.x = x * norm;
            self.y = y * norm;
            self.z = z * norm;
        }
        self
    }

    pub fn dot(&self, v: Vector3) -> f64 {
        return self.x * v.x + self.y * v.y + self.z * v.z;
    }

    pub fn cross(&mut self, v: Vector3) -> &mut Self {
        let ax = self.x;
        let ay = self.y;
        let az = self.z;
        let bx = v.x;
        let by = v.y;
        let bz = v.z;

        self.x = ay * bz - az * by;
        self.y = az * bx - ax * bz;
        self.z = ax * by - ay * bx;
        self
    }

    pub fn lerp(&mut self, v: &Vector3, t: f64) -> &mut Self {
        let ax = self.x;
        let ay = self.y;
        let az = self.z;
        self.x = ax + t * (v.x - ax);
        self.y = ay + t * (v.y - ay);
        self.z = az + t * (v.z - az);
        self
    }

    pub fn transform_mat4(&mut self, mat: Matrix4) -> &mut Self {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let m = mat.values();
        self.x = m[0] * x + m[4] * y + m[8] * z + m[12];
        self.y = m[1] * x + m[5] * y + m[9] * z + m[13];
        self.z = m[2] * x + m[6] * y + m[10] * z + m[14];
        self
    }

    pub fn transform_mat3(&mut self, mat: Matrix3) -> &mut Self {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let m = mat.values();
        self.x = x * m[0] + y * m[3] + z * m[6];
        self.y = x * m[1] + y * m[4] + z * m[7];
        self.z = x * m[2] + y * m[5] + z * m[8];
        self
    }

    /*
        pub fn transform_quat(&mut self, mat: Quaternion) -> &mut Self {
            // benchmarks: http://jsperf.com/quaternion-transform-vec3-implementations
            let  x = self.x; let y = self.y; let z = self.z,
                qx = q.x; let qy = q.y; let qz = q.z; let qw = q.w,

                // calculate quat * vec
                ix = qw * x + qy * z - qz * y,
                iy = qw * y + qz * x - qx * z,
                iz = qw * z + qx * y - qy * x,
                iw = -qx * x - qy * y - qz * z;

            // calculate result * inverse quat
            self.x = ix * qw + iw * -qx + iy * -qz - iz * -qy;
            self.y = iy * qw + iw * -qy + iz * -qx - ix * -qz;
            self.z = iz * qw + iw * -qz + ix * -qy - iy * -qx;
            self
        }
    */

    /**
     * Multiplies this Vector3 by the specified matrix; let
     * applying a W divide. self is useful for projection,
     * e.g. unprojecting a 2D point into 3D space.
     *
     * @method  prj
     * @param {Matrix4} the 4x4 matrix to multiply with
     * @return {Vector3} self object for chaining
     */
    pub fn project(&mut self, mat: Matrix4) -> &mut Self {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let m = mat.values();
        let a00 = m[0];
        let a01 = m[1];
        let a02 = m[2];
        let a03 = m[3];
        let a10 = m[4];
        let a11 = m[5];
        let a12 = m[6];
        let a13 = m[7];
        let a20 = m[8];
        let a21 = m[9];
        let a22 = m[10];
        let a23 = m[11];
        let a30 = m[12];
        let a31 = m[13];
        let a32 = m[14];
        let a33 = m[15];

        let l_w = 1.0 / (x * a03 + y * a13 + z * a23 + a33);

        self.x = (x * a00 + y * a10 + z * a20 + a30) * l_w;
        self.y = (x * a01 + y * a11 + z * a21 + a31) * l_w;
        self.z = (x * a02 + y * a12 + z * a22 + a32) * l_w;
        self
    }

    /**
     * Unproject this point from 2D space to 3D space.
     * The point should have its x and y properties set to
     * 2D screen space, and the z either at 0 (near plane)
     * or 1 (far plane). The provided matrix is assumed to already
     * be combined, i.e. projection * view * model.
     *
     * After self operation, self vector's (x, y, z) components will
     * represent the unprojected 3D coordinate.
     *
     * @param  {Vector4} viewport          screen x, y, width and height in pixels
     * @param  {Matrix4} invProjectionView combined projection and view matrix
     * @return {Vector3}                   self object, for chaining
     */
    pub fn unproject(&mut self, viewport: Vector4, inv_projection_view: Matrix4) -> &mut Self {
        let view_x = viewport.x;
        let view_y = viewport.y;
        let view_width = viewport.z;
        let view_height = viewport.w;

        let mut x = self.x;
        let mut y = self.y;
        let z = self.z;

        x = x - view_x;
        y = view_height - y - 1.0;
        y = y - view_y;

        self.x = (2.0 * x) / view_width - 1.0;
        self.y = (2.0 * y) / view_height - 1.0;
        self.z = 2.0 * z - 1.0;

        return self.project(inv_projection_view);
    }

    pub fn random(&mut self, scale: Option<f64>) -> &mut Self {
        // Default scale = 1.0
        let scale = scale.unwrap_or(1.0);

        let mut rng = rand::thread_rng();

        let r: f64 = rng.gen::<f64>() * 2.0 * PI;
        let z: f64 = (rng.gen::<f64>() * 2.0) - 1.0;
        let z_scale: f64 = (1.0_f64 - z * z).sqrt() * scale;

        self.x = r.cos() * z_scale;
        self.y = r.sin() * z_scale;
        self.z = z * scale;
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
        self
    }

    /*
        pub fn sub = pub fn subtract;

        pub fn mul = pub fn multiply;

        pub fn div = pub fn divide;

        pub fn dist = pub fn distance;

        pub fn distSq = pub fn distanceSq;

        pub fn len = pub fn length;

        pub fn lenSq = pub fn lengthSq;



        pub fn str = pub fn toString;
    */
}

impl Display for Vector3 {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "vec3({:.3},{:.3},{:.3})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn vec3_new() {}

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
    }
}
