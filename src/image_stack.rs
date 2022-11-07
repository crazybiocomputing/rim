//
//  RIM - Rust Image
//  Copyright (C) 2022  Jean-Christophe Taveau, Nicolas Maurice, Bluwen Guidoux.
//
//  This file is part of RIM
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with RIM.  If not, see <http://www.gnu.org/licenses/>.

//size
//vecgteur d'image processor
//int
//w, h, cs, T avoir les infos sur le type d'image dans le stack

#![allow(non_camel_case_types)]
#![allow(unused)]#![allow(non_camel_case_types)]
#![allow(unused)]

use crate::color_space::ColorSpace;
use crate::grayscale::Gray;
use crate::image_processor::ImageProcessor;
use crate::pixel::PixelType;

///
/// ImageStack
///
/// This class represents an expandable array of images.
///
pub struct ImageStack<T: PixelType, C: ColorSpace> {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub data: Vec<Vec<T>>,
    pub labels: Vec<String>,
    pub cs: C, // metadata: MetaData
}

impl<T: PixelType + std::clone::Clone, C: ColorSpace> ImageStack<T, C> {
    // Constructors
    pub fn new(w: u32, h: u32, pixel_stack: Vec<Vec<T>>, cs: C) -> Self {
        let nslices: u32 = pixel_stack.len().clone() as u32;
        ImageStack {
            width: w,
            height: h,
            depth: nslices,
            data: pixel_stack,
            labels: vec![""; nslices as usize]
                .iter()
                .enumerate()
                .map(|(i, v)| (i + 1).to_string())
                .collect(),
            cs: cs,
        }
    }

    // Accessors
    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn get_height(&self) -> u32 {
        self.height
    }
    pub fn n_slices(&self) -> u32 {
        self.depth
    }
    pub fn labels(&self) -> &Vec<String> {
        &self.labels
    }
    pub fn data(&self) -> &Vec<Vec<T>> {
        &self.data
    }
    pub fn get_slice_size(&self) -> usize {
        (self.width * self.height) as usize
    }

    ///
    /// Returns the label of the specified slice, where 1<=n<=nslices.
    ///
    /// Returns an Error "Out of range" if the slice does not have a label or 'n'; is out of range.
    ///
    /// For DICOM and FITS stacks, labels may contain header information.
    ///
    pub fn get_slice_label(&self, n: usize) -> Result<String, &str> {
        match n {
            x if x > 0 && x <= self.n_slices() as usize => Ok(self.labels[n - 1].clone()),
            _ => Err("Out of range"),
        }
    }

    ///
    /// Sets the label of the specified slice, where 1<=n<=nslices.
    ///
    pub fn set_slice_label(&mut self, label: String, n: usize) {
        // TODO
        // Check that the slice exists in data and n is correct.
        self.labels[n - 1] = label;
    }

    // Manipulating slices

    ///
    /// Add a slice at the end of the ImageStack
    ///
    /// # Arguments
    ///
    /// * `slice_label` - A _String_ label describing the slice.
    /// * `ip` - An _ImageProcessor_.
    ///
    pub fn add_slice(&mut self, slice_label: String, ip: &ImageProcessor<T, C>) {
        self.data.push(ip.data.to_vec());
        self.labels.push(slice_label);
    }
}

///
/// Macro to create a stack
///
macro_rules! stack_fill_with {
    // This macro takes expressions of type `expr`
    ($v:expr, $pixel:ty, $w:expr, $h:expr, $d:expr) => {
        // Call `find_min!` on the tail `$y`
        ImageStack {
            width: $w,
            height: $h,
            depth: $d,
            data: vec![vec![$v as $pixel; ($w * $h) as usize]; $d],
            labels: vec![""; $d]
                .iter()
                .enumerate()
                .map(|(i, _)| (i + 1).to_string())
                .collect(),
            cs: Gray::<$pixel>::new(),
        }
    };
}

