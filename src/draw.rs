pub trait Draw {
    /// Draws the specified ROI on this image using the line width and color defined by ip.set_Line_Width() and ip.set_Color().
    fn draw​(Roi roi) {}

    /// Draws a dot using the current line width and fill/draw value.
    fn draw_Dot​(i32 xcenter, i32 ycenter) {}

    /// Draws a line from (x1,y1) to (x2,y2).
    ffn draw_Line​(i32 x1, i32 y1, i32 x2, i32 y2) {}

    ffn draw_Line4​(i32 x1, i32 y1, i32 x2, i32 y2)  

    /// Draws an elliptical shape.
    fn draw_Oval​(i32 x, i32 y, i32 width, i32 height) {}

    /// Draws the specified Overlay on this image.
    fn draw_Overlay​(Overlay overlay) {}
 
    /// Sets the pixel at (x,y) to the current fill/draw value.
    fn draw_pixel​(i32 x, i32 y) {}

    /// Draws a polygon.
    fn draw_Polygon​(java.awt.Polygon p) {}

    /// Draws a rectangle.
    fn draw_Rect​(i32 x, i32 y, i32 width, i32 height) {}

    /// Draws the specified ROI on this image using the stroke width, stroke color and fill color 
    /// defined by roi.set_Stroke_Width, roi.set_Stroke_Color() and roi.set_Fill_Color().
    fn draw_Roi​(Roi roi) {}

    /// Draws a string at the current drawing location using the current fill/draw value.
    fn draw_String​(java.lang.String s) {}

    /// Draws a string at the specified location using the current fill/draw value.
    fn draw_String​(java.lang.String s, i32 x, i32 y) {}

    /// Draws a string at the specified location with a filled background.
    fn draw_String​(java.lang.String s, i32 x, i32 y, java.awt.Color background) {}

    /// Fills the image or ROI bounding rectangle with the current fill/draw value.
    fn fill() {}

    /// Fills the ROI with the current fill/draw value.
    fn fill​(Roi roi) {}

    /// Fills pixels that are within the ROI bounding rectangle and part of the mask (i.e.
    fn fill​(Image_Processor mask) {}

    /// Fills outside an Roi.
    fn fill_Outside​(Roi roi) {}

    /// Fills an elliptical shape.
    fn fill_Oval​(i32 x, i32 y, i32 width, i32 height) {}

    /// Fills a polygon.
    fn fill_Polygon​(java.awt.Polygon p) {}

    /// Fills a rectangle.
    fn fill_Rect​(i32 x, i32 y, i32 width, i32 height) {}

    /// Returns 'true' if the fill/draw value has been set.
    bool fill_Value_Set() {}
    
    /// Returns the background fill value.
    fn f64 get_background_value() {}
    
    /// Returns the default fill/draw value.
    fn  get_foreground_value() -> f64 {}

    /// Returns an array containing the pixel values along the line starting at (x1,y1) and ending at (x2,y2).
    f64[] get_Line​(f64 x1, f64 y1, f64 x2, f64 y2) {}

    /// Returns the current line width.
    i32 get_Line_Width() {}
    
    /// Draws a line from the current drawing location to (x2,y2).
    fn line_to​(i32 x2, i32 y2) {}

    /// Sets the current drawing location.
    fn moveto​(i32 x, i32 y) {}
    
    /// Specifies whether or not text is drawn using antialiasing.
    fn set_antialiased_text​(bool antialiased_Text) {}

    /// Sets the background fill/draw color.
    fn set_background_color​(java.awt.Color color) {}

    ///Sets the background fill value used by the rotate() and scale() methods.
    fn set_background_value​(f64 value) {}
    
    fn set_clip_rect​( clip_rect: Rectangle) {}
    /// Updates the clipping rectangle used by line_To(), draw_Line(), draw_Dot() and draw_Pixel().

    /// Sets the default fill/draw value.
    fn set_color​(f64 value) {}

    /// Sets the default fill/draw value.
    fn set_color​(i32 value) {}
    
    /// Sets the default fill/draw value to the pixel value closest to the specified color.
    fn set_color​(java.awt.Color color) {}

    /// Sets the color model.
    fn set_color_model​(java.awt.image.Color_Model cm) {}

    fn set_global_background_color() {}
    /// Sets the global (Color Picker) background color as the fill/draw color.

    fn set_global_foreground_color() {}
    /// Sets the global (Color Picker) foreground color as the fill/draw color.

    fn set_font​(java.awt.Font font) {}
    /// Sets the font used by draw_String().

    fn set_font_size​(i32 size) {}
    /// Sets the size of the font used by draw_String().

    fn set_justification​(i32 justification) {}
    /// Sets the justification used by draw_String(), where justification is CENTER_JUSTIFY, RIGHT_JUSTIFY or LEFT_JUSTIFY.

    fn set_line_width​(i32 width) {}
    /// Sets the line width used by line_To() and draw_Dot().


}


