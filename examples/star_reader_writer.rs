use rim::io::star_reader::StarIO;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    StarIO::load_file_star(&args[1]);
}
