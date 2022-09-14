use crate::io::star_parser::parse_star;
use std::fs;

pub struct StarIO {}

impl StarIO {
    /**
     * Load distant file from RCSB web site
     */
    pub fn load_mmcif(pdb_id: &str) {
        let path = "https://files.rcsb.org/view";
        let url = format!("${}/${}.cif", path, pdb_id.to_uppercase());
        println!("{}", url);

        // Get data from RCSB PDB site
        // TODO

        // Parse text and return
        // parse_star(txt)
    }

    pub fn load_file_mmcif(filename: &str) {
        Self::load_file_star(filename)
    }

    pub fn load_file_star(filename: &str) {
        // Get data
        let txt = fs::read_to_string(filename).expect("Unable to read file");

        // Parse text and return
        parse_star(txt)
    }
}
