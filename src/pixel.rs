//
//  RIM - Rust IMage
//  Copyright (&self,C) 2022  Jean-Christophe Taveau.
//
//  This file is part of RIM
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (&self,at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with RIM.  If not, see <http://www.gnu.org/licenses/>.
 
 
///
/// Pixel
///


pub trait PixelType {
    type COMPONENT;
    fn zero() -> Self;
    fn to_f32(&self) -> f32;
    fn to_value(&self) -> Self;
    fn clamp_pixel(v: f32) -> Self;
}

// u8
impl PixelType for u8 {
    type COMPONENT = u8;
    fn zero() -> u8 {
        0
    }
    fn to_f32(&self) -> f32 {
        *self as f32
    }
    fn to_value(&self) -> u8 {
        *self
    }
    fn clamp_pixel(v: f32) -> u8 {
        if v < u8::min_value() as f32 {
            u8::min_value()
        } else if v > u8::max_value() as f32 {
            u8::max_value()
        } else {
            v as u8
        }
    }
}

// TODO u16,u32


// f32
impl PixelType for f32 {
    type COMPONENT = f32;
    fn zero() -> f32 {
        0.0
    }
    fn to_f32(&self) -> f32 {
        *self
    }
    fn to_value(&self) -> f32 {
        *self
    }
    fn clamp_pixel(v: f32) -> f32 {
        // No clamping is done, TODO?
        v
    }
}


// TODO f64

