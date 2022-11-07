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
 
 
 
/** This class consists of public fields that describe an image file. */
pub struct FileInfo {
    /* File format (TIFF, GIF_OR_JPG, BMP, etc.). Used by the File/Revert command */
    pub file_format: u32,

    // File type (GRAY8, GRAY_16_UNSIGNED, RGB, etc.)
    pub file_type: u32,
    pub file_name: String,
    pub directory: String,
    pub url: String,
    pub width: u32,
    pub height: u32,
    pub offset: u32, // =0 by default Use getOffset() to read
    pub n_images: u32,
    pub gap_between_images: u32, // Use getGap() to read
    pub white_is_zero: bool,
    pub intel_byte_order: bool,
    pub compression: u32,
    //    pub stripOffsets: Vec<u32>,
    //    pub stripLengths: Vec<u32>,
    //    pub  rowsPerStrip: u32,
    //    pub  lutSize: u32,
    //    pub reds: Vec<u8>,
    //    pub greens: Vec<u8>,
    //    pub blues: Vec<u8>,
    //    pub pixels,
    //    pub  debugInfo: String,
    //    pub sliceLabels: Vec<String>,
    pub info: String,
    //    pub InputStream inputStream,
    //    pub VirtualStack virtualStack,
    //    pub sliceNumber: u32, // used by FileInfoVirtualStack
    pub pixel_width: f64,  // =1.0,
    pub pixel_height: f64, // =1.0,
    pub pixel_depth: f64,  // =1.0,
    pub unit: String,
    //    pub calibrationFunction: u32,
    //    pub coefficients: Vec<f64>,
    //    pub  valueUnit: String,
    //    pub  frameInterval: f64,
    pub description: String,
    // Use <i>longOffset</i> instead of <i>offset</i> when offset>2147483647.
    //    pub longOffset: u64,  // Use getOffset() to read
    // Use <i>longGap</i> instead of <i>gapBetweenImages</i> when gap>2147483647.
    //    pub longGap: u64,  // Use getGap() to read
    // Extra metadata to be stored in the TIFF header
    //    pub metaDataTypes: Vec<u32>, // must be < 0xffffff
    //    pub byte[][] metaData,
    //    pub double[] displayRanges,
    //    pub byte[][] channelLuts,
    //    pub byte[] plot,         // serialized plot
    //    pub byte[] roi,          // serialized roi
    //    pub byte[][] overlay,    // serialized overlay objects
    pub samples_per_pixel: u32,
    //    pub  openNextDir: String,
    //    pub openNextName: String,
    //    pub properties: Vec<String>, // {key,value,key,value,...}
    pub image_saved: bool,
}

impl FileInfo {
    /** 8-bit unsigned integer (0-255). */
    pub const GRAY8: u32 = 0;

    /** 16-bit signed integer (-32768-32767). Imported signed images
    are converted to unsigned by adding 32768. */
    pub const GRAY16_SIGNED: u32 = 1;

    /** 16-bit unsigned eger (0-65535). */
    pub const GRAY16_UNSIGNED: u32 = 2;

    /** 32-bit signed integer. Imported 32-bit integer images are
    converted to floating-po. */
    pub const GRAY32_INT: u32 = 3;

    /** 32-bit floating-po. */
    pub const GRAY32_FLOAT: u32 = 4;

    /** 8-bit unsigned eger with color lookup table. */
    pub const COLOR8: u32 = 5;

    /** 24-bit interleaved RGB. Import/export only. */
    pub const RGB: u32 = 6;

    /** 24-bit planar RGB. Import only. */
    pub const RGB_PLANAR: u32 = 7;

    /** 1-bit black and white. Import only. */
    pub const BITMAP: u32 = 8;

    /** 32-bit interleaved ARGB. Import only. */
    pub const ARGB: u32 = 9;

    /** 24-bit interleaved BGR. Import only. */
    pub const BGR: u32 = 10;

    /** 32-bit unsigned integer. Imported 32-bit integer images are
    converted to floating-po. */
    pub const GRAY32_UNSIGNED: u32 = 11;

    /** 48-bit interleaved RGB. */
    pub const RGB48: u32 = 12;

    /** 12-bit unsigned integer (0-4095). Import only. */
    pub const GRAY12_UNSIGNED: u32 = 13;

    /** 24-bit unsigned integer. Import only. */
    pub const GRAY24_UNSIGNED: u32 = 14;

    /** 32-bit interleaved BARG (MCID). Import only. */
    pub const BARG: u32 = 15;

    /** 64-bit floating-po. Import only.*/
    pub const GRAY64_FLOAT: u32 = 16;

    /** 48-bit planar RGB. Import only. */
    pub const RGB48_PLANAR: u32 = 17;

    /** 32-bit interleaved ABGR. Import only. */
    pub const ABGR: u32 = 18;

    /** 32-bit interleaved CMYK. Import only. */
    pub const CMYK: u32 = 19;

    // File formats
    pub const UNKNOWN: u32 = 0;
    pub const RAW: u32 = 1;
    pub const TIFF: u32 = 2;
    pub const GIF_OR_JPG: u32 = 3;
    pub const FITS: u32 = 4;
    pub const BMP: u32 = 5;
    pub const DICOM: u32 = 6;
    pub const ZIP_ARCHIVE: u32 = 7;
    pub const PGM: u32 = 8;
    pub const IMAGEIO: u32 = 9;

