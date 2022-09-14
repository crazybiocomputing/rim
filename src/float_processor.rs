

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
    protected java.awt.Image img  
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