/*
impl<T> ImageStack<T> where T: Copy {

    pub fn create_stack(width: u32, height: u32, size: Cell<u32>, data: RefCell<Vec<RefCell<ImageProcessor<T>>>>,cs : ColorSpace<T>,focus_slice: Cell<u32>) -> ImageStack<T>{
        return ImageStack{
            width : width,
            height : height,
            size : size,
            data: data,
            cs : cs,
            focus_slice: focus_slice
        }
    }

    pub fn create_byte_stack(width: u32, height: u32, size: u32) -> ByteStack {
        let cs : ColorSpace<u8> = ColorSpace::<u8>::Gray8();
        let data = RefCell::new(vec![RefCell::new(ByteProcessor::create_byte_processor(width,height));size.try_into().unwrap()]);
        let focus_slice = Cell::new(0);
        let size = Cell:: new(size);
        return ImageStack::<u8>::create_stack(width,height,size,data,cs,focus_slice)
    }

    pub fn create_float_stack(width: u32, height: u32, size: u32) -> FloatStack {
        let cs : ColorSpace<f32> = ColorSpace::<f32>::Grayf32();
        let data = RefCell::new(vec![RefCell::new(FloatProcessor::create_float_processor(width,height));size.try_into().unwrap()]);
        let focus_slice = Cell::new(0);
        let size = Cell:: new(size);
        return ImageStack::<f32>::create_stack(width,height,size,data,cs,focus_slice)
    }

    pub fn create_color_stack(width: u32, height: u32, size: u32) -> ColorStack {
        let cs : ColorSpace<(u8,u8,u8)> = ColorSpace::<(u8,u8,u8)>::Rgb24();
        let data = RefCell::new(vec![RefCell::new(ColorProcessor::create_color_processor(width,height));size.try_into().unwrap()]);
        let focus_slice = Cell::new(0);
        let size = Cell:: new(size);
        return ImageStack::<(u8,u8,u8)>::create_stack(width,height,size,data,cs,focus_slice)
    }

///Debug///
pub fn debug_stack(&self){
    println!("ImageStacks : Dimensions : {}x{}x{} px, Bit depth : {}, data length : {}", self.get_width_stack(), self.get_height_stack(), self.get_size(), self.get_bit_depth_stack(), self.get_data_stack().len());
    println!("Focus slice : {}", self.get_focus_slice());
}

///Get///

    /// Returns the size of the s
    pub fn get_size(&self) -> u32 {
        return self.size.get()
    }

    /// Returns the width of the stack
    pub fn get_width_stack(&self) -> u32{
        return self.width
    }

    /// returns the height of the stack
    pub fn get_height_stack(&self) -> u32{
        return self.height
    }

    /// returns the index of the focused slice
    pub fn get_focus_slice(&self) -> u32{
        return self.focus_slice.get()
    }

    /// returns the number of colors
    pub fn get_nb_channels_stacks(&self) -> u8{
        return self.cs.get_nb_channels()
    }

    /// Returns the bit debth of the stack's color space
    pub fn get_bit_depth_stack(&self) -> u8{
        return self.cs.get_bit_depth()
    }

    /// Returns a reference to the array of Image Processor
    pub fn get_data_stack(&self) -> RefMut<Vec<RefCell<ImageProcessor<T>>>>{
        return self.data.borrow_mut()
    }
    /*
    /// Returns a reference to the current slice (does not work)
    pub fn get_one_slice(&self)-> RefMut<ImageProcessor<T>>{
        return self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut()
    }
    */

    /// Returns a copy of the focused slice
    pub fn get_one_slice_copy(&self)-> ImageProcessor<T>{
        return self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow().clone()
    }


///Set///

    ///Increases the size of the stack by one. Should generally not be called
    pub fn set_size(&self){
        self.size.set(self.size.get()+1);
    }

    ///Adds an image as a slice as the end of the stack
    pub fn set_data_stack(&self,img: ImageProcessor<T>){
        if img.get_width() != self.get_width_stack() {
            panic!("Width out of bounds ! width stack={}, width image={}",self.get_width_stack(),img.get_width());
        }
        if img.get_height() != self.get_height_stack(){
            panic!("Heigth out of bounds ! height stack={}, height image={}",self.get_height_stack(),img.get_height());
        }
        self.get_data_stack().push(RefCell::new(img));
        self.set_size();
    }
    ///Selects the slice to focus on from its index
    pub fn set_slice_number(&self,slice:u32) {
        if (slice>=self.get_size()) & (self.get_size() !=0) {
            panic!("Slice out of bounds ! Slices range from 0 to {} and you asked for slice numbre {}", (self.get_size()-1),slice);
        }
        self.focus_slice.set(slice);
        println!("Focus slice : {}", self.get_focus_slice());
        self.get_one_slice_copy().debug();
    }
}

impl<T> Access<T> for ImageStack<T> where T:Copy{
    type Output = T;

    ///// Get 1 pixel /////
    fn get_pixel(&self, index: u32) -> Self::Output{
        return self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut().get_pixel(index)
    }

    fn get_pixel_at(&self,x: u32, y: u32) -> Self::Output{
        if x >= self.get_width_stack(){
            panic!("Pixel out of bounds ! x={}, width={}",x,self.get_width_stack());
        }
        if y >= self.get_height_stack(){
            panic!("Pixel out of bounds ! y={}, height={}",y,self.get_height_stack());
        }
        return self.get_pixel(y*self.get_width_stack()+x)
    }
    // No check, faster, but prone to errors
    fn get(&self,index: usize) -> Self::Output{
        return self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut().get_data()[index];
    }


