use std::env;
use std::fs::read;
use num_traits::{Float, Pow, pow};
use rim::{float_processor::FloatProcessor, grayscale::Gray32, image_processor::ImageProcessor, color_space::ColorSpace};
use rim::cryoem::sinogram::Sinogram;
use rim::grayscale::Gray;
use rim::image_traits::Access;
use rim::io::file_info::FileInfo;
use rim::io::image_reader::{FileOpener, OutputProcessor};


// TEST pour manipuler RUST
fn mean(data: &Vec<f32>) -> f32 {
    let mut sum: f32 = 0 as f32;
    for num in data {
        sum += num;
    }
    ((sum as f32) / data.len() as f32) as f32
}

fn var(data: &Vec<f32>) -> f32 {
    let mut sum_var: f32 = 0 as f32;
    let moy = mean(data);
    for num in data {
        sum_var += f32::pow(num - moy, 2);
    }
    (sum_var / data.len() as f32) as f32
}

fn sd(data: &Vec<f32>) -> f32 {
    let variance = var(data);
    variance.sqrt()
}

fn norm(vector : &Vec<f32>) -> Vec<f32> {
    let mut norm_vec = Vec::<f32>::new();
    let m = mean(vector);
    let std = sd(vector);
    for value in vector {
        let norm_value = (value - m) / std;
        norm_vec.push(norm_value)
    }
    norm_vec
}

fn olny_sd_norm(vector : &Vec<f32>) -> Vec<f32> {
    let mut norm_vec = Vec::<f32>::new();
    let std = sd(vector);
    for value in vector {
        let norm_value = value / std;
        norm_vec.push(norm_value);
    }
    norm_vec
}

fn cross_correlation(x : &Vec<f32>, y : &Vec<f32>) -> Vec<f32> {
    let n = x.len();
    let mean_x = mean(x);
    let mean_y = mean(y);
    let mut sum_x = 0 as f32;
    let mut sum_y = 0 as f32;
    let mut result = Vec::<f32>::new();
    for i in 0..x.len() {
        sum_x += f32::pow(x[i] - mean_x, 2);
        sum_y += f32::pow(y[i] - mean_y, 2);
    }
    let denom = (sum_x * sum_y).sqrt();
    for delay in 0..n {
        let mut num = 0 as f32;
        for i in 0..n {
            let shift_neg = (i + delay);
            let shift_pos = (i as i32 - delay as i32);
            println!("{}",shift_pos);
            if shift_neg < n{
                num += (x[i] - mean_x) * (y[shift_neg] - mean_y);
            }
            if shift_pos > 0 as i32 {
                num += (x[i] - mean_x) * (y[shift_pos as usize] - mean_y);
            }
        }
        result.push(num / denom);
    }
    result
}

fn corr_1_line(colonne : &Vec<f32>, ip: &FloatProcessor) -> Vec<f32> {
    ///Generate a Vec<f32> containing the cross-correlation between a vector and all vector of a FloatProcessor.
    ///
    ///# Parameters
    ///
    /// `colonne` : A Vec<32>, in our case it contains the pixel value of a column of a Singogram
    /// `ip` : A FloatProcessor, in our case a sinogram
    ///
    /// # Returns
    ///
    /// A Vec<f32> containing the cross-correlation between the vector and each column of the FloatProcessor.
    ///
    let mut corr = Vec::<f32>::new();
        for x in 0..ip.get_width() {
            let curr_col = get_col(ip, x);
            corr.push(euc_dist(colonne, &curr_col));
        }

    corr
}

fn get_col(ip: &FloatProcessor, index: u32) -> Vec<f32> {
    ///Get a specific column of pixels of a FloatProcessor
    ///
    /// # Parameters
    /// `ip` : A FloatProcessor, in our case a sinogram
    /// `index` : Index of the column to extract
    ///
    /// # Returns
    ///
    /// A Vec<f32> containing the pixel value from the extracted column
    ///
    let mut row : Vec<f32> = Vec::<f32>::new();
    for y in 0..ip.get_height() {
        let pixel = (ip.get_pixel_at(index as u32, y as u32)).unwrap() as f32;
        row.push(pixel);
    }
    row
}

fn euc_dist(vect1: &Vec<f32>, vect2: &Vec<f32>) -> f32 {
    /// Compute the euclidean dsitance between two vectors
    ///
    /// # Parameters
    ///
    /// `vect1` : A Vec<f32>, in our case it contains the pixel value of a column of a Singogram
    /// `vect2` : A Vec<f32>, in our case it contains the pixel value of a column of a Singogram
    ///
    /// # Returns
    ///
    /// The computed distance as f32.
    let mut sum: f32 = 0 as f32;
    for i in 0..vect1.len() {
        sum += f32::pow(vect1[i] - vect2[i], 2);
    }
    sum.sqrt()
}

fn corr_sinogram(ip1 : &FloatProcessor, ip2 : &FloatProcessor) -> Vec<f32> {
    let mut scf = Vec::<f32>::new();
        for x in 0..ip1.get_width() {
            let col = get_col(&ip1, x);
            let mut dist = corr_1_line(&col, &ip2);
            scf.append(&mut dist);
        }
    scf
}


fn percent_diff(num1 : f32, num2 : f32) -> f32 {
    if num1 > num2 {
        num1 / num2 * (100 as f32)
    }
    else {
        num2 / num1 * (100 as f32)
    }
}


fn get_sinogram(filename : &str) -> FloatProcessor{
    let mut vect = Vec::<f32>::new();
    let mut height = 0 as u32;
    let mut width = 0 as u32;
    let proc = FileOpener::open_processor(filename, 128, 128, FileInfo::GRAY32_FLOAT);
    if let OutputProcessor::FloatProcessor(proc) = proc {
        vect = proc.data().clone();
        height = proc.get_height();
        width = proc.get_width();
    }
    let start = 0 as f32;
    let end = 180 as f32;
    let step = 1 as f32;
    let fp = ImageProcessor::new(width, height, vect, Gray32::new());
    let op = Sinogram::new_in_range(&fp, start , end, step);
    println!(vect.len());
    op

}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    let sin1 = get_sinogram("./samples/7.raw");
    let sin2 = get_sinogram("./samples/50.raw");

    let corr_test = corr_sinogram(&sin1, &sin2);
    for i in 0..128 {
        println!("index {}, valeur {}",i, corr_test[i])
    }


}
