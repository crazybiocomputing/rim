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

#![allow(non_camel_case_types)]
#![allow(unused)]

use crate::color_space::ColorSpace;
use crate::gray_processor::*;
use crate::grayscale::Gray;
use crate::image_processor::ImageProcessor;
use crate::image_traits::Access;
use crate::statistics::Statistics;

// Alias
pub type FloatProcessor = ImageProcessor<f32, Gray<f32>>;

impl Statistics<f32> for FloatProcessor {
    type Output = u16;
    type Output_f32 = f32;

    fn update_stats(&mut self) {
        if self.metadata.stats.is_dirty() {
            let mut sum: f64 = 0.0;
            let mut sum2: f64 = 0.0;
            let mut hist = vec![0u32; 255];
            let start = (self.metadata.roi.y() * self.width + self.metadata.roi.x()) as usize;
            let mut mi: f64 = self.data[start] as f64;
            let mut mx = mi;
            let mut count: u32 = 0;
            for y in self.metadata.roi.y()..(self.metadata.roi.y() + self.metadata.roi.height()) {
                let mut i = y * self.width + self.metadata.roi.x();
                for x in self.metadata.roi.x()..(self.metadata.roi.x() + self.metadata.roi.width())
                {
                    let v: f64 = self.getf(i as usize) as f64;
                    let index: usize = self.get(i as usize) as usize;
                    sum += v as f64;
                    sum2 += (v * v) as f64;
                    i += 1;
                    hist[(((v as f32) / f32::MAX) * (255 as f32)) as usize] += 1;
                    mi = if v < mi { v } else { mi };
                    mx = if v > mx { v } else { mx };
                    count += 1;
                }
            }
            let mut std_dev = (count as f64 * sum2 - sum * sum) / count as f64;
            std_dev = if std_dev > 0.0 {
                (std_dev / (count as f64 - 1.0_f64)).sqrt()
            } else {
                0.0
            };
            self.metadata
                .set_stats(&hist, mi, mx, sum / (count as f64), std_dev);
        }
    }

    fn histogram(&self) -> &Vec<u32> {
        self.metadata.get_histogram()
    }

    fn min_value(&self) -> f64 {
        self.metadata.get_min()
    }

    fn max_value(&self) -> f64 {
        self.metadata.get_max()
    }

    fn mean(&self) -> f64 {
        self.metadata.get_mean()
    }
    fn standard_deviation(&self) -> f64 {
        self.metadata.get_std_dev()
    }
}

