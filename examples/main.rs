use rim::io::file_info::FileInfo;

use rim::test;

pub fn main() {
    test();
    let _fi = FileInfo::new();
    println!(
        "I'm using the library: {:?}",
        rim::really_complicated_code(1, 2)
    );
}