    ///// set 1 Pixel /////
    fn set_pixel(&mut self,index: u32, value: Self::Output){
        if u32::from(index) >= self.get_width_stack()*self.get_height_stack(){
            panic!("Pixel out of bounds ! index = {}, data length : {}",index ,self.get_width_stack()*self.get_height_stack());
        }

        self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut().get_data()[usize::try_from(index).unwrap()] = value;
    }
    fn set_pixel_at(&mut self,x: u32, y: u32, value: Self::Output){
        if x >= self.get_width_stack(){
            panic!("Pixel out of bounds ! x={}, width={}",x,self.get_width_stack());
        }
        if y >= self.get_height_stack(){
            panic!("Pixel out of bounds ! y={}, height={}",y,self.get_height_stack());
        }
        self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut().set_pixel(y*self.get_width_stack()+x,value);
    }
    // No check, faster, but prone to errors
    fn set(&mut self,index: u32, value: Self::Output){
        self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut().get_data()[usize::try_from(index).unwrap()] = value;
    }

    fn get_row(&self,x: u32, y: u32) -> Vec<Self::Output>{
        let mut out : Vec<Self::Output> = Vec::new();
        let width = self.get_width_stack();
        for local_x in x..width{
            out.push(self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut().get(usize::try_from(y*width+local_x).unwrap()));
        }
        return out;
    }
    fn get_column(&self,x: u32, y: u32) -> Vec<Self::Output>{
        let mut out : Vec<Self::Output> = Vec::new();
        let width = self.get_width_stack();
        let height = self.get_height_stack();
        for local_y in y..height{
            out.push(self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut().get(usize::try_from(local_y*width+x).unwrap()));
        }
        return out;
    }

    fn set_row(&mut self,x: u32, y: u32, data: Vec<Self::Output>){
        self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut().set_row(x,y,data);
    }
    fn set_column(&mut self,x: u32, y: u32, data: Vec<Self::Output>){
        self.get_data_stack()[usize::try_from(self.get_focus_slice()).unwrap()].borrow_mut().set_column(x,y,data);
    }

    fn set_slice_number(&self,slice: u32){
        self.set_slice_number(slice);
    }
}
*/

#[cfg(test)]
mod test {

    use crate::color_space::ColorSpace;
    use crate::grayscale::*;
    use crate::image_processor::*;
    use crate::image_stack::*;
    use crate::short_processor::ShortProcessor;

    #[test]
    fn new_stack() {
        let stack = ImageStack::<u8, Gray8>::new(
            3,
            2,
            vec![
                vec![1u8, 2u8, 3u8, 4u8, 5u8, 6u8],
                vec![11u8, 12u8, 13u8, 14u8, 15u8, 16u8],
                vec![21u8, 22u8, 23u8, 24u8, 25u8, 26u8],
                vec![31u8, 32u8, 33u8, 34u8, 35u8, 36u8],
            ],
            // labels: vec![String::from("001"),String::from("002"),String::from("003"),String::from("004")],
            Gray::<u8>::new(),
        );
        let answer = vec![
            vec![1u8, 2u8, 3u8, 4u8, 5u8, 6u8],
            vec![11u8, 12u8, 13u8, 14u8, 15u8, 16u8],
            vec![21u8, 22u8, 23u8, 24u8, 25u8, 26u8],
            vec![31u8, 32u8, 33u8, 34u8, 35u8, 36u8],
        ];
        assert!(stack.data().iter().all(|row| answer.contains(row)));
        assert_eq!(stack.labels()[0..4], vec!["1", "2", "3", "4"]);
    }

    #[test]
    fn macro_fill_with_fortyfive_u8_dimension() {
        let stack = stack_fill_with!(45, u8, 2, 3, 4);
        assert_eq!(stack.get_width(), 2);
        assert_eq!(stack.get_height(), 3);
        assert_eq!(stack.n_slices(), 4);
    }

    #[test]
    fn macro_fill_with_fortyfive_u8_content() {
        let stack = stack_fill_with!(45, u8, 2, 3, 4);
        assert_eq!(stack.data().len(), 4);
        assert_eq!(stack.data()[0], vec![45u8; 6]);
        assert_eq!(stack.data()[1], vec![45u8; 6]);
        assert_eq!(stack.data()[2], vec![45u8; 6]);
    }

    #[test]
    fn macro_fill_with_fortyfive_u8_labels() {
        let stack = stack_fill_with!(45, u8, 2, 3, 4);
        assert_eq!(stack.labels()[0..4], vec!["1", "2", "3", "4"]);
    }

    #[test]
    fn macro_fill_with_u16() {
        let stack = stack_fill_with!(32768, u16, 2, 3, 4);
        assert_eq!(stack.labels()[0..4], vec!["1", "2", "3", "4"]);
    }

