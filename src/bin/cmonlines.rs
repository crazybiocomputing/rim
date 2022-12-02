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
fn rank(values: &Vec<f32>) ->Vec<f32>{
    /// https://imagej.nih.gov/ij/developer/source/ij/util/Tools.java.html
    ///
    /// Calculates peak positions of 1D array N.Vischer, 06-mar-2017
    ///
    /// # Parameters
    ///
    /// `xx` : A Vec<f32> containing peaks.
    /// `tolerance` : Depth of a qualified valley must exceed tolerance.
    ///                Tolerance must be >= 0. Flat tops are marked at their centers.
    /// `edgemode` : 0=include, 1=exclude
    ///              edgeMode = 0 (include edges) peak may be separated by one qualified valley and by a border.
    ///              edgeMode = 1 (exclude edges) peak must be separated by two qualified valleys
    ///
    /// # Returns
    ///
    /// Positions of peaks, sorted with decreasing amplitude, as Vect<f32>
    let len= values.len();
    let mut indexes= Vec::<f32>::new();
    let mut data = values.clone();
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for value in data{
        for i in 0..len{
            if value == values[i]{
                indexes.push(i as f32);
            }
        }
    }
    indexes
}
fn find_maxima(xx: &Vec<f32>, mut tolerance: f32, edgemode: f32) ->Vec<f32>{
    /// https://imagej.nih.gov/ij/developer/source/ij/plugin/filter/MaximumFinder.java.html
    /// Ranks a Vec<f32> from the index of the smallest value to index of tha biggest value
    ///
    /// # Parameters
    ///
    /// `values` : A Vec<f32> that will be ranked
    ///
    /// # Returns
    ///
    ///
    let INCLUDE_EDGE: f32 = 0 as f32;
    let len: usize = xx.len() as usize;
    if len < 2 {
        return Vec::<f32>::with_capacity(0);
    }
    if tolerance<0 as f32{
        tolerance = 0 as f32;
    }
    let mut max_positions:Vec<f32> = vec![0.0; len];
    let mut max: f32 = xx[0];
    let mut min: f32 = xx[0];
    let mut max_pos: f32 = 0 as f32;
    let mut last_max_pos: f32 = -1 as f32;
    let mut left_valley_found: bool = edgemode as f32 == INCLUDE_EDGE as f32;
    let mut max_count: f32 = 0 as f32;
    for i in 0..len{
        let val: f32 = xx[i];
        if val > min + tolerance{
            left_valley_found = true;
        }
        if (val > max) & (left_valley_found){
            max = val;
            max_pos = i as f32;
        }
        if left_valley_found{
            last_max_pos = max_pos;
        }
        if (val < max - tolerance) & (left_valley_found){
            max_positions[max_count as usize]= max_pos;
            max_count+=1 as f32;
            left_valley_found = false;
            min = val;
            max = val;
        }
        if val < min {
            min = val;
            if !left_valley_found{
                max = val;
            }
        }

    }
    if edgemode == INCLUDE_EDGE{
        if (max_count> 0 as f32) & (max_positions[max_count as usize-1] != last_max_pos){
            max_positions[max_count as usize+1] = last_max_pos;
        }
        if (max_count == 0 as f32) & (max -min >= tolerance){
            max_positions[max_count as usize+1] = last_max_pos;
        }
    }
    let mut cropped:Vec<f32> = vec![0.0; max_count as usize];
    cropped.copy_from_slice(&max_positions[0..max_count as usize]);
    max_positions = cropped;
    let mut max_values:Vec<f32> = vec![0.0; max_count as usize];
    for i in 0..max_count as usize{
        let mut pos : f32 = max_positions[i];
        let mut mid_pos : f32 = pos;
        while (pos < (len - 1) as f32) & (xx[pos as usize] == xx[pos as usize +1]) {
            mid_pos += 0.5;
            pos+=1 as f32;
        }
        max_positions[i] = mid_pos;
        max_values[i] = xx[max_positions[i] as usize];
    }
    let rank_positions = rank(&max_values);
    let mut returnArr:Vec<f32> = vec![0.0; max_count as usize];
    for i in 0..(max_count as usize){
        let pos: f32 = max_positions[rank_positions[i] as usize];
        returnArr[max_count as usize- i -1] = pos; //use descending order
    }
    returnArr
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    let sin1 = get_sinogram("./samples/7.raw");
    let sin2 = get_sinogram("./samples/50.raw");

    let corr_test = corr_sinogram(&sin1, &sin2);
    /*for i in 0..128 {
        println!("index {}, valeur {}", i, corr_test[i])
    }*/

    let vectest:Vec<f32> = vec![1.0, 5.0, 10.0, 2.0, 15.0];
    let vecrank = rank(&vectest);
    for i in 0..vecrank.len(){
        println!("rank: {}, valeur vec: {}", vecrank[i], vectest[vecrank[i] as usize])
    }

    // Ici j'ai créé un vecteur avec 3 pics entourés de petites valeurs
    // Si tu enleves une petite valeur entre 2 grandes ça comptera plus comme 2 pics mais 1 seul
    // Enfin edgemode 1 quoi
    let xxtest:Vec<f32> = vec![1.0,500.09,2.0,50.5,3.0,8.0,100.0,0.05];
    let a :Vec<f32> = find_maxima(&xxtest, 0 as f32, 1 as f32);
    for i in 0..a.len(){
        println!("maximum index: {}, valeur vec: {}", a[i], xxtest[a[i] as usize])
    }

}
