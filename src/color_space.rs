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

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
enum Space {
    Gray,
    Rgb
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct ColorSpace<T>  {
    nb_channels : u8,
    bits_per_color : u8,
    space : Space,
    min : T,
    max : T
}

impl<T> ColorSpace<T> where T: Copy {
    //// Constructeurs ////
    pub fn Gray8() -> ColorSpace<u8> {
        return ColorSpace{
            nb_channels : 1,
            bits_per_color : 8,
            space : Space::Gray,
            min : u8::MIN,
            max : u8::MAX
        }
    }
    pub fn Grayf32() -> ColorSpace<f32> {
        return ColorSpace{
            nb_channels : 1,
            bits_per_color : 32,
            space : Space::Gray,
            min : f32::MIN,
            max : f32::MAX
        }
    }
    pub fn Rgb24() -> ColorSpace<(u8,u8,u8)> {
        return ColorSpace{
            nb_channels : 3,
            bits_per_color : 8,
            space : Space::Rgb,
            min : (u8::MIN,u8::MIN,u8::MIN),
            max : (u8::MAX,u8::MAX,u8::MAX)
        }
    }

    //// Getters ////
    pub fn get_nb_channels(&self) -> u8 {
        return self.nb_channels
    }
    pub fn get_bit_depth(&self) -> u8 {
        return self.bits_per_color * self.get_nb_channels()
    }

    pub fn get_min(&self) -> T {
        return self.min
    }
    pub fn get_max(&self) -> T {
        return self.max
    }
}

#[cfg(test)]
mod test{
    use crate::color_space::ColorSpace;
    use crate::color_space::Space;
    use core::cell::RefCell;
    use core::cell::Cell;

    #[test]
    fn test_Gray8(){
        let color = ColorSpace{
            nb_channels : 1,
            bits_per_color : 8,
            space : Space::Gray,
            min : u8::MIN,
            max : u8::MAX
        };
        assert_eq!(ColorSpace::<u8>::Gray8(),color);
    }

    #[test]
    fn test_Grayf32(){
        let color = ColorSpace{
            nb_channels : 1,
            bits_per_color : 32,
            space : Space::Gray,
            min : f32::MIN,
            max : f32::MAX
        };
        assert_eq!(ColorSpace::<f32>::Grayf32(),color);
    }

    #[test]
    fn test_Rgb24(){
        let color = ColorSpace{
            nb_channels : 3,
            bits_per_color : 8,
            space : Space::Rgb,
            min : (u8::MIN,u8::MIN,u8::MIN),
            max : (u8::MAX,u8::MAX,u8::MAX)
        };
        assert_eq!(ColorSpace::<(u8,u8,u8)>::Rgb24(),color);
    }

    #[test]
    fn test_get_nb_channels(){
        let color = ColorSpace{
            nb_channels : 3,
            bits_per_color : 8,
            space : Space::Rgb,
            min : (u8::MIN,u8::MIN,u8::MIN),
            max : (u8::MAX,u8::MAX,u8::MAX)
        };
        assert_eq!(color.get_nb_channels(),3);
    }

    #[test]
    fn test_get_bit_depth(){
        let color = ColorSpace{
            nb_channels : 1,
            bits_per_color : 32,
            space : Space::Gray,
            min : f32::MIN,
            max : f32::MAX
        };
        assert_eq!(color.get_bit_depth(),32);
    }

    #[test]
    fn test_get_min_Gray8(){
        let color = ColorSpace{
            nb_channels : 1,
            bits_per_color : 8,
            space : Space::Gray,
            min : u8::MIN,
            max : u8::MAX
        };
        assert_eq!(color.get_min(),0);
    }

    #[test]
    fn test_get_min_Grayf32(){
        let color = ColorSpace{
            nb_channels : 1,
            bits_per_color : 32,
            space : Space::Gray,
            min : f32::MIN,
            max : f32::MAX
        };
        assert_eq!(color.get_min(),-3.4028235e38);
    }

    #[test]
    fn test_get_min_RGB(){
        let color = ColorSpace{
            nb_channels : 3,
            bits_per_color : 8,
            space : Space::Rgb,
            min : (u8::MIN,u8::MIN,u8::MIN),
            max : (u8::MAX,u8::MAX,u8::MAX)
        };
        assert_eq!(color.get_min(),(0,0,0));
    }

    #[test]
    fn test_get_max_Gray8(){
        let color = ColorSpace{
            nb_channels : 1,
            bits_per_color : 8,
            space : Space::Gray,
            min : u8::MIN,
            max : u8::MAX
        };
        assert_eq!(color.get_max(),255);
    }

    #[test]
    fn test_get_max_Grayf32(){
        let color = ColorSpace{
            nb_channels : 1,
            bits_per_color : 32,
            space : Space::Gray,
            min : f32::MIN,
            max : f32::MAX
        };
        assert_eq!(color.get_max(),3.4028235e38);
    }

    #[test]
    fn test_get_max_RGB(){
        let color = ColorSpace{
            nb_channels : 3,
            bits_per_color : 8,
            space : Space::Rgb,
            min : (u8::MIN,u8::MIN,u8::MIN),
            max : (u8::MAX,u8::MAX,u8::MAX)
        };
        assert_eq!(color.get_max(),(255,255,255));
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
