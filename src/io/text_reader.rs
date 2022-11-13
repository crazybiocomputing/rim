//
//  RIM - Rust IMage
//  Copyright (&self,C) 2022  Jean-Christophe Taveau.
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

use crate::results_table::{Cell, Column, ResultsTable};
use std::{
    fs,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

///
///
///
pub struct TextReader {}

impl TextReader {
    /// Open a CSV file
    ///
    /// # Arguments
    ///
    /// * `filename` File name
    /// * `separator` (tab, comma, semi-column, etc.). Default value is the comma `,`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let rt = TextReader::open_csv("./samples/test.csv", None).unwrap();
    /// let col = rt.get_column("String::from("A"));
    /// // Read a given value
    /// let v = col.get_value(1).to_f64(); // expected 4.0
    /// ```
    pub fn open_csv(filename: &str, separator: Option<char>) -> io::Result<ResultsTable> {
        let input = BufReader::new(fs::File::open(filename)?);
        let mut table = ResultsTable::new(filename.to_string());
        // Split in lines and scan
        let mut head = false;
        let mut headings = Vec::<String>::new();
        for (i, line) in input.lines().enumerate() {
            let line = line?;
            let words: Vec<String> = line
                .split(separator.unwrap_or(','))
                .map(|s| s.to_string())
                .collect();
            if !head && words.len() >= 1 {
                headings = words.clone();
                head = true;
            } else {
                // Add a new row
                table.add_row();
                // Convert in float if required
                let mut row = Vec::<Cell>::new();
                for (j, w) in words.iter().enumerate() {
                    let f = w.parse();
                    match f {
                        Ok(x) => table.add_value(&headings[j], Cell::Number(x)),
                        _ => table.add_value(&headings[j], Cell::Text(w.clone())), // Stored as String
                    }
                }
            }
        }
        Ok(table)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::results_table::*;

    #[test]
    fn open_csv_file_and_read_columns_and_rows() {
        let rt = TextReader::open_csv("./samples/test.csv", None).unwrap();
        assert_eq!(rt.size(), 5);
        assert_eq!(rt.get_last_column(), 3);
    }
  
    #[test]
    fn open_csv_file_and_read_value() {
        let rt = TextReader::open_csv("./samples/test.csv", None).unwrap();
        let col = rt.get_column(String::from("A")).unwrap();
        // Read a given value
        let v = col.get_value(1).to_f64(); // expected 4.0
        assert_eq!(v,4.0);
    }
  
    #[test]
    fn open_csv_file_with_default_sep() {
        let col_nums = vec![
            vec![1.0, 4.0, 7.0, 10.0, 13.0],
            vec![2.0, 5.0, 8.0, 11.0, 14.0],
            vec![3.0, 6.0, 9.0, 12.0, 15.0]
        ];
        let col_txt = vec![
            "Chicken".to_string(),
            "Pig".to_string(),
            "Human".to_string(),
            "Mouse".to_string(),
            "Lumbricus".to_string(),
        ];
        /*
        A,B,C,Species
        1,2,3,Chicken
        4,5,6,Pig
        7,8,9,Human
        10,11,12,Mouse
        13,14,15,Lumbricus
        */
        let rt = TextReader::open_csv("./samples/test.csv", None).unwrap();

        for (i,col) in ["A","B","C"].iter().enumerate() {
            assert!(rt
                .get_column(col.to_string()) // Get Result<Column>
                .unwrap() // Get Column
                .to_vec() // Get Vec<Cell>
                .iter()   // Get Iterator
                .zip(&col_nums[i]) // Merge with answer
                .all(|(a, b)| // Check if all elements are equal
                    match *a {
                    Cell::Number(x) => x == *b,
                    _ => false
                    }
                )
            );
        }
    }
}
