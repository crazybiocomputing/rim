pub trait ROI {

    /// Returns a Rectangle that represents the current region of interest.
    java.awt.Rectangle get_roi() {}

   /// Sets the ROI (Region of interest) and clipping rectangle to the entire image.
    fn reset_Roi() {}

    fn set_mask​(Image_Processor mask) {}
    /// Defines a byte mask that limits processing to an irregular ROI.

    fn set_roi​(i32 x, i32 y, i32 rwidth, i32 rheight) {}
    /// Defines a rectangular region of i32erest and sets the mask to null if this ROI is not the same size as the previous one.

    fn set_roi​(Roi roi) {}
    /// Defines a non-rectangular region of i32erest that will consist of a rectangular ROI and a mask.

    fn set_roi​(java.awt.Polygon roi) {}
    /// Defines a polygonal region of i32erest that will consist of a rectangular ROI and a mask.

    fn set_roi​(java.awt.Rectangle roi) {}
    /// Defines a rectangular region of i32erest and sets the mask to null if this ROI is not the same size as the previous one.


}