    // Compression modes
    pub const COMPRESSION_UNKNOWN: u32 = 0;
    pub const COMPRESSION_NONE: u32 = 1;
    pub const LZW: u32 = 2;
    pub const LZW_WITH_DIFFERENCING: u32 = 3;
    pub const JPEG: u32 = 4;
    pub const PACK_BITS: u32 = 5;
    pub const ZIP: u32 = 6;
}

impl FileInfo {
    /** Creates a FileInfo object with all of its fields set to their default value. */
    pub fn new() -> FileInfo {
        // assign default values
        FileInfo {
            file_format: FileInfo::UNKNOWN,
            file_type: FileInfo::GRAY8,
            file_name: String::from("Untitled"),
            directory: "".to_string(),
            url: "".to_string(),
            width: 0,
            height: 0,
            offset: 0, // =0 by default Use getOffset() to read
            n_images: 1,
            gap_between_images: 0,
            white_is_zero: false,
            compression: FileInfo::COMPRESSION_NONE,
            intel_byte_order: true,
            info: String::from("No info"),
            pixel_width: 1.0,
            pixel_height: 1.0,
            pixel_depth: 1.0,
            unit: String::from("px"),
            description: String::from("None"),
            samples_per_pixel: 1,
            image_saved: false,
        }
    }

    /** Returns the file path. */
    pub fn get_file_path(&self) -> String {
        let dir: String = if self.directory.is_empty() {
            String::from("")
        } else {
            self.directory.clone()
        };
        // dir = R.addSeparator(dir);
        dir + &self.file_name
    }

    /** Returns the offset as a long. */
    pub fn get_offset(&self) -> u64 {
        //        return longOffset>0L ? longOffset:((long)offset)&0xffffffffL;
        1234
    }

    /** Returns the gap between images as a long. */
    pub fn get_gap(&self) -> u64 {
        //        return longGap > 0L ? longGap:((long)gapBetweenImages)&0xffffffffL;
        1234
    }

    /** Returns the number of bytes used per pixel. */
    pub fn get_bytes_per_pixel(&self) -> u64 {
        match self.file_type {
            FileInfo::GRAY8 | FileInfo::COLOR8 | FileInfo::BITMAP => 1,
            FileInfo::GRAY16_SIGNED | FileInfo::GRAY16_UNSIGNED | FileInfo::GRAY12_UNSIGNED => 2,
            FileInfo::GRAY32_INT
            | FileInfo::GRAY32_UNSIGNED
            | FileInfo::GRAY32_FLOAT
            | FileInfo::ARGB
            | FileInfo::GRAY24_UNSIGNED
            | FileInfo::BARG
            | FileInfo::ABGR
            | FileInfo::CMYK => 4,
            FileInfo::RGB | FileInfo::RGB_PLANAR | FileInfo::BGR => 3,
            FileInfo::RGB48 | FileInfo::RGB48_PLANAR => 6,
            FileInfo::GRAY64_FLOAT => 8,
            _ => 0,
        }
    }

    fn get_type(&self) -> String {
        match self.file_type {
            FileInfo::GRAY8 => "byte".to_string(),
            FileInfo::GRAY16_SIGNED => "short".to_string(),
            FileInfo::GRAY16_UNSIGNED => "ushort".to_string(),
            FileInfo::GRAY32_INT => "int".to_string(),
            FileInfo::GRAY32_UNSIGNED => "uint".to_string(),
            FileInfo::GRAY32_FLOAT => "float".to_string(),
            FileInfo::COLOR8 => "byte(lut)".to_string(),
            FileInfo::RGB => "RGB".to_string(),
            FileInfo::RGB_PLANAR => "RGB(p)".to_string(),
            FileInfo::RGB48 => "RGB48".to_string(),
            FileInfo::BITMAP => "bitmap".to_string(),
            FileInfo::ARGB => "ARGB".to_string(),
            FileInfo::ABGR => "ABGR".to_string(),
            FileInfo::BGR => "BGR".to_string(),
            FileInfo::BARG => "BARG".to_string(),
            FileInfo::CMYK => "CMYK".to_string(),
            FileInfo::GRAY64_FLOAT => "double".to_string(),
            FileInfo::RGB48_PLANAR => "RGB48(p)".to_string(),
            _ => "".to_string(),
        }
    }
} // End of impl

impl ToString for FileInfo {
    fn to_string(&self) -> String {
        format!("name={}, dir={}, width={}, height={}, nImages={}, offset={}, gap={}, type={}, byteOrder={}, format={}, url={}, whiteIsZero={}, comp={}, samples={}", 
      &self.file_name,
      &self.directory,
      &self.width,
      &self.height,
      &self.n_images,
      &self.get_offset(),
      &self.get_gap(),
      &self.get_type(),
      if self.intel_byte_order {"little"} else {"big"},
      &self.file_format,
      &self.url,
      if self.white_is_zero {"t"} else {"f"},
//            + ", lutSize=" + self.lutSize
      &self.compression,
//            + ", ranges=" + if displayRanges!=null {""+displayRanges.length/2} else {"null"}
      &self.samples_per_pixel
    )
    }
}
