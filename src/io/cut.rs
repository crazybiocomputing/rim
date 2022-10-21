
use std::fs::File;
use std::io::Read;
use std::fs::metadata;
use std::env;
use std::num;
mod classe;
use classe::*;



fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}


fn cut_by_slices(file:&ImageReader) ->Vec<Vec<u8>>
{
    //u32::from(file.len());
    //&file.len().to_u32();
    
    let mut nb_pix_per_slice = file.len()/ file.slice();
    //nb_pix_per_slice as u32;
    let mut newbuffer:Vec<Vec<u8>> = vec![];
    for i in (0..file.len()).step_by(nb_pix_per_slice.try_into().unwrap())
    {
        let mut temp:Vec<u8> = vec![];
        for j in i..(nb_pix_per_slice+i)
        {

            let mut u = (j) as usize;
            temp.push(file.buffer()[u]);
        }
        newbuffer.push(temp);
    }
    //file.modifyBufferType(newbuffer);
    return newbuffer;
}

fn cut_by_pixels_16(file:&ImageReader, image:Vec<Vec<u8>>) -> Vec<Vec<(u8,u8)>>
{
    let mut nb_pix_per_slice = file.len()/ file.slice();
    let mut buffer:Vec<Vec<(u8,u8)>> = vec![];
    for i in 0..file.slice()
    {
        let mut temp:Vec<(u8, u8)> = vec![];
        for j in (0..nb_pix_per_slice).step_by(2)
        {
            let iu = i as usize;
            let ju = j as usize;
            let mut tup = (image[iu][ju], image[iu][ju+1]);
            temp.push(tup);
        }
        buffer.push(temp);
    }
    return buffer;
}

fn cut_by_pixels_32(file:&ImageReader, image:Vec<Vec<u8>>) -> Vec<Vec<(u8,u8, u8, u8)>>
{
    let mut nb_pix_per_slice = file.len()/ file.slice();
    let mut buffer:Vec<Vec<(u8,u8, u8, u8)>> = vec![];
    for i in 0..file.slice()
    {
        let mut temp:Vec<(u8, u8, u8, u8)> = vec![];
        for j in (0..nb_pix_per_slice).step_by(4)
        {
            let iu = i as usize;
            let ju = j as usize;
            let mut tup = (image[iu][ju], image[iu][ju+1], image[iu][ju+2], image[iu][ju+3]);
            temp.push(tup);
        }
        buffer.push(temp);
    }
    return buffer;
}

fn cut_by_pixels_rgb(file:&ImageReader, image:Vec<Vec<u8>>) -> Vec<Vec<(u8,u8, u8)>>
{
    let mut nb_pix_per_slice = file.len()/ file.slice();
    let mut buffer:Vec<Vec<(u8,u8, u8)>> = vec![];
    for i in 0..file.slice()
    {
        let mut temp:Vec<(u8, u8, u8)> = vec![];
        for j in (0..nb_pix_per_slice).step_by(3)
        {
            let iu = i as usize;
            let ju = j as usize;
            let mut tup = (image[iu][ju], image[iu][ju+1], image[iu][ju+2]);
            temp.push(tup);
        }
        buffer.push(temp);
    }
    return buffer;
}


fn sorting_images (file:&ImageReader) -> Answer
{
    let mut image:Vec<Vec<u8>> = vec![file.buffer().clone()];
    if file.slice() > 1 
    {
        image = cut_by_slices(&file);
        println!("{:?}", image);
    }

    if file.fi() == "16bit"
    {
        let cutimage16 = cut_by_pixels_16(&file, image);
        return cutimage16;
    }

    else if file.fi() == "32bit"
    {
        let cutimage32 = cut_by_pixels_32(&file, image);
        return cutimage32;
    }

    else if file.fi() == "rgb"
    {
        let cutimagergb = cut_by_pixels_rgb(&file, image);
        return cutimagergb;
    }

    return image;

}

enum Answer
{
    image(Vec<Vec<u8>>), 
    cutimage16(Vec<Vec<(u8, u8)>>), 
    cutimage32(Vec<Vec<(u8, u8, u8)>>), 
    cutimagergb(Vec<Vec<(u8, u8, u8, u8)>>),
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let filename = &args[1].to_string();
    let mut test = get_file_as_byte_vec(filename);
    println!("{:?}", test);
    let cut = ImageReader::new (2, 2, 2, &"rgb".to_string(), test);
    let mut testcut = sorting_images(&cut);
    println!("{:?}", testcut);
}
