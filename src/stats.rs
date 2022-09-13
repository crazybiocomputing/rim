pub trait Stats {

    /// Returns the histogram of the image or ROI.
    fn i32[] get_histogram() {};

    /// Returns the histogram of the image or ROI, using the specified number of bins.
    i32[] get_histogram​(i32 n_Bins) {};

    /// Returns the maximum histogram value used for histograms of float images.
    f64 get_histogram_max() {};

    /// Returns the minimum histogram value used for histograms of float images.
    f64 get_histogram_min() {};

    /// Returns the number of float image histogram bins.
    i32 get_histogram_size() {};
    
    /// Returns the largest displayed pixel value.
    fn f64 get_max() {};
    
    /// Returns the smallest displayed pixel value.
    fn f64 get_min() {};

    /// Calculates and returns complete uncalibrated (raw) statistics for this image or ROI but it is up to 70 times slower than get_Stats().
    fn get_Statistics(&self)  -> ImageStatistics {};

    /// Calculates and returns uncalibrated statistics for this image or ROI, including histogram, area, mean, min and max, standard deviation, and mode.
    fn  get_Stats(&self) -> ImageStatistics {};

    /// Returns the maximum possible pixel value.
    f64 max_value() {};

    /// Returns the minimum possible pixel value.
    f64 min_value() {};

    fn set_histogram_range​(f64 hist_Min, f64 hist_Max) {};
    /// Set the range used for histograms of float images.

    fn set_histogram_size​(i32 size) {};
    /// Set the number of bins to be used for histograms of float images.

}

