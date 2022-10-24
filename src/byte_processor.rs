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
 
use crate::color_space::ColorSpace;
use crate::grayscale::Gray;
use crate::image_processor::ImageProcessor;
use crate::gray_processor::*;

// Alias compatible with ImageJ
type Gray8 = Gray<u8>;
type ByteProcessor = ImageProcessor<u8,Gray8>;


#[cfg(test)]
mod tests {

    use super::*;


    use crate::image_processor::*;
    use crate::image_traits::Access;
    use crate::operator::Operator;


    
    #[test]
    fn get_pixel_at_xy() {
        let ip = ByteProcessor::new(2, 2, vec![1,2,3,4], Gray8::new() );
        let px = ip.get_pixel_at(1,1);
        assert_eq!(px.unwrap(), 4);
    }
    
    #[test]
    fn get_pixel_rgb_from_index() {
        let ip = ByteProcessor::new(4, 3, vec![1,2,3,4,5,6,7,8,9,10,11,12], Gray8::new() );
        let px = ip.get_pixel(7);
        assert_eq!(px.unwrap(), 8);
    }

    #[test]
    fn add() {
        let mut ip = ByteProcessor::new(4, 3, vec![1,2,3,4,5,6,7,8,9,10,11,12], Gray8::new() );
        ip.add(10);
        assert_eq!(ip.get(3),14);
    }

    #[test]
    fn subtract() {
        let mut ip = ByteProcessor::new(4, 3, vec![20u8; 12], Gray8::new() );
        ip.subtract(12);
        assert_eq!(ip.get(3),8);
    }

    #[test]
    fn substract_underflow() {
        let mut ip = ByteProcessor::new(4, 3, vec![1,2,3,4,5,6,7,8,9,10,11,12], Gray8::new() );
        ip.subtract(10);
        assert_eq!(ip.get(3),0);
    }

    #[test]
    fn multiply() {
        let mut ip = ByteProcessor::new(4, 3, vec![10u8;12], Gray8::new() );
        ip.multiply(2);
        assert_eq!(ip.get(3),20);
    }

    #[test]
    fn multiply_overflow()  {
        let mut ip = ByteProcessor::new(4, 3, vec![150u8;12], Gray8::new() );
        ip.multiply(2);
        assert_eq!(ip.get(3),255);
    }

    #[test]
    fn divide() {
        let mut ip = ByteProcessor::new(4, 3, vec![30u8;12], Gray8::new() );
        ip.divide(10);
        assert_eq!(ip.get(3),3);
    }
    
    #[test]
    fn floor() {
        let mut ip = ByteProcessor::new(4, 3, vec![1,2,3,4,5,6,7,8,9,10,11,12], Gray8::new() );
        ip.floor(6);
        let answer = vec![6u8,6u8,6u8,6u8,6u8,6u8,7u8,8u8,9u8,10u8,11u8,12u8];
        assert!(ip.data().iter().all(|item| answer.contains(item)));
    }
    
    #[test]
    fn ceil() {
        let mut ip = ByteProcessor::new(4, 3, vec![1,2,3,4,5,6,7,8,9,10,11,12], Gray8::new() );
        ip.ceil(7);
        let answer = vec![1u8,2u8,3u8,4u8,5u8,6u8,7u8,7u8,7u8,7u8,7u8,7u8];
        assert!(ip.data().iter().all(|item| answer.contains(item)));

    }
    
    #[test]
    #[ignore] //Aléatoire, échoue parfois, réussi parfois
    fn noise() {
        let mut ip = ByteProcessor::new(4, 3, vec![100u8;12], Gray8::new() );
        ip.noise(10.0);
        assert_ne!(ip.get(3),100);
    }

    #[test]
    fn abs(){
        let mut ip = ByteProcessor::new(4, 3, vec![100u8;12], Gray8::new() );
        ip.abs();
        assert_eq!(ip.get(3),100);
    }

    #[test]
    fn exp(){
        let mut ip = ByteProcessor::new(4, 3, vec![2u8;12], Gray8::new() );
        ip.exp();
        assert_eq!(ip.get(3),7);
    }

    #[test]
    fn sqrt(){
        let mut ip = ByteProcessor::new(4, 3, vec![4u8;12], Gray8::new() );
        ip.sqrt();
        assert_eq!(ip.get(3),2);
    }

    #[test]
    fn ln(){
        let mut ip = ByteProcessor::new(4, 3, vec![35u8;12], Gray8::new() );
        ip.ln();
        assert_eq!(ip.get(3),3);
    }

    #[test]
    fn log(){
        let mut ip = ByteProcessor::new(4, 3, vec![1,2,3,4,5,6,7,8,9,10,11,12], Gray8::new() );
        ip.add(35);
        ip.log();
        assert_eq!(ip.get(3),1);
    }

    //gamma

    
}


