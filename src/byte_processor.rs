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
 
 
impl Access<u8> for ImageProcessor<u8,Gray8> {
    type Output = u8;
    /// Check index bounds
    fn get_pixel(&self,index: usize) -> Self::Output {
        let size = (self.get_width() * self.get_height()) as usize;
        match index {
            x if x < size => self.data()[index],
            _ => panic!("Out of bounds")
        }
    }
   fn get_pixel_at(&self,x: u32, y: u32) -> Self::Output {
        Self::get_pixel(self,(x + self.get_width() * y) as usize)
    }

    fn getf(&self,index: usize) -> f32 {
        self.data()[index] as f32
    }

}

// Alias compatible with ImageJ

type ByteProcessor = ImageProcessor<u8,Gray8>;


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_pixel_at_xy() {
        let ip = ByteProcessor {width: 2, height: 2, data: vec![1,2,3,4], cs: Gray8};
        let px = ip.get_pixel_at(1,1);
        assert_eq!(px, 4);
    }
    
    #[test]
    fn get_pixel_rgb_from_index() {
        let ip = ByteProcessor {width: 4, height: 3, data: vec![1,2,3,4,5,6,7,8,9,10,11,12], cs: Gray8};
        let px = ip.get_pixel(7);
        assert_eq!(px, 8);
    }
}

