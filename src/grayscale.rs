//
//  RIM - Rust Image
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


/// Authors:  Nicolas Maurice, Bluwen Guidoux

use crate::pixel::PixelType;
use crate::color_space::*;

pub type Gray<T> = Space<T>;

impl<T: PixelType> ColorSpace for Gray<T> {
    fn new() -> Self {
        Gray::<T> {
            component: T::zero(),
            channels: 1,
            stride: 1,
            pack: true,
        }
    }
    // Accessors
    fn channels(&self) -> u8 {
        1
    }
    fn stride(&self) -> u32 {
        1
    }
    fn pack(&self) -> bool {
        true
    }
    fn channel_names(&self) -> Vec<&str> {
        vec!["gray"]
    }
}
