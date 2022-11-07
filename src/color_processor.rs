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
 
#![allow(non_camel_case_types)]
#![allow(unused)]

// Aliases compatible with ImageJ
type ColorProcessor = IP<u8,Rgb24>;

impl Access<u8> for  IP<u8,Rgb24> {
    type Output = (u8,u8,u8);
    
    fn get_pixel(&self,index: usize) -> Self::Output {
        let i = index*self.cs.channels() as usize;
        match self.cs.space() {
            Space::Rgb => (self.data[i],self.data[i+1],self.data[i+2]),
            Space::Prgb => (
                    self.data[index],
                    self.data[index + (self.width * self.height) as usize],
                    self.data[index + (self.width * self.height * 2) as usize]
                ), 
            _ => panic!("not implemented")
        }
    }
   fn get_pixel_at(&self,x: u32, y: u32) -> Self::Output {
        Self::get_pixel(self,(x + self.get_width() * y) as usize)
    }
    
    fn getf(&self,index: usize) -> f32 {
        let tup = Self::get_pixel(self,index);
        (tup.0 + tup.1 + tup.2) as f32 / 3.0
    }

}


