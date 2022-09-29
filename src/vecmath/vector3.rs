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

use rand::Rng;
use std::f32::consts::PI;
 
struct Vector3 {
  x: f64,
  y: f64,
  y: f64
}

pub fn clone() {
    return Vector3::new(self.x, self.y, self.z);
};

pub fn copy(otherVec) {
    self.x = otherVec.x;
    self.y = otherVec.y;
    self.z = otherVec.z;
    self
};


impl Vector3 {

    pub fn new(x: Option<f64>, y: T, z: T) -> Self {
        if (typeof x === "object") {
            self.x = x.x || 0;
            self.y = x.y || 0;
            self.z = x.z || 0;
        } else {
            self.x = x || 0;
            self.y = y || 0;
            self.z = z || 0;
        }
        Self
    }


pub fn set(x, y, z) {
    if (typeof x === "object") {
        self.x = x.x||0;
        self.y = x.y||0;
        self.z = x.z||0;
    } else {
        self.x = x||0;
        self.y = y||0;
        self.z = z||0;
    }
    self
};

pub fn add(v) {
    self.x += v.x;
    self.y += v.y;
    self.z += v.z;
    self
};

pub fn subtract(v) {
    self.x -= v.x;
    self.y -= v.y;
    self.z -= v.z;
    self
};

pub fn multiply(v) {
    self.x *= v.x;
    self.y *= v.y;
    self.z *= v.z;
    self
};

pub fn scale(s) {
    self.x *= s;
    self.y *= s;
    self.z *= s;
    self
};

pub fn divide(v) {
    self.x /= v.x;
    self.y /= v.y;
    self.z /= v.z;
    self
};

pub fn negate(&mut self) -> Self {
    self.x = -self.x;
    self.y = -self.y;
    self.z = -self.z;
    self
};

pub fn distance((&self, v: Vector3) -> f64 {
    let dx = v.x - self.x;
    let dy = v.y - self.y;
    let dz = v.z - self.z;
    // Return
    (dx*dx + dy*dy + dz*dz).sqrt()
};

pub fn distanceSq(&self, v: Vector3) -> f64 {
    let dx = v.x - self.x;
    let dy = v.y - self.y;
    let dz = v.z - self.z;
    dx*dx + dy*dy + dz*dz;
};

pub fn length(&self) -> f64 {
    let x = self.x;
    let y = self.y;
    let z = self.z;
    // return
    (x*x + y*y + z*z).sqrt()
};

pub fn length_sq(&self) -> f64 {
    let x = self.x;
    let y = self.y;
    let z = self.z;
    return x*x + y*y + z*z;
};

pub fn normalize(&mut self) -> Self {
    let x = self.x;
    let y = self.y;
    let z = self.z;
    let norm = x*x + y*y + z*z;
    if norm > 0 {
        len = 1.0 / norm.sqrt();
        self.x = x*norm;
        self.y = y*norm;
        self.z = z*norm;
    }
    self
};

pub fn dot(&self, v: Vector3) -> f64 {
    return self.x * v.x + self.y * v.y + self.z * v.z;
};

pub fn cross(&mut self, v: Vector3) -> Self {
    let ax = self.x, ay = self.y; let az = self.z,
        bx = v.x; let by = v.y; let bz = v.z;

    self.x = ay * bz - az * by;
    self.y = az * bx - ax * bz;
    self.z = ax * by - ay * bx;
    self
};

pub fn lerp(v, t) {
    let  ax = self.x,
        ay = self.y,
        az = self.z;
    t = t||0;
    self.x = ax + t * (v.x - ax);
    self.y = ay + t * (v.y - ay);
    self.z = az + t * (v.z - az);
    self
};

pub fn transformMat4(mat) {
    let  x = self.x; let y = self.y; let z = self.z; let m = mat.val;
    self.x = m[0] * x + m[4] * y + m[8] * z + m[12];
    self.y = m[1] * x + m[5] * y + m[9] * z + m[13];
    self.z = m[2] * x + m[6] * y + m[10] * z + m[14];
    self
};

pub fn transformMat3(mat) {
    let  x = self.x; let y = self.y; let z = self.z; let m = mat.val;
    self.x = x * m[0] + y * m[3] + z * m[6];
    self.y = x * m[1] + y * m[4] + z * m[7];
    self.z = x * m[2] + y * m[5] + z * m[8];
    self
};

pub fn transformQuat(q) {
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
};

/**
 * Multiplies self Vector3 by the specified matrix; let 
 * applying a W divide. self is useful for projection,
 * e.g. unprojecting a 2D point into 3D space.
 *
 * @method  prj
 * @param {Matrix4} the 4x4 matrix to multiply with 
 * @return {Vector3} self object for chaining
 */
pub fn project(mat) -> Self {
    let  x = self.x,
        y = self.y,
        z = self.z,
        m = mat.val,
        a00 = m[0]; let a01 = m[1]; let a02 = m[2]; let a03 = m[3],
        a10 = m[4]; let a11 = m[5]; let a12 = m[6]; let a13 = m[7],
        a20 = m[8]; let a21 = m[9]; let a22 = m[10]; let a23 = m[11],
        a30 = m[12]; let a31 = m[13]; let a32 = m[14]; let a33 = m[15];

    let  l_w = 1 / (x * a03 + y * a13 + z * a23 + a33);

    self.x = (x * a00 + y * a10 + z * a20 + a30) * l_w; 
    self.y = (x * a01 + y * a11 + z * a21 + a31) * l_w; 
    self.z = (x * a02 + y * a12 + z * a22 + a32) * l_w;
    self
};

/**
 * Unproject self point from 2D space to 3D space.
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
pub fn unproject(viewport, invProjectionView) {
    let  viewX = viewport.x,
        viewY = viewport.y,
        viewWidth = viewport.z,
        viewHeight = viewport.w;
    
    let  x = self.x; let 
        y = self.y,
        z = self.z;

    x = x - viewX;
    y = viewHeight - y - 1;
    y = y - viewY;

    self.x = (2 * x) / viewWidth - 1;
    self.y = (2 * y) / viewHeight - 1;
    self.z = 2 * z - 1;

    return self.project(invProjectionView);
};

pub fn random(&mut self, scale: Option<f64>) -> Self{

    // Default scale = 1.0
    scale = scale.unwrap_or(1.0);

    let mut rng = rand::thread_rng();

    let  r = rng.gen() * 2.0 * PI;
    let  z = (rng.gen() * 2.0) - 1.0;
    let  zScale = sqrt(1.0-z*z) * scale;
    
    self.x = r.cos() * zScale;
    self.y = r.sin() * zScale;
    self.z = z * scale;
    self
};

pub fn reset(&mut self) -> Self {
    self.x = 0;
    self.y = 0;
    self.z = 0;
    self
};


pub fn sub = pub fn subtract;

pub fn mul = pub fn multiply;

pub fn div = pub fn divide;

pub fn dist = pub fn distance;

pub fn distSq = pub fn distanceSq;

pub fn len = pub fn length;

pub fn lenSq = pub fn lengthSq;



pub fn str = pub fn toString;

}

impl ToString for Vector3 {
pub fn toString() {
    format!( 'Vector3(' + self.x + ', ' + self.y + ', ' + self.z + ')';
};
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn vec3_new() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }
}