/*
// ... or hard-coded class from trait...
struct FloatProcessor {
    /// Field Summary
    /// Fields Modifier and Type Field Description
    protected bool antialiased_Text
    protected java.awt.image.Color_Model base_CM
    static i32 BICUBIC {}
    i32erpolation methods
    static i32 BILINEAR {}
    i32erpolation methods
    static i32 BLACK {}
    /// Value of pixels included in masks.
    static i32 BLACK_AND_WHITE_LUT
    static i32 BLUR_MORE
    protected byte[] b_LUT1
    protected byte[] b_LUT2
    protected bool bold_Font
    static i32 CENTER_JUSTIFY {}
    /// Center justify text.
    protected i32 clip_XMax
    protected i32 clip_XMin
    protected i32 clip_YMax
    protected i32 clip_YMin
    protected java.awt.image.Color_Model cm
    protected java.awt.image.Color_Model cm2
    static i32 CONVOLVE
    protected float[] c_Table
    protected i32 cx
    protected i32 cy
    protected static java.awt.image.Index_Color_Model default_Color_Model
    protected java.awt.Color drawing_Color
    protected bool fill_Value_Set
    static i32 FIND_EDGES
    protected java.awt.Graphics2D fm_Graphics
    protected java.awt.image.Buffered_Image fm_Image
    protected java.awt.Font font
    protected java.awt.Font_Metrics font_Metrics
    protected byte[] g_LUT1
    protected byte[] g_LUT2
    protected i32 height
    protected f64 histogram_Max
    protected f64 histogram_Min
    protected i32 histogram_Size
    protected java.awt.image.Buffered_Image image
    protected java.awt.Image ip
    protected bool i32erpolate
    protected i32 i32erpolation_Method
    protected bool inversion_Tested
    static i32 INVERT_PROJECTION {}
    /// Composite image projection modes.
    protected bool inverted_Lut
    static i32 ISODATA {}
    /// Isodata thresholding method
    static i32 ISODATA2 {}
    /// Modified isodata method used in Image/Adjust/Threshold tool
    protected i32 justification
    static i32 LEFT_JUSTIFY {}
    /// Left justify text.
    protected i32 line_Width
    protected bool lut_Animation
    protected i32 lut_Update_Mode
    static i32 MAX
    static i32 MAX_PROJECTION {}
    /// Composite image projection modes.
    protected f64 max_Threshold
    static i32 MEDIAN_FILTER
    static i32 MIN
    static i32 MIN_PROJECTION {}
    /// Composite image projection modes.
    protected bool min_Max_Set
    protected f64 mi32hreshold
    static i32 NEAREST_NEIGHBOR {}
    i32erpolation methods
    protected bool new_Pixels
    static i32 NO_LUT_UPDATE
    static f64 NO_THRESHOLD {}
    /// Value returned by get_Mi32hreshold() when thresholding is not enabled.
    static i32 NONE {}
    i32erpolation methods
    static i32 OVER_UNDER_LUT
    protected java.awt.image.Writable_Raster raster
    static i32 RED_LUT
    static i32 RIGHT_JUSTIFY {}
    /// Right justify text.
    protected byte[] r_LUT1
    protected byte[] r_LUT2
    protected static java.util.Random rnd
    protected i32 roi_Height
    protected i32 roi_Width
    protected i32 roi_X
    protected i32 roi_Y
    protected java.awt.image.Sample_Model sample_Model
    protected static f64 seed
    static i32 SET_FIRST_CHANNEL {}
    /// Composite image projection modes.
    protected i32 snapshot_Height
    protected i32 snapshot_Width
    protected java.awt.image.Memory_Image_Source source
    static i32 SUM_PROJECTION {}
    /// Composite image projection modes.
    static i32 UPDATE_BLUE {}
    /// Composite image projection modes.
    static i32 UPDATE_GREEN {}
    /// Composite image projection modes.
    static i32 UPDATE_RED {}
    /// Composite image projection modes.
    protected i32 width
    protected i32 x_Max
    protected i32 x_Min
    protected i32 y_Max
    protected i32 y_Min


}


impl FloatProcessor  {

    /// Constructor Summary
    /// Constructors Constructor Description
    /// Image_Processor()
    with_array2D(T[][] array) {}

    /// Creates a Float_Processor from a 2D float array using the default LUT.
    /// Float_Processor​(int[][] array)
    /// Creates a Float_Processor from a 2D int array.
    /// Creates a blank Float_Processor using the default grayscale LUT that displays zero as black.
    blank(int width, int height) {}

    /// Creates a Float_Processor from a double array using the default grayscale LUT.
    /// Float_Processor​(int width, int height, float[] pixels)
    /// Creates a new Float_Processor using the specified pixel array.

    /// Creates a new Float_Processor using the specified pixel array and Color_Model.
    with_color_model(int width, int height, float[] pixels, java.awt.image.Color_Model cm) {}


impl FloatProcessor for ImageProcessor {


}

impl Operator for FloatProcessor {

    /// Adds float 'value' to each pixel in the image or ROI.
    fn add​_float(&self, value: f64) {
      let f = |v:f32,x:i32,y:i32, z:i32,w:i32,h:i32,a:f32,d:f32| -> f32 {
        v + value
      }
      self.apply_func(f);
    };
}

*/

#[cfg(test)]
mod tests {

    use super::*;

    use crate::image_processor::*;
    use crate::image_traits::Access;
    use crate::operator::Operator;