    #[test]
    fn slice_set_get_label() {
        let mut stack = stack_fill_with!(45, u8, 2, 3, 4);
        stack.set_slice_label(String::from("New Label"), 3);
        assert_eq!(stack.get_slice_label(3).unwrap(), "New Label".to_string());
    }

    #[test]
    fn slice_add() {
        let mut stack = stack_fill_with!(32768, u16, 2, 3, 4);
        let short_ip = ShortProcessor::new(
            2,
            3,
            vec![65535u16, 65535u16, 65535u16, 65535u16, 65535u16, 65535u16],
            Gray16::new(),
        );
        stack.add_slice("last".to_string(), &short_ip);
        assert_eq!(stack.labels()[4], "last");
        assert_eq!(
            stack.data()[4],
            vec![65535u16, 65535u16, 65535u16, 65535u16, 65535u16, 65535u16]
        );
    }

    /*
        #[test]
        fn test_ImageProcessor_get_pixel(){
           let img= ByteProcessor::create_byte_processor(10,15);
            assert_eq!(img.get_pixel(0),0);
        }

        #[test]
        #[should_panic(expected = "Pixel out of bounds ! index = 200, data length : 200")]
        fn test_ImageProcessor_get_pixel_panic(){
           let img= ByteProcessor::create_byte_processor(10,20);
           img.get_pixel(200);
        }

        #[test]
        fn test_ImageProcessor_get_pixel_at(){
           let img= ByteProcessor::create_byte_processor(10,15);
            assert_eq!(img.get_pixel_at(0,1),0);
        }

        #[test]
        #[should_panic(expected = "Pixel out of bounds ! x=10, width=10")]
        fn test_ImageProcessor_get_pixel_at_panic_width(){
           let img= ByteProcessor::create_byte_processor(10,15);
            img.get_pixel_at(10,0);
        }

        #[test]
        #[should_panic(expected = "Pixel out of bounds ! y=15, height=15")]
        fn test_ImageProcessor_get_pixel_at_panic_height(){
           let img= ByteProcessor::create_byte_processor(10,15);
            img.get_pixel_at(0,15);
        }

        #[test]
        fn test_ImageProcessor_get(){
            let img= ByteProcessor::create_byte_processor(10,15);
             assert_eq!(img.get(0),0);
        }

        #[test]
        fn test_ImageProcessor_set_pixel(){
            let mut img= ByteProcessor::create_byte_processor(2,2);
            img.set_pixel(0,10);
            assert_eq!(img.get_pixel(0),10);
        }

        #[test]
        #[should_panic(expected = "Pixel out of bounds ! index = 4, data length : 4")]
        fn test_ImageProcessor_set_pixel_panic(){
            let mut img= ByteProcessor::create_byte_processor(2,2);
            img.set_pixel(4,10);
        }

        #[test]
        fn test_ImageProcessor_set_pixel_color(){
            let mut img= ColorProcessor::create_color_processor(2,2);
            img.set_pixel(0,(10,0,16));
            assert_eq!(img.get_pixel(0),(10,0,16));
        }

        #[test]
        fn test_ImageProcessor_set_pixel_at(){
            let mut img= ByteProcessor::create_byte_processor(2,2);
            img.set_pixel_at(0,0,10);
            assert_eq!(img.get_pixel(0),10);
        }

        #[test]
        #[should_panic(expected = "Pixel out of bounds ! x=4, width=2")]
        fn test_ImageProcessor_set_pixel_at_panic_width(){
            let mut img= ByteProcessor::create_byte_processor(2,2);
            img.set_pixel_at(4,0,10);
        }

        #[test]
        #[should_panic(expected = "Pixel out of bounds ! y=4, height=2")]
        fn test_ImageProcessor_set_pixel_at_panic_height(){
            let mut img= ByteProcessor::create_byte_processor(2,2);
            img.set_pixel_at(0,4,10);
        }

        #[test]
        fn test_ImageProcessor_set_pixel_at_color(){
            let mut img= ColorProcessor::create_color_processor(2,2);
            img.set_pixel_at(0,0,(10,0,16));
            assert_eq!(img.get_pixel(0),(10,0,16));
        }

        #[test]
        fn test_ImageProcessor_set_pixel_at_float(){
            let mut img= FloatProcessor::create_float_processor(2,2);
            img.set_pixel_at(0,0,10.2);
            assert_eq!(img.get_pixel(0),10.2);
        }

        #[test]
        fn test_ImageProcessor_set_column(){
            let mut img= ColorProcessor::create_color_processor(2,2);
            let vec = vec![(1,1,1),(14,0,20)];
            img.set_column(0,0,vec![(1,1,1),(14,0,20)]);
            let row = img.get_column(0,0);
            assert_eq!(row,vec);
        }

        #[test]
        #[should_panic(expected = "You cannot set the slice number for a single image")]
        fn test_ImageProcessor_set_slice_number(){
            let mut img= ColorProcessor::create_color_processor(2,2);
            img.set_slice_number(1);
        }

        #[test]
        fn test_ImageStack_get_pixel(){
            let stack= ByteStack::create_byte_stack(10,15,12);
            assert_eq!(stack.get_pixel(0),0);
        }

        #[test]
        fn test_ImageStack_get_pixel_at(){
            let stack= ByteStack::create_byte_stack(10,15,12);
            assert_eq!(stack.get_pixel_at(0,1),0);
        }

        #[test]
        #[should_panic(expected = "Pixel out of bounds ! x=20, width=10")]
        fn test_ImageStack_get_pixel_at_panic_width(){
            let stack= FloatStack::create_float_stack(10,15,12);
            stack.get_pixel_at(20,1);
        }

        #[test]
        #[should_panic(expected = "Pixel out of bounds ! y=30, height=15")]
        fn test_ImageStack_get_pixel_at_panic_height(){
            let stack= ColorStack::create_color_stack(10,15,12);
            stack.get_pixel_at(0,30);
        }

        #[test]
        fn test_ImageStack_get(){
            let stack= FloatStack::create_float_stack(10,15,12);
            assert_eq!(stack.get(0),0.0);
        }

        #[test]
        fn test_ImageStack_set_pixel(){
            let mut stack= FloatStack::create_float_stack(10,15,12);
            stack.set_pixel(0,20.5);
            assert_eq!(stack.get_pixel(0),20.5);
        }

        #[test]
        #[should_panic(expected = "Pixel out of bounds ! index = 300, data length : 150")]
        fn test_ImageStack_set_pixel_panic(){
            let mut stack= FloatStack::create_float_stack(10,15,12);
            stack.set_pixel(300,20.5);
        }

        #[test]
        fn test_ImageStack_set_pixel_at(){
            let mut stack= ColorStack::create_color_stack(10,15,12);
            stack.set_pixel_at(0,0,(10,50,60));
            assert_eq!(stack.get_pixel(0),(10,50,60));
        }

        #[test]
        #[should_panic(expected = "Pixel out of bounds ! x=10, width=10")]
        fn test_ImageStack_set_pixel_at_panic_width(){
            let mut stack= ColorStack::create_color_stack(10,15,12);
            stack.set_pixel_at(10,0,(10,50,60));
        }

        #[test]
        #[should_panic(expected = "Pixel out of bounds ! y=15, height=15")]
        fn test_ImageStack_set_pixel_at_panic_height(){
            let mut stack= ByteStack::create_byte_stack(10,15,12);
            stack.set_pixel_at(0,15,30);
        }

        #[test]
        fn test_ImageStack_set(){
            let mut stack= ByteStack::create_byte_stack(10,15,12);
            stack.set(0,30);
            assert_eq!(stack.get_pixel(0),30);
        }

        #[test]
        fn test_ImageStack_get_row(){
            let stack= ByteStack::create_byte_stack(2,2,12);
            let vec = vec![0,0];
            assert_eq!(stack.get_row(0,0),vec);
        }

        #[test]
        fn test_ImageStack_get_column(){
            let stack= ByteStack::create_byte_stack(2,2,12);
            let vec = vec![0,0];
            assert_eq!(stack.get_column(0,0),vec);
        }

        #[test]
        fn test_ImageStack_set_row(){
            let mut stack= ColorStack::create_color_stack(10,15,12);
            let vec = vec![(255,50,8)];
            stack.set_row(0,0,vec![(255,50,8)]);
            assert_eq!(stack.get_row(0,0)[0],vec[0]);
        }

        #[test]
        fn test_ImageStack_set_column(){
            let mut stack= ColorStack::create_color_stack(10,15,12);
            let vec = vec![(255,50,8)];
            stack.set_column(2,2,vec![(255,50,8)]);
            assert_eq!(stack.get_column(2,0)[2],vec[0]);
        }

        #[test]
        fn test_ImageStack_set_slice_number(){
            let mut stack= FloatStack::create_float_stack(10,15,12);
            stack.set_slice_number(11);
            assert_eq!(stack.get_focus_slice(),11);
        }

    }


    #[cfg(test)]
    mod test{
        //use super ::super::ImageStack::{*};
        //use super :: super:: ImageProcessor::{*};
        use crate::image_stack::ImageStack;
        use crate::image_stack::ImageProcessor;
        use crate::image_stack::ByteStack;
        use crate::image_stack::ByteProcessor;
        use crate::image_stack::FloatProcessor;
        use crate::image_stack::FloatStack;
        use crate::color_space::ColorSpace;
        use crate::image_stack::ColorStack;
        use crate::image_stack::ColorProcessor;
        use core::cell::RefCell;
        use core::cell::Cell;

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


        #[test]
        fn byte_processor_add_byte(){
            let img = ByteProcessor::create_byte_processor(10,10);
            img.add(10);
            assert_eq!(img.get(3),10);
        }

        #[test]
        fn byte_processor_add_float(){
            let img = FloatProcessor::create_float_processor(10,10);
            img.add(4.0);
            assert_eq!(img.get(3),4.0);
        }

        #[test]
        fn byte_processor_subtract_byte(){
            let img = ByteProcessor::create_byte_processor(10,10);
            img.add(20);
            img.subtract(10);
            assert_eq!(img.get(3),10);
        }

        #[test]
        fn byte_processor_substract_underflow(){
            let img = ByteProcessor::create_byte_processor(10,10);
            img.substract(10);
            assert_eq!(img.get(3),0);
        }


        #[test]
        fn byte_processor_add_substract_stack(){
            let img = ByteStack::create_byte_stack(10,10,10);
            img.add(20);
            img.substract(4);
            assert_eq!(img.get(3),16);
        }

        #[test]
        fn byte_processor_multiply_byte(){
            let img = ByteProcessor::create_byte_processor(10,10);
            img.add(10);
            img.multiply(2);
            assert_eq!(img.get(3),20);
        }

        #[test]
        fn byte_processor_multiply_float(){
            let img = FloatProcessor::create_float_processor(10,10);
            img.add(4.0);
            img.multiply(3.0);
            assert_eq!(img.get(3),12.0);
        }

        #[test]
        fn byte_processor_divide_byte(){
            let img = ByteProcessor::create_byte_processor(10,10);
            img.add(30);
            img.divide(10);
            assert_eq!(img.get(3),3);
        }

        #[test]
        fn byte_processor_divide_float(){
            let img = FloatProcessor::create_float_processor(10,10);
            img.add(20.0);
            img.divide(5.0);
            assert_eq!(img.get(3),4.0);
        }

        #[test]
        fn byte_processor_divide_stack(){
            let img = ByteStack::create_byte_stack(10,10,10);
            img.add(20);
            img.divide(2);
            assert_eq!(img.get(3),10);
        }

        #[test]
        fn byte_processor_multiply_stack(){
            let img = FloatStack::create_float_stack(10,10,10);
            img.add(20.0);
            img.multiply(3.0);
            assert_eq!(img.get(3),60.0);
        }

        #[test]
        fn byte_processor_floor_byte(){
            let img = ByteProcessor::create_byte_processor(10,10);
            img.floor(3);
            assert_eq!(img.get(3),3);
        }

        #[test]
        fn byte_processor_floor_float(){
            let img = FloatProcessor::create_float_processor(10,10);
            img.floor(4.0);
            assert_eq!(img.get(3),4.0);
        }

        #[test]
        fn byte_processor_floor_stack(){
            let img = ByteStack::create_byte_stack(10,10,10);
            img.add(20);
            img.floor(6);
            assert_eq!(img.get(3),20);
        }

        #[test]
        fn byte_processor_ceil_byte(){
            let img = ByteProcessor::create_byte_processor(10,10);
            img.add(20);
            img.ceil(12);
            assert_eq!(img.get(3),12);
        }

        #[test]
        fn byte_processor_ceil_float(){
            let img = FloatProcessor::create_float_processor(10,10);
            img.add(20.0);
            img.ceil(4.0);
            assert_eq!(img.get(3),4.0);
        }

        #[test]
        fn byte_processor_ceil_stack(){
            let img = ByteStack::create_byte_stack(10,10,10);
            img.add(20);
            img.ceil(6);
            assert_eq!(img.get(3),6);
        }

        #[test]
        #[ignore] //Aléatoire, échoue parfois, réussi parfois
        fn byte_processor_noise_byte(){
            let img = ByteProcessor::create_byte_processor(10,10);
            img.add(100);
            img.noise(10.0);
            assert_ne!(img.get(3),100);
        }

        #[test]
        #[ignore] //Aléatoire, échoue parfois, réussi parfois
        fn byte_processor_noise_float(){
            let img = FloatProcessor::create_float_processor(10,10);
            img.noise(2.0);
            assert_ne!(img.get(3),0.0);
        }

        #[test]
        #[ignore] //Aléatoire, échoue parfois, réussi parfois
        fn byte_processor_noise_stack(){
            let img = FloatStack::create_float_stack(10,10,10);
            img.noise(3.0);
            assert_ne!(img.get(3),0.0);
        }

        #[test]
        fn byte_processor_abs_byte(){
            let img = ByteProcessor::create_byte_processor(10,10);
            img.add(100);
            img.abs();
            assert_eq!(img.get(3),100);
        }

        #[test]
        fn byte_processor_abs_float(){
            let img = FloatProcessor::create_float_processor(10,10);
            img.add(100.0);
            img.abs();
            assert_eq!(img.get(3),-100.0);
        }

        #[test]
        fn byte_processor_abs_stack(){
            let img = FloatStack::create_float_stack(10,10,10);
            img.add(100.0);
            img.abs();
            assert_eq!(img.get(3),-100.0);
        }

        #[test]
        fn byte_processor_exp_byte(){
            let img = ByteProcessor::create_byte_processor(10,10);
            img.add(2);
            img.exp();
            assert_eq!(img.get(3),7);
        }

        #[test]
        fn byte_processor_exp_float(){
            let img = FloatProcessor::create_float_processor(10,10);
            img.add(3.0);
            img.exp();
            assert_eq!(img.get(3),20.085537);
        }

        #[test]
        fn byte_processor_exp_stack(){
            let img = FloatStack::create_float_stack(10,10,10);
            img.add(4.0);
            img.exp();
            assert_eq!(img.get(3),54.59815);
        }

        #[test]
        fn byte_processor_sqrt_byte(){
            let img = ByteProcessor::create_byte_processor(10,10);
            img.add(4);
            img.sqrt();
            assert_eq!(img.get(3),2);
        }

        #[test]
        fn byte_processor_sqrt_float(){
            let img = FloatProcessor::create_float_processor(10,10);
            img.add(9.0);
            img.sqrt();
            assert_eq!(img.get(3),3.0);
        }

        #[test]
        fn byte_processor_sqrt_stack(){
            let img = ByteStack::create_byte_stack(10,10,10);
            img.add(16);
            img.sqrt();
            assert_eq!(img.get(3),4);
        }

        #[test]
        fn byte_processor_ln_byte(){
            let img = ByteProcessor::create_byte_processor(10,10);
            img.add(35);
            img.ln();
            assert_eq!(img.get(3),3);
        }

        #[test]
        fn byte_processor_ln_float(){
            let img = FloatProcessor::create_float_processor(10,10);
            img.add(29.0);
            img.ln();
            assert_eq!(img.get(3),3.3672957);
        }

        #[test]
        fn byte_processor_ln_stack(){
            let img = ByteStack::create_byte_stack(10,10,10);
            img.add(99);
            img.ln();
            assert_eq!(img.get(3),4);
        }

        #[test]
        fn byte_processor_log_byte(){
            let img = ByteProcessor::create_byte_processor(10,10);
            img.add(35);
            img.log();
            assert_eq!(img.get(3),1);
        }

        #[test]
        fn byte_processor_log_float(){
            let img = FloatProcessor::create_float_processor(10,10);
            img.add(29.0);
            img.log();
            assert_eq!(img.get(3),1.4623979);
        }

        #[test]
        fn byte_processor_log_stack(){
            let img = ByteStack::create_byte_stack(10,10,10);
            img.add(200);
            img.log();
            assert_eq!(img.get(3),2);
        }




        #[test]
        fn test_create_byte_stack(){
            let stack = ByteStack {width: 10,
                height: 15,
                size: Cell:: new(12),
                data: RefCell::new(vec![RefCell::new(ByteProcessor::create_byte_processor(10,15));12]),
                cs : ColorSpace::<u8>::Gray8(),
                focus_slice: Cell::new(0)
                };
            assert_eq!(ByteStack::create_byte_stack(10,15,12), stack);
        }

        #[test]
        fn test_create_float_stack(){
            let stack = FloatStack {width: 10,
                height: 15,
                size: Cell:: new(12),
                data: RefCell::new(vec![RefCell::new(FloatProcessor::create_float_processor(10,15));12]),
                cs : ColorSpace::<f32>::Grayf32(),
                focus_slice: Cell::new(0)
                };
            assert_eq!(FloatStack::create_float_stack(10,15,12), stack);
        }


        #[test]
        fn test_create_color_stack(){
            let stack = ColorStack {width: 10,
                height: 15,
                size: Cell:: new(12),
                data: RefCell::new(vec![RefCell::new(ColorProcessor::create_color_processor(10,15));12]),
                cs : ColorSpace::<(u8,u8,u8)>::Rgb24(),
                focus_slice: Cell::new(0)
                };
            assert_eq!(ColorStack::create_color_stack(10,15,12), stack);
        }

        #[test]
        fn test_get_size(){
            let stack = ByteStack {width: 10,
                height: 15,
                size: Cell:: new(12),
                data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
                cs : ColorSpace::<u8>::Gray8(),
                focus_slice: Cell::new(0)
                };
            assert_eq!(stack.get_size(),12);
        }

        #[test]
        fn test_get_width_stack(){
            let stack = ByteStack {width: 10,
                height: 15,
                size: Cell:: new(12),
                data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
                cs : ColorSpace::<u8>::Gray8(),
                focus_slice: Cell::new(0)
                };
            assert_eq!(stack.get_width_stack(),10);
        }

        #[test]
        fn test_get_height_stack(){
            let stack = ByteStack {width: 10,
                height: 15,
                size: Cell:: new(12),
                data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
                cs : ColorSpace::<u8>::Gray8(),
                focus_slice: Cell::new(0)
                };
            assert_eq!(stack.get_height_stack(),15);
        }

        #[test]
        //should_get_focus_slice_equal_to_0
        fn test_get_focus_slice(){
            let stack = ByteStack {width: 10,
                height: 15,
                size: Cell:: new(12),
                data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
                cs : ColorSpace::<u8>::Gray8(),
                focus_slice: Cell::new(0)
                };
            assert_eq!(stack.get_focus_slice(),0);
        }

        #[test]
        fn test_get_nb_channels_stacks(){
            let stack = ByteStack {width: 10,
                height: 15,
                size: Cell:: new(12),
                data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
                cs : ColorSpace::<u8>::Gray8(),
                focus_slice: Cell::new(0)
                };
            assert_eq!(stack.get_nb_channels_stacks(),1);
        }

        #[test]
        fn test_get_bit_depth_stack(){
            let stack = ByteStack {width: 10,
                height: 15,
                size: Cell:: new(12),
                data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
                cs : ColorSpace::<u8>::Gray8(),
                focus_slice: Cell::new(0)
                };
            assert_eq!(stack.get_bit_depth_stack(),8);
        }

        /*#[test]
        fn test_get_data_stack(){
            let stack = ByteStack {width: 10,
                height: 15,
                size: Cell:: new(12),
                data: RefCell::new(vec![ImageProcessor::<u8>::create_byte_processor(10,15);12]),
                cs : ColorSpace::Gray8(),
                focus_slice: Cell::new(0)
                };
            assert_eq!(stack.get_data_stack(),vec![ImageProcessor::<u8>::create_byte_processor(10,15);12]);
        }*/

