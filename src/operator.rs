
pub trait Operator {
    /// Adds 'value' to each pixel in the image or ROI.
    fn add​(&self, value: i32) {};
    
    /// Adds 'value' to each pixel in the image or ROI.
    fn add​_float(&self, value: f64) {};

    /// Binary AND of each pixel in the image or ROI with 'value'.
    fn and​(&self,i32 value) {};

    /// If this is a 32-bit or signed 16-bit image, performs an absolute value transform, otherwise does nothing.
    fn abs(&self) {};

    /// Performs a exponential transform on the image or ROI.
    fn exp(&self) {};

    /// Performs gamma correction of the image or ROI.
    fn gamma​(f64 value) {};

    /// Does a natural logarithmic (base e) transform of the image or ROI.
    fn ln(&self) {};

    /// Does a logarithmic (base 10) transform of the image or ROI.
    fn log(&self) {};
 
    /// Pixels greater than 'value' are set to 'value'.
    fn max​(&self, f64 value) {};

    /// Pixels less than 'value' are set to 'value'.
    fn min​(&self,f64 value) {};

    /// Multiplies each pixel in the image or ROI by 'value'.
    fn multiply​(&self,f64 value) {};

    /// Adds pseudorandom, Gaussian ("normally") distributed values, with mean 0.0 and the specified standard deviation, to this image or ROI.
    fn noise​(f64 standard_Deviation) {};

    /// Binary OR of each pixel in the image or ROI with 'value'.
    fn or​(&self,i32 value) {};

    static fn set_random_seed​(random_seed: f64) {};

    /// Performs a square transform on the image or ROI.
    fn sqr(&self) {};

    /// Performs a square root transform on the image or ROI.
    fn sqrt(&self) {};

    fn subtract​(&self,f64 value) {};
    /// Subtracts 'value' from each pixel in the image or ROI.

    /// Binary exclusive OR of each pixel in the image or ROI with 'value'.
     fn xor​(i32 value) {};

    /// Uses the Process/Math/Macro command to apply functional code to this image/volume.
    /// The function takes eight arguments:
    /// v : current pixel/voxel value.
    /// x,y,z : XY- or XYZ-coordinates of the pixel/voxel
    /// w,h: width and height of the processor
    /// a: angle (polar coordinate)
    /// d: distance from center (polar coordinate)
    fn apply_func​(&self, func: F) {};

    /// Uses the Process/Math/Macro command to apply macro code to this image.
    /// See apply_func(..)
    fn apply_macro​(&self, func: F) {};


}
