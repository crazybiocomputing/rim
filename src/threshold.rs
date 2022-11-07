//
//  RIM - Rust Image
//  Copyright (C) 2022  Jean-Christophe Taveau.
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


pub trait Threshold {

    fn auto_threshold() {}
    /// Converts the image to binary using an automatically determined threshold.
    /// Image_Processor bin​(i32 shrink_Factor) {}
    /// Returns a copy of this image that has been reduced in size using binning.


    /// Returns a pixel value (threshold) that can be used to divide the image i32o objects and background.
    fn  get_auto_threshold() -> T {};

    /// This is a version of get_Auto_Threshold() that uses a histogram passed as an argument.
    fn get_auto_threshold​(i32[] histogram) -> T {};

    /// Returns the upper threshold level.
    f64 get_Max_Threshold() {};

    /// Returns the lower threshold level.
    f64 get_min_threshold() {};

    /// Resets the threshold if mi32hreshold=max_Threshold and lut_Update_Mode=NO_LUT_UPDATE.
    fn reset_binary_threshold() {};

    /// Disables thresholding.
    /// Image_Processor resize​(i32 dst_Width) {}
    /// Returns a new Image_Processor containing a scaled copy of this image or ROI, with the aspect ratio mai32ained.
    fn reset_threshold() {};
    
    /// Automatically sets the lower and upper threshold levels, 
    /// where 'method' must be ISODATA or ISODATA2 and 
    /// 'lut_Update' must be RED_LUT, BLACK_AND_WHITE_LUT, OVER_UNDER_LUT or NO_LUT_UPDATE.
    fn set_Auto_Threshold​(i32 method, i32 lut_Update) {};

    fn set_Auto_Threshold​(Auto_Thresholder.Method method, bool dark_Background;

    fn set_Auto_Threshold​(Auto_Thresholder.Method method, bool dark_Background, i32 lut_Update;

    /// Automatically sets the lower and upper threshold levels, 
    /// where 'method' must be "Default", "Huang", "i32ermodes", 
    /// "Iso_Data", "IJ_Iso_Data", "Li", "Max_Entropy", "Mean", 
    /// "Min_Error", "Minimum", "Moments", "Otsu", "Percentile", 
    /// "Renyi_Entropy", "Shanbhag", "Triangle" or "Yen".
    fn set_Auto_Threshold​(java.lang.String method) {};

    fn set_Auto_Threshold​(java.lang.String m_String, bool dark_Background, i32 lut_Update)  {};

    fn set_Binary_Threshold() {};

   fn set_threshold​(f64 min_threshold, f64 max_threshold) {}
    /// Sets the lower and upper threshold levels using NO_LUT_UPDATE.

    fn set_threshold​(f64 min_threshold, f64 max_threshold, i32 lut_Update) {}
    /// Sets the lower and upper threshold levels.

    fn threshold​(i32 level) {}
    /// Sets pixels less than or equal to level to 0 and all other pixels to 255.

    /// Set the lower Over/Under thresholding color.
    static fn set_under_color​(i32 red, i32 green, i32 blue) {}

 
}