        #[test]
        fn test_get_one_slice(){
            let stack = ByteStack {width: 10,
                height: 15,
                size: Cell:: new(12),
                data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
                cs : ColorSpace::<u8>::Gray8(),
                focus_slice: Cell::new(0)
                };
                let img =ImageProcessor::<u8>::create_byte_processor(10, 15);
            assert_eq!(stack.get_one_slice_copy(),img);
        }


        #[test]
        fn test_set_size(){
            let stack = ByteStack {width: 10,
                height: 15,
                size: Cell:: new(12),
                data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
                cs : ColorSpace::<u8>::Gray8(),
                focus_slice: Cell::new(0)
                };
            stack.set_size();
            assert_eq!(stack.get_size(),13);
        }

        #[test]
        fn test_set_data_stack(){
            let stack = ByteStack {width: 10,
                height: 15,
                size: Cell:: new(12),
                data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
                cs : ColorSpace::<u8>::Gray8(),
                focus_slice: Cell::new(0)
                };
            let img =ImageProcessor::<u8>::create_byte_processor(10, 15);
            stack.set_data_stack(img);
            assert_eq!(stack.get_size(),13);
        }


        #[test]
        #[should_panic(expected = "Width out of bounds ! width stack=10, width image=1")]
        fn test_set_data_stack_panic_width(){
            let stack = ByteStack {width: 10,
                height: 15,
                size: Cell:: new(12),
                data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
                cs : ColorSpace::<u8>::Gray8(),
                focus_slice: Cell::new(0)
                };
            let img =ImageProcessor::<u8>::create_byte_processor(1, 15);
            stack.set_data_stack(img);
        }

