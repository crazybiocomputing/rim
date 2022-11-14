//
//  RIM - Rust Image
//  Copyright (C) 2022  Jean-Christophe Taveau.
//
//  This file is part of RIM
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (&self,at your option) any later version.
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

pub struct Roi {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Roi {
    // Accessors
    pub fn x(&self) -> u32 {
        self.x
    }
    pub fn y(&self) -> u32 {
        self.y
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    // Compatibility for ImageJ
    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn get_height(&self) -> u32 {
        self.height
    }
}

///
/// Storage of statistics
/// No computation is done at this level
///
pub struct Statistics {
    is_dirty: bool,
    min: f64,
    max: f64,
    mean: f64,
    std_dev: f64,
    n_buckets: usize,
    histogram: Vec<u32>,
}

impl Statistics {
    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }
    pub fn n_buckets(&self) -> usize {
        self.n_buckets
    }
    pub fn histogram(&self) -> &Vec<u32> {
        &self.histogram
    }
}
///
/// MetaData storing image information
///
///
pub struct MetaData {
    pub roi: Roi,
    pub stats: Statistics,
}

impl MetaData {
    pub fn new(w: u32, h: u32) -> Self {
        let n_buckets: usize = 256;
        MetaData {
            roi: Roi {
                x: 0,
                y: 0,
                width: w,
                height: h,
            },
            stats: Statistics {
                is_dirty: true,
                min: 0.0,
                max: 0.0,
                mean: 0.0,
                std_dev: 0.0,
                n_buckets,
                histogram: vec![0u32; n_buckets],
            },
        }
    }

    pub fn get_min(&self) -> f64 {
        self.stats.min
    }
    pub fn get_max(&self) -> f64 {
        self.stats.max
    }
    pub fn get_mean(&self) -> f64 {
        self.stats.mean
    }
    pub fn get_std_dev(&self) -> f64 {
        self.stats.std_dev
    }
    pub fn get_histogram(&self) -> &Vec<u32> {
        self.stats.histogram()
    }
    pub fn get_histogram_size(&self) -> usize {
        self.stats.n_buckets()
    }
    pub fn get_histogram_min(&self) -> usize {
        let opt = self.stats.histogram().iter().position(|&x| x != 0);
        match opt {
            Some(x) => x,
            None => 0,
        }
    }
    pub fn get_histogram_max(&self) -> usize {
        let opt = self.stats.histogram().iter().rev().position(|&x| x != 0);
        match opt {
            Some(x) => self.stats.n_buckets - 1 - x,
            None => self.stats.n_buckets - 1,
        }
    }
    pub fn set_histogram(&mut self, hist: &Vec<u32>) {
        self.stats.histogram = hist.to_vec();
        self.stats.n_buckets = hist.len();
        self.stats.is_dirty = false;
    }

    pub fn set_stats(&mut self, hist: &Vec<u32>, mi: f64, mx: f64, mean: f64, stddev: f64) {
        self.stats.histogram = hist.to_vec();
        self.stats.n_buckets = hist.len();
        self.stats.is_dirty = false;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn histogram_min() {
        let mut meta = MetaData::new(5, 5);
        let mut hist = vec![0u32; 256];
        hist[2] = 10;
        hist[4] = 15;
        meta.set_histogram(&hist);
        assert_eq!(meta.get_histogram_min(), 2);
    }

    #[test]
    fn histogram_min_equal_to_first_bin() {
        let mut meta = MetaData::new(5, 5);
        let mut hist = vec![0u32; 256];
        hist[0] = 10;
        hist[4] = 15;
        meta.set_histogram(&hist);
        assert_eq!(meta.get_histogram_min(), 0);
    }

    #[test]
    fn histogram_max() {
        let mut meta = MetaData::new(5, 5);
        let mut hist = vec![0u32; 256];
        hist[2] = 10;
        hist[4] = 15;
        meta.set_histogram(&hist);
        assert_eq!(meta.get_histogram_max(), 4);
    }

    #[test]
    fn histogram_max_equal_last_bin() {
        let mut meta = MetaData::new(5, 5);
        let mut hist = vec![0u32; 256];
        hist[0] = 10;
        hist[255] = 15;
        meta.set_histogram(&hist);
        assert_eq!(meta.get_histogram_max(), 255);
    }
}
