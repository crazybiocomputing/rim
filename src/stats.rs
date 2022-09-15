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

