use std::env;
use num_traits::{Pow, pow};
use rim::{float_processor::FloatProcessor, grayscale::Gray32, image_processor::ImageProcessor, color_space::ColorSpace};
use rim::image_traits::Access;

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


fn get_col(ip: &FloatProcessor, index: u32) -> Vec<f32> {
    ///Get a specific column of o=pixel of a FloatProcessor
    ///
    /// # Parameters
    /// `ip` : A FloatProcessor, in our case a sinogram
    /// `index` : Index of the column to extract
    ///
    /// # Returns
    ///
    /// A Vec<f32> containing the pixel value frmo the extracted column
    ///
    let mut row: Vec<f32> = Vec::<f32>::new();
    for y in 0..ip.get_height() {
        let pixel = (ip.get_pixel_at(index as u32, y as u32)).unwrap() as f32;
        row.push(pixel);
    }
    row
}

fn distance(colonne: &Vec<f32>, ip: &FloatProcessor) -> Vec<f32> {
    ///Generate a Vec<f32> containing the distance between a vector and all vector of a FloatProcessor
    ///
    ///# Parameters
    ///
    /// `colonne` : A Vec<32>, in our case it contains the pixel value of a column of a Singogram
    /// `ip` : A FloatProcessor, in our case a sinogram
    ///
    /// # Returns
    ///
    /// A Vec<f32> containing the distance between the vector and each column of the FloatProcessor
    ///
    let mut dist = Vec::<f32>::new();
    for x in 0..ip.get_width() {
        let curr_col = get_col(ip, x);
        dist.push(euc_dist(colonne, &curr_col));
    }
    dist
}

fn sinogram_dist_function(ip1: &FloatProcessor, ip2: &FloatProcessor) -> Vec<f32> {
    ///Generate the Sinogram Distance Function for 2 FloatProcessor, for each line of the first Sinogram
    /// the distance is computed versus all the lines of the second Sinogram.
    ///
    /// # Parameters
    ///
    /// `ip1` : A FloatProcessor, in our case a sinogram
    /// `ip2` : A FloatProcessor, in our case a sinogram
    ///
    /// # Returns
    ///
    /// A vec<f32> containing all the distances computed

    let mut sdf = Vec::<f32>::new();
    for x in 0..ip1.get_width() {
        let col = get_col(&ip1, x);
        let mut dist = distance(&col, &ip2);
        sdf.append(&mut dist);
    }
    sdf
}

fn get_col_test(oui: &Vec<f32>, col: u32) -> Vec<f32> {
    let mut row: Vec<f32> = Vec::<f32>::new();
    for y in 0..4 {
        let index = (col + 4 * y) as usize;
        let pixel = oui[index];
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

// fn data_normalisation(ip : &FloatProcessor) -> FloatProcessor{
//     //TODO
// }


fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    let vect1 = vec![10 as f32, 15 as f32, 25 as f32, 30 as f32,
                     20 as f32, 50 as f32, 20 as f32, 12 as f32,
                     25 as f32, 15 as f32, 60 as f32, 28 as f32,
                     22 as f32, 18 as f32, 50 as f32, 14 as f32];
    let vect = vec![5 as f32, 10 as f32, 15 as f32, 8 as f32,
                    25 as f32, 20 as f32, 8 as f32, 14 as f32,
                    40 as f32, 12 as f32, 8 as f32, 10 as f32,
                    30 as f32, 15 as f32, 10 as f32, 14 as f32];
    let test_euc = vec![5 as f32, 10 as f32, 15 as f32, 20 as f32];
    let test_euc_2 = vec![10 as f32, 15 as f32, 20 as f32, 25 as f32];
    let moy = mean(&vect);
    println!("La moyenne : {}", moy);
    let stand = sd(&vect);
    println!("L'écart type : {}", stand);
    let euc = euc_dist(&test_euc, &test_euc_2);
    println!("La distance : {}", euc);
    let test_again = get_col_test(&vect1, 2);
    for i in 0..test_again.len() {
        println!("{}", test_again[i]);
    }
}
