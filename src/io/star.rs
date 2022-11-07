//
//  RIM - Rust Image
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
//
// Authors:

#![allow(non_camel_case_types)]
#![allow(unused)]

use std::fmt;

///
/// This class allows the storage of containers (Category or Table).
///
/// A STAR data structure is composed of one root `DataSet` comprising children as Category or Table
///
#[derive(Debug)]
pub struct DataSet {
    key: String,
    objects: Vec<Container>, // Category and/or Table
}

impl DataSet {
    /// Constructor
    ///
    /// Creates an empty `DataSet` with a given id
    ///
    /// # Arguments
    ///
    /// * name - String corresponding to the `id`
    ///
    pub fn new(id: String) -> Self {
        DataSet {
            key: String::from(id),
            objects: Vec::<Container>::new(),
        }
    }

    ///
    /// Add a child item (`Category` or `Table`) at the end of **this** `DataSet`
    ///
    /// # Arguments
    ///
    /// * item - `Container` corresponding to an object of type `Category` or `Table`
    ///
    pub fn push(&mut self, item: Container) {
        self.objects.push(item);
    }

    ///
    /// Get an  item (`Category` or `Table`) at the given index
    ///
    /// # Arguments
    ///
    /// * index - `usize` a value between 0 upto items numbers
    ///
    pub fn item_at(&self, index: usize) -> &Container {
        &self.objects[index]
    }

    ///
    /// Get the first table (`Category` or `Table`) from its name
    ///
    /// # Arguments
    ///
    /// * name - `String`
    ///
    pub fn item(&self, name: String) -> &Container {
        &self.objects.iter().find(|&x| x.name() == name).unwrap()
    }
}

///
/// This class is a wrapper for `Category` and `Table` objects
///
/// A Container is the child of the root `DataSet`
///
///
#[derive(Debug)]
pub enum Container {
    Category(Category),
    Table(Table),
}

impl Container {
    pub fn name(&self) -> String {
        match self {
            Container::Category(x) => x.key.clone(),
            Container::Table(y) => y.key.clone(),
        }
    }
    pub fn to_table(&self) -> Result<&Table, &'static str> {
        match self {
            Container::Table(x) => Ok(x),
            _ => Err("Unknown Container"),
        }
    }
}

impl fmt::Display for Container {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Container::Category(ctg) => write!(f, "Category::key: {}", ctg.key),
            Container::Table(tbl) => write!(f, "Table:: {}", tbl.key),
        }
    }
}

///
/// This class allows the storage of key/value data termed _attributes_ in the mmcif specs.
///
/// Attributes are gathered in a single structure called _category_ according to the mmcif specs.
///
/// A Category is the child of the root `DataSet`
///
///
#[derive(Debug)]
pub struct Category {
    pub key: String,
    attrs: Vec<Attribute>, // Attributes
}
impl Category {
    pub fn new(name: String) -> Self {
        Category {
            key: name,
            attrs: Vec::<Attribute>::new(),
        }
    }
    pub fn push(&mut self, attr: Attribute) {
        self.attrs.push(attr);
    }
}

impl fmt::Display for Category {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "Category {}", self.key)
    }
}

///
/// This class allows the storage of tabular data.
///
/// These data are defined as rows and the columns are specified by their respective headers.
///
/// A Table is the child of the root `DataSet`
///
///
#[derive(Debug)]
pub struct Table {
    key: String,
    header: Vec<String>,
    rows: Vec<Row>,
}

impl Table {
    pub fn new(name: String) -> Self {
        Table {
            key: name,
            header: Vec::<String>::new(),
            rows: Vec::<Row>::new(),
        }
    }
    pub fn set_head_name(&mut self, name: String) {
        self.key = name;
    }
    pub fn add_column_head(&mut self, name: String) {
        self.header.push(name);
    }
    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn get_column(&self, index: usize) -> Vec<&Field> {
        let mut col = Vec::<&Field>::new();
        for row in &self.rows {
            for (i, field) in row.cells.iter().enumerate() {
                if i == index {
                    col.push(field);
                }
            }
        }
        col
    }
}

///
/// This class allows the storage of one row in tabular data.
///
///
#[derive(Debug)]
pub struct Row {
    index: usize,
    cells: Vec<Field>,
}

impl Row {
    pub fn new(index: usize) -> Self {
        Row {
            index,
            cells: Vec::<Field>::new(),
        }
    }
    pub fn push(&mut self, item: Field) {
        self.cells.push(item);
    }
}

///
/// The value stored in an Attribute may be a Number (f64) or a Text (String depending of the context)
///
#[derive(Debug)]
pub enum Field {
    Number(f64),
    Text(String),
}

///
/// This class allows the storage of key/value data.
///
#[derive(Debug)]
pub struct Attribute {
    key: String,
    value: Field, // String or Number
}

impl Attribute {
    pub fn new(key: String, value: Field) -> Self {
        Attribute { key, value }
    }
}
