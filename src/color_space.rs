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
 

#![allow(unused)]

enum Space {
    Gray,
    Rgb
}

pub struct ColorSpace {
    nb_channels : u8,
    space : Space
}

impl ColorSpace {
    //// Constructeurs ////
    pub fn Gray8() -> Self {
        return ColorSpace{
            nb_channels : 1,
            space : Space::Gray
        }
    }
    pub fn Grayf32() -> Self {
        return ColorSpace{
            nb_channels : 1,
            space : Space::Gray
        }
    }
    pub fn Rgb24() -> Self {
        return ColorSpace{
            nb_channels : 3,
            space : Space::Rgb
        }
    }

    //// Getter ////
    pub fn get_nb_channels(&self) -> u8 {
        return self.nb_channels
    }
}

/*
Old Code
enum Space {
    Gray, // Grayscale
    Prgb, // Planar r,r,r,g,g,g,b,b,b
    Prgba, // Planar
    Rgb, // Packed r,g,b,r,g,b,r,g,b
    Rgba
}

trait PixelType {
   type COMPONENT;
}

trait GraySpace {
    const SPACE: Space;
    // Accessors
    fn space(&self) -> Space {
        Self::SPACE
    }
    fn get_bit_depth(&self) -> u8;
}


trait ColorSpace {
    const SPACE: Space;
    const CHANNELS: u8;
    const STRIDE: u32;
    const PACK: bool;
    // Accessors
    fn space(&self) -> Space {
        Self::SPACE
    }
    fn channels(&self) -> u8 {
        Self::CHANNELS
    }
    fn stride(&self) -> u32 {
        Self::STRIDE
    }
    fn pack(&self) -> bool {
        Self::PACK
    }
    fn channel_names(&self) -> Vec<&str>;
    fn get_bit_depth(&self) -> u8;
}

// Grayscale Processors
impl PixelType for u8 {
    type COMPONENT = u8;
}

struct Gray8;
impl GraySpace for Gray8 {
    const SPACE: Space = Space::Gray;
    fn get_bit_depth(&self) -> u8 {
      8
    }
}

struct Gray16;
impl PixelType for u16 {
    type COMPONENT = u16;
}
impl GraySpace for Gray16 {
    const SPACE: Space = Space::Gray;
    fn get_bit_depth(&self) -> u8 {
      16
    }
}

struct Gray32;
impl PixelType for u32 {
    type COMPONENT = u32;
}
impl GraySpace for Gray32 {
    const SPACE: Space = Space::Gray;
    fn get_bit_depth(&self) -> u8 {
      32
    }
}

struct Grayf32;
impl PixelType for f32 {
    type COMPONENT = f32;
}
impl GraySpace for Grayf32 {
    const SPACE: Space = Space::Gray;
    fn get_bit_depth(&self) -> u8 {
      32
    }
}

// Color
struct Rgb24;
impl PixelType for Rgb24 {
    type COMPONENT = u8;
}

impl ColorSpace for Rgb24 {
    const SPACE: Space = Space::Rgb;
    const CHANNELS : u8 = 3;
    const STRIDE : u32 = 1;
    const PACK: bool = true;
    fn channel_names(&self) -> Vec<&str> {
        vec!["red","green","blue"]
    }
    fn get_bit_depth(&self) -> u8 {
      24
    }
}

*/