    #[test]
    fn get_pixel_at_xy() {
        let ip = FloatProcessor::new(
            3,
            3,
            vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9],
            Gray::<f32>::new(),
        );
        let px = ip.get_pixel_at(1, 1).unwrap();
        assert_eq!(px, 0.5);
    }

    #[test]
    fn get_pixel_rgb_from_index() {
        let ip = FloatProcessor::new(
            3,
            3,
            vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9],
            Gray::<f32>::new(),
        );
        let px = ip.get_pixel(7).unwrap();
        assert_eq!(px, 0.7);
    }

    #[test]
    fn float_processor_add() {
        let mut ip = FloatProcessor::new(3, 3, vec![0.0; 9], Gray::<f32>::new());
        ip.add(4.5);
        assert_eq!(ip.get(3), 4.5);
    }

    #[test]
    fn float_processor_substract() {
        let mut ip = FloatProcessor::new(3, 3, vec![0.0; 9], Gray::<f32>::new());
        ip.add(20.0);
        ip.subtract(4.0);
        assert_eq!(ip.get(3), 16.0);
    }

    #[test]
    fn float_processor_multiply() {
        let mut ip = FloatProcessor::new(3, 3, vec![0.0; 9], Gray::<f32>::new());
        ip.add(4.0);
        ip.multiply(3.0);
        assert_eq!(ip.get(3), 12.0);
    }

    #[test]
    fn float_processor_divide() {
        let mut ip = FloatProcessor::new(3, 3, vec![0.0; 9], Gray::<f32>::new());
        ip.add(20.0);
        ip.divide(5.0);
        assert_eq!(ip.get(3), 4.0);
    }

    #[test]
    fn float_processor_floor() {
        let mut ip = FloatProcessor::new(3, 3, vec![0.54321; 9], Gray::<f32>::new());
        ip.floor(4.0);
        assert_eq!(ip.get(3), 4.0);
    }

    #[test]
    fn float_processor_ceil() {
        let mut ip = FloatProcessor::new(3, 3, vec![0.54321; 9], Gray::<f32>::new());
        ip.add(20.0);
        ip.ceil(4.0);
        assert_eq!(ip.get(3), 4.0);
    }

    #[test]
    #[ignore] //Aléatoire, échoue parfois, réussi parfois
    fn float_processor_noise() {
        let mut ip = FloatProcessor::new(3, 3, vec![0.54321; 9], Gray::<f32>::new());
        ip.noise(2.0);
        assert_ne!(ip.get(3), 0.0);
    }

    #[test]
    fn float_processor_abs() {
        let mut ip = FloatProcessor::new(3, 3, vec![0.54321; 9], Gray::<f32>::new());
        ip.add(100.0);
        ip.abs();
        assert_eq!(ip.get(3), -100.0);
    }

    #[test]
    fn float_processor_exp() {
        let mut ip = FloatProcessor::new(3, 3, vec![0.54321; 9], Gray::<f32>::new());
        ip.add(3.0);
        ip.exp();
        assert_eq!(ip.get(3), 20.085537);
    }

    #[test]
    fn float_processor_sqrt() {
        let mut ip = FloatProcessor::new(3, 3, vec![0.54321; 9], Gray::<f32>::new());
        ip.add(9.0);
        ip.sqrt();
        assert_eq!(ip.get(3), 3.0);
    }

    #[test]
    fn float_processor_ln() {
        let mut ip = FloatProcessor::new(3, 3, vec![0.54321; 9], Gray::<f32>::new());
        ip.add(29.0);
        ip.ln();
        assert_eq!(ip.get(3), 3.3672957);
    }

    #[test]
    fn float_processor_log() {
        let mut ip = FloatProcessor::new(3, 3, vec![0.54321; 9], Gray::<f32>::new());
        ip.add(29.0);
        ip.log();
        assert_eq!(ip.get(3), 1.4623979);
    }

    //gamma
}