        #[test]
        #[should_panic(expected = "Heigth out of bounds ! height stack=15, height image=1")]
        fn test_set_data_stack_panic_heigth(){
            let stack = ByteStack {width: 10,
                height: 15,
                size: Cell:: new(12),
                data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
                cs : ColorSpace::<u8>::Gray8(),
                focus_slice: Cell::new(0)
                };
            let img =ImageProcessor::<u8>::create_byte_processor(10, 1);
            stack.set_data_stack(img);
        }

        #[test]
        fn test_set_slice_number(){
            let stack = ByteStack {width: 10,
                height: 15,
                size: Cell:: new(12),
                data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
                cs : ColorSpace::<u8>::Gray8(),
                focus_slice: Cell::new(0)
                };
            stack.set_slice_number(11);
            assert_eq!(stack.get_focus_slice(),11);
        }

        #[test]
        #[should_panic(expected = "Slice out of bounds ! Slices range from 0 to 11 and you asked for slice numbre 12")]
        fn test_set_slice_number_panic(){
            let stack = ByteStack {width: 10,
                height: 15,
                size: Cell:: new(12),
                data: RefCell::new(vec![RefCell::new(ImageProcessor::<u8>::create_byte_processor(10,15));12]),
                cs : ColorSpace::<u8>::Gray8(),
                focus_slice: Cell::new(0)
                };
            stack.set_slice_number(12);
        }
        */
}
