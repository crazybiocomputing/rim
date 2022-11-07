
pub trait Filter {
    /// Blurs the image by convolving with a Gaussian function.
    fn blur_gaussian​(f64 sigma) {}
    
    /// Performs a convolution operation using the specified kernel.
    fn convolve​(float[] kernel, i32 kernel_width, i32 kernel_height) {}

    /// Convolves the image or ROI with the specified 3x3 i32eger convolution kernel.
    fn convolve3x3​(i32[] kernel) {}

    /// Dilates the image or ROI using a 3x3 minimum filter. 
    fn dilate() {}
    
    /// Erodes the image or ROI using a 3x3 maximum filter.
    fn erode() {}
    
    /// A 3x3 filter operation, where the argument (BLUR_MORE, FIND_EDGES, MEDIAN_FILTER, MIN or MAX) determines the filter type.
    fn filter​(i32 type) {}

    /// Finds edges in the image or ROI using a Sobel operator.
    fn find_edges() {}

    fn get_neighborhood​(i32 x, i32 y, f64[][] arr) {};

    /// Inverts the image or ROI.
    fn invert() {}

    /// Inverts the values in this image's LUT (indexed color model).
    fn invert_Lut() {}

    /// A 3x3 median filter.
    fn median_Filter() {}
 
    fn sharpen() {}
    /// Sharpens the image or ROI using a 3x3 convolution kernel.

    /// Replaces each pixel with the 3x3 neighborhood mean.
    fn smooth() {}
}


