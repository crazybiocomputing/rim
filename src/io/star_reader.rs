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
 
 
 /*
use crate::io::star_parser::parse_star;
use std::fs;

pub struct StarIO {}

impl StarIO {
    ///
    /// Load distant mmCIF file from RCSB web site
    ///
    pub fn load_mmcif(pdb_id: &str) {
        let path = "https://files.rcsb.org/view";
        let url = format!("${}/${}.cif", path, pdb_id.to_uppercase());
        println!("{}", url);

        // Get data from RCSB PDB site
        // TODO

        // Parse text and return
        // parse_star(txt)
    }
    ///
    /// Load local mmCIF file from disk
    ///
    pub fn load_file_mmcif(filename: &str) {
        Self::load_file_star(filename)
    }
    ///
    /// Load local STAR file from disk
    ///
    pub fn load_file_star(filename: &str) {
        // Get data
        let txt = fs::read_to_string(filename).expect("Unable to read file");

        // Parse text and return
        parse_star(txt)
    }
}
*/
