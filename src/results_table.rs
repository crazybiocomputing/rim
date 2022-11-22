//
//  RIM - Rust IMage
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

///
/// This class allows the storage of tabular data.
///
/// These data are defined as rows and the columns are specified by their respective headers.
///
/// A Table is the child of the root `DataSet`
///
///

use core::slice::Iter;

#[derive(Debug)]
pub struct ResultsTable {
    name: String,
    headings: Vec<String>,
    columns: Vec<Column>,
    count: usize,
}

impl ResultsTable {
    pub fn new(name: String) -> Self {
        ResultsTable {
            name,
            headings: Vec::<String>::new(),
            columns: Vec::<Column>::new(),
            count: 0,
        }
    }

    ///
    /// Returns the rows number (aka size) of this ResultsTable
    ///
    pub fn size(&self) -> usize {
        self.count
    }

    ///
    ///
    ///
    pub fn set_head_name(&mut self, name: String) {
        self.name = name;
    }

    ///
    /// Returns the index of the last used column, or -1 if no columns are used.
    ///
    pub fn get_last_column(&self) -> i32 {
        self.columns.len() as i32 - 1
    }

    ///
    /// Returns a comma delimited string containing the column headings.
    ///
    pub fn get_column_headings(&self) -> String {
        self.headings.join(",")
    }

    ///
    /// Returns the column headings as an array of Strings.
    ///
    pub fn get_headings(&self) -> Vec<String> {
        self.headings.iter().map(|h| h.clone()).collect()
    }

    ///
    /// Returns the heading of the specified column or None if the column is empty.
    ///
    pub fn get_column_heading(&self, column: usize) -> Option<&String> {
        self.headings.get(column)
    }

    ///
    /// Returns the index of the first column with the given heading. heading.
    /// If not found, returns COLUMN_NOT_FOUND.
    pub fn get_column_index(&self, heading: &String) -> Result<usize, &str> {
        match self.headings.iter().position(|h| h == heading) {
            Some(x) => Ok(x),
            None => Err("COLUMN NOT FOUND"),
        }
    }

    ///
    /// Returns the value of the given column and row, where column must
    /// be less than or equal the value returned by [get_last_column()](ResultsTable::get_last_column) and row
    /// must be greater than or equal zero and less than the value
    /// returned by [size()](ResultsTable::size).
    pub fn get_value_as_float(&self, column: usize, row: usize) -> f64 {
        // TODO
        // self.columns.get(column)
        0.0
    }

    ///
    /// Returns the value of the specified column and row, where column
    /// is the column heading and row is a number greater than or equal zero
    /// and less than value returned by [size()](ResultsTable::size).
    ///
    /// Throws an IllegalArgumentException
    /// if this ResultsTable does not have a column with the specified heading.
    pub fn get_value(&self, column: &String, row: usize) -> Result<&Cell, &str> {
        let icol = self.get_column_index(column);
        if icol.is_ok() {
            return match (icol.unwrap(), row) {
                (col, rw)
                    if col >= 0
                        && col as i32 <= self.get_last_column()
                        && rw >= 0
                        && rw < self.size() =>
                {
                    Ok(&self.columns[col].cells[rw])
                }
                _ => Err("Column and/or Row Out of range"),
            };
        }
        return Err("Column and/or Row Out of range");
    }

    ///
    /// Returns `true` if the specified column exists and is not empty.
    ///
    pub fn column_exists(&self, column: &String) -> bool {
        if let Ok(icol) = self.get_column_index(column) {
            true
        } else {
            false
        }
    }

    ///
    /// Returns the string value of the given column and row, where row must
    /// be greater than or equal zero and less than the value returned by size().
    ///
    pub fn get_string_value(&self, column: &String, row: usize) -> String {
        "TODO".to_string()
    }

    ///
    /// Returns the string value of the given column and row,
    /// where column must be less than or equal the value returned
    /// by [get_last_column()](ResultsTable::get_last_column)() and row must be greater than or equal zero
    /// and less than the value returned by [size()](ResultsTable::size).
    ///
    pub fn get_string_value_at(&self, column: usize, row: usize) -> Result<&String, &str> {
        Err("TODO")
    }

    ///
    /// Adds a numeric or text value to the specified column index, on the last table row.
    ///
    /// Use add_row() to add another row to the table.
    ///
    pub fn add_value_at(&mut self, column: usize, value: Cell) {
        self.columns[column].cells[self.count - 1] = value;
    }

    ///
    /// Adds a numeric value to the specified column, on the last table row.
    /// If the column does not exist, it is created. Use add_row() to add another row to the table.
    ///
    /// # Example:
    ///
    /// ``` rust
    /// let rt = ResultsTable()::new();
    /// for i in 0..360 {
    ///    rt.add_row();
    ///    let n = (i as f64 ) / 180.0 * std::f64::consts::PI;
    ///    rt.add_value("n", n);
    ///    rt.add_value("Sine(n)", n.sin());
    ///    rt.add_value("Cos(n)", n.cos());
    /// }
    ///
    /// ```
    ///
    pub fn add_value(&mut self, column: &String, value: Cell) {
        let index = self.get_column_index(&column);
        let mut icol: usize = 0;
        // Create a new column if needed
        if index.is_err() {
            self.headings.push(column.clone());
            icol = self.headings.len() - 1;
            let mut new_col = Column::new(column.clone(), icol);
            // Fill with Cell::None (empty)
            new_col.cells = (0..self.count).map(|_i| Cell::None).collect();
            self.columns.push(new_col);
        } else {
            icol = index.unwrap();
        }
        self.add_value_at(icol, value);
    }

    pub fn add_value_float(&mut self, column: String, value: f64) {}

    pub fn add_value_string(&mut self, column: String, value: String) {}

    pub fn add_column_head(&mut self, name: String) {
        self.headings.push(name);
    }

    ///
    /// Adds an empty row to the table.
    ///
    pub fn add_row(&mut self) {
        self.count += 1;
        for col in self.columns.iter_mut() {
            col.push(Cell::None);
        }
    }

    ///
    /// Get row content at given index and returns 
    /// the row as a vector.
    ///
    pub fn get_row_at(&self, index: usize) -> Vec<&Cell> {
        let mut row = Vec::<&Cell>::new();
        for col in &self.columns {
            for (i, cell) in col.cells.iter().enumerate() {
                if i == index {
                    row.push(cell);
                }
            }
        }
        row
    }

    /// Returns a copy of the given column,
    /// or None if the column is not found.
    pub fn get_column(&self, column: String) -> Result<&Column, &str> {
        let index = self.get_column_index(&column);
        match index {
            Ok(icol) => Ok(&self.columns[icol]),
            Err(e) => Err(e),
        }
    }

    /// Returns a copy of the given column as a String array,
    /// or null if the column is not found.
    pub fn get_column_as_strings(&self, column: String) -> Result<Vec<String>, &str> {
        let index = self.get_column_index(&column);
        match index {
            Ok(icol) => Ok(self.columns[icol].cells().iter().map(|cell| cell.to_string()).collect()),
            Err(e) => Err(e),
        }
    }

    /// Returns a copy of the given column as a f64 array,
    /// or null if the column is empty.
    pub fn get_column_as_floats(&self, column: String) -> Result<Vec<f64>,&str> {
        let index = self.get_column_index(&column);
        match index {
            Ok(icol) => Ok(self.columns[icol].cells().iter().map(|cell| cell.to_f64()).collect()),
            Err(e) => Err(e),
        }
    }

    /// Sets the values of the given column to the values in the array.
    /// If the specified column does not exist, it is created.
    ///
    /// If the array is shorter than the column length, the remaining values of the column are
    /// left unchanged. If the array is longer, the table is extended.
    /// String values are unaffected, but only used if the numeric value at the given position is NaN.
    pub fn set_values(&mut self, column: &String, values: &Vec<Cell>) {}

    /// Sets the value of the given column and row, where 0<=row<size().
    /// If the specified column does not exist, it is created.
    ///
    pub fn set_value(&mut self, column: &String, row: usize, value: Cell) {}
}

/*
setValue

public void setValue​(int column, int row, double value)

Sets the value of the given column and row, where where 0<=column<=(lastRow+1 and 0<=row<=size().
setValue

public void setValue​(java.lang.String column, int row, java.lang.String value)

Sets the string value of the given column and row, where where 0<=row<size(). If the specified column does not exist, it is created. When adding columns, show() must be called to update the window that displays the table.
setValue

public void setValue​(int column, int row, java.lang.String value)

Sets the string value of the given column and row, where where 0<=column<=(lastRow+1 and 0<=row<=size().
setValues
*/

/*
        getResultsTable

        public static ResultsTable getResultsTable()

        Returns the ResultsTable used by the Measure command. This table must be displayed in the "Results" window.
        getResultsTable

        public static ResultsTable getResultsTable​(java.lang.String title)

        Returns the ResultsTable with the specified title, or null if it does not exist,
        getActiveTable

        public static ResultsTable getActiveTable()

        Returns the active (front most) displayed ResultsTable.
        getResultsWindow


        public void addRow()



        public void incrementCounter()

        Adds a row to the table.
        addColumns

        public void addColumns()

        Obsolete; the addValue() method automatically adds columns as needed.

        See Also:
            addValue(String, double)

        getCounter

        public int getCounter()

        Returns the current value of the measurement counter.
        size

e.
        addValue

        public void addValue​(int column, double value)

        Adds a numeric value to the specified column, on the last table row. Use addRow() to add another row to the table.

        See Also:
            addRow(), addValue(String,double), addValue(String,String), size()

        addValue




        See Also:
            addRow(), addValue(String,String), size()

        addValue

        public void addValue​(java.lang.String column, java.lang.String value)

        Adds a string value to the specified column, on the last table row. If the column does not exist, it is created. Use addRow() to add another row to the table.

        See Also:
            addRow(), addValue(String,double), size()

        addLabel

        public void addLabel​(java.lang.String label)

        Adds a label to the beginning of the current row.
        addLabel

        public void addLabel​(java.lang.String columnHeading, java.lang.String label)

        Deprecated.
        Replaced by setValue(String,int,String)
        setLabel

        public void setLabel​(java.lang.String label, int row)

        Adds a label to the beginning of the specified row, or updates an existing lable, where 0<=rowshow() to update the window displaying the table.
        disableRowLabels

        public void disableRowLabels()

        Set the row label column to null if the column label is "Label".
        getColumn

        public double[] getColumn​(java.lang.String column)

        Returns a copy of the given column as a double array, or null if the column is not found.
        getColumnAsStrings

        public java.lang.String[] getColumnAsStrings​(java.lang.String column)

        Returns a copy of the given column as a String array, or null if the column is not found.
        getColumn

        public float[] getColumn​(int column)

        Returns a copy of the given column as a float array, or null if the column is empty.
        getColumnAsDoubles

        public double[] getColumnAsDoubles​(int column)

        Returns a copy of the given column as a double array, or null if the column is empty.
        getTableAsImage

        public ImageProcessor getTableAsImage()

        Returns the contents of this ResultsTable as a FloatProcessor.
        createTableFromImage

        public static ResultsTable createTableFromImage​(ImageProcessor ip)

        Creates a ResultsTable from an image or image selection.
        columnExists

        public boolean columnExists​(int column)

        Returns 'true' if the specified column exists and is not empty.
        getColumnIndex

        public int getColumnIndex​(java.lang.String heading)

        Returns the index of the first column with the given heading. heading. If not found, returns COLUMN_NOT_FOUND.
        getFreeColumn

        public int getFreeColumn​(java.lang.String heading)

        Sets the heading of the the first available column and returns that column's index. Returns COLUMN_IN_USE if this is a duplicate heading.
        getValueAsDouble

        public double getValueAsDouble​(int column, int row)

        Returns the value of the given column and row, where column must be less than or equal the value returned by getLastColumn() and row must be greater than or equal zero and less than the value returned by size().
        getValue

        public float getValue​(int column, int row)

        Deprecated.
        replaced by getValueAsDouble
        getValue

        public double getValue​(java.lang.String column, int row)

        Returns the value of the specified column and row, where column is the column heading and row is a number greater than or equal zero and less than value returned by size(). Throws an IllegalArgumentException if this ResultsTable does not have a column with the specified heading.
        columnExists

        public boolean columnExists​(java.lang.String column)

        Returns 'true' if the specified column exists and is not emptly.
        getStringValue

        public java.lang.String getStringValue​(java.lang.String column, int row)

        Returns the string value of the given column and row, where row must be greater than or equal zero and less than the value returned by size().
        getStringValue

        public java.lang.String getStringValue​(int column, int row)

        Returns the string value of the given column and row, where column must be less than or equal the value returned by getLastColumn() and row must be greater than or equal zero and less than the value returned by size().
        getLabel

        public java.lang.String getLabel​(int row)

        Returns the label of the specified row. Returns null if the row does not have a label.


        getColumnHeadings

        public java.lang.String getColumnHeadings()

        Returns a tab or comma delimited string containing the column headings.
        getHeadings

        public java.lang.String[] getHeadings()

        Returns the column headings as an array of Strings.
        getColumnHeading

        public java.lang.String getColumnHeading​(int column)

        Returns the heading of the specified column or null if the column is empty.
        getRowAsString

        public java.lang.String getRowAsString​(int row)

        Returns a tab or comma delimited string representing the given row, where 0<=row<=size()-1.
        getColumnAsVariables

        public Variable[] getColumnAsVariables​(java.lang.String column)

        Implements the Table.getColumn() macro function.
        setColumn

        public void setColumn​(java.lang.String column, Variable[] array)

        Implements the Table.setColumn() macro function.
        setHeading

        public void setHeading​(int column, java.lang.String heading)

        Deprecated.
        Replaced by addValue(String,double) and setValue(String,int,double)
        setDefaultHeadings

        public void setDefaultHeadings()

        Sets the headings used by the Measure command ("Area", "Mean", etc.).
        setPrecision

        public void setPrecision​(int precision)

        Sets the decimal places (digits to the right of decimal point) that are used when this table is displayed.
        setDecimalPlaces

        public void setDecimalPlaces​(int column, int digits)

        setNaNEmptyCells

        public void setNaNEmptyCells​(boolean NaNEmptyCells)

        Set 'true' to initially fill data arrays with NaNs instead of zeros.
        saveColumnHeaders

        public void saveColumnHeaders​(boolean save)

        d2s

        public static java.lang.String d2s​(double n, int decimalPlaces)

        This is a version of IJ.d2s() that uses scientific notation for small numbes that would otherwise display as zero.
        deleteRow

        public void deleteRow​(int rowIndex)

        Deletes the specified row.
        deleteRows

        public void deleteRows​(int index1, int index2)

        Deletes the specified rows.
        deleteColumn

        public void deleteColumn​(java.lang.String column)

        Deletes the specified column.
        renameColumn

        public void renameColumn​(java.lang.String oldName, java.lang.String newName)

        Changes the name of a column.
        reset

        public void reset()


        addResults

        public void addResults()

        Adds the last row in this table to the Results window without updating it.
        updateResults

        public void updateResults()

        Updates the Results window.

        public static ResultsTable open2​(java.lang.String path)

        Opens a tab or comma delimited text file and returns it as a ResultsTable, without requiring a try/catch statement. Displays a file open dialog if 'path' is empty or null.
        open

        public static ResultsTable open​(java.lang.String path) throws java.io.IOException

        Opens a tab or comma delimited text file and returns it as a ResultsTable. Displays a file open dialog if 'path' is empty or null.

        Throws:
            java.io.IOException
        See Also:
            open2(String)

        save

        public boolean save​(java.lang.String path)

        Saves this ResultsTable as a tab or comma delimited text file. The table is saved as a CSV (comma-separated values) file if 'path' ends with ".csv". Displays a file save dialog if 'path' is empty or null. Does nothing if the table is empty. Displays an error message and returns 'false' if there is an error.
        saveAndRename

        public boolean saveAndRename​(java.lang.String path)

        saveAs

        public void saveAs​(java.lang.String path) throws java.io.IOException

        Throws:
            java.io.IOException


        public boolean applyMacro​(java.lang.String macro)

        Applies a macro to each row of the table; the columns are assigned variable names as given by getHeadingsAsVaribleNames(). New variables starting with an uppercase letter create a new column with this name. The variable 'row' (the row index) is pre-defined. Except for the row label (if existing), currently only supports numeric values, no Strings.

        Returns:
            false in case of a macro error

        getHeadingsAsVariableNames

        public java.lang.String[] getHeadingsAsVariableNames()

        Returns the column headings; headings not suitable as variable names are converted to valid variable names by replacing non-fitting characters with underscores and adding underscores. To make unique names, underscores+numbers are added as required.
        getTitle

        public java.lang.String getTitle()

        columnDeleted

        public boolean columnDeleted()

        selectRow

        public static boolean selectRow​(Roi roi)

        Selects the row in the "Results" table assocuiated with the specified Roi. The row number is obtained from the roi name..
        sort

        public void sort​(java.lang.String column)

        Sorts this table on the specified column, with string support. Author: 'mountain_man', 8 April 2019
        setIsResultsTable

        public void setIsResultsTable​(boolean isResultsTable)

*/
///
/// This class allows the storage of one row in tabular data.
///
///
#[derive(Debug)]
pub struct Column {
    index: usize,
    title: String,
    cells: Vec<Cell>,
    icount: usize
}

impl Column {
    pub fn new(title: String, index: usize) -> Self {
        Column {
            index,
            title,
            cells: Vec::<Cell>::new(),
            icount: 0
        }
    }
    pub fn push(&mut self, item: Cell) {
        self.cells.push(item);
    }

    pub fn to_vec(&self) -> Vec<Cell> {
        self.cells.clone()
    }
    pub fn get_value(&self, index: usize) -> Cell {
        self.cells[index].clone()
    }
    pub fn cells(&self) -> &Vec<Cell> {
      &self.cells
    }
}


impl Iterator for Column {
    // we will be counting with usize
    type Item = Cell;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        // Increment our count. This is why we started at zero.
        self.icount += 1;

        // Check to see if we've finished counting or not.
        if self.icount < self.cells.len() {
            Some(self.cells[self.icount].clone())
        } else {
            None
        }
    }
}
///
/// The value stored in a column may be a Number (f64) or a Text 
/// (String depending of the context)
///
#[derive(Debug, Clone)]
pub enum Cell {
    Number(f64),
    Text(String),
    None,
}

impl Cell {
    pub fn to_f64(&self) -> f64 {
        match self {
            Cell::Number(x) => *x,
            _ => f64::NAN,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Cell::Number(x) => x.to_string(),
            Cell::Text(w) => w.clone(),
            Cell::None => String::from("None"),
        }
    }
}
#[cfg(test)]
mod tests {

    use crate::results_table::{Cell, ResultsTable};

    #[test]
    fn new_table_with_two_rows() {
        let mut rt = ResultsTable::new("Test".to_string());
        rt.add_row();
        rt.add_row();

        assert_eq!(rt.name, String::from("Test"));
        assert_eq!(rt.count, 2);
    }

    #[test]
    fn get_headings() {
        let mut rt = ResultsTable::new("Test".to_string());
        rt.add_row();
        rt.add_value(&"num".to_string(), Cell::Number(3.14));

        assert_eq!(rt.name, String::from("Test"));
        assert_eq!(rt.count, 2);
    }

    #[test]
    fn new_table_with_one_column_two_rows() {
        // Private
        fn eq_with_nan_eq(a: f64, b: f64) -> bool {
            (a.is_nan() && b.is_nan()) || (a == b)
        }
        // Create Table
        let mut rt = ResultsTable::new("Test".to_string());
        rt.add_row();
        rt.add_row();
        rt.add_value(&"num".to_string(), Cell::Number(3.14));

        // Convert to a Vec for assertion
        let mut arr = Vec::<f64>::new();
        for cell in rt.columns[0].cells.iter() {
            match cell {
                Cell::Number(x) => arr.push(*x),
                Cell::None => arr.push(f64::NAN),
                _ => panic!("Wrong value"),
            }
        }
        // Assertion
        assert!(arr
            .iter()
            .zip(vec![f64::NAN, 3.14])
            .all(|(a, b)| eq_with_nan_eq(*a, b)));
    }

    #[test]
    fn create_sin_cos_table() {
        let mut rt = ResultsTable::new("SinCos Table".to_string());
        for i in 0..360 {
            rt.add_row();
            let n = (i as f64) / 180.0 * std::f64::consts::PI;
            rt.add_value(&"n".to_string(), Cell::Number(n));
            rt.add_value(&"Sine(n)".to_string(), Cell::Number(n.sin()));
            rt.add_value(&"Cos(n)".to_string(), Cell::Number(n.cos()));
        }
        assert_eq!(
            rt.headings,
            vec![
                String::from("n"),
                String::from("Sine(n)"),
                String::from("Cos(n)")
            ]
        );
    }
    #[test]
    fn get_first_column_entitled_a_as_floats() {
        /*
            A,B,Species
            1,2.0,Chicken
            Wrong,5.0,Pig
            7,8.0,Human
        */
        let mut rt = ResultsTable::new("Table".to_string());
        rt.add_row();
        rt.add_value(&"A".to_string(), Cell::Number(1 as f64));
        rt.add_value(&"B".to_string(), Cell::Number(2.0));
        rt.add_value(&"Species".to_string(), Cell::Text("Chicken".to_string()));
        rt.add_row();
        rt.add_value(&"A".to_string(), Cell::Text("Wrong".to_string()));
        rt.add_value(&"B".to_string(), Cell::Number(5.0));
        rt.add_value(&"Species".to_string(), Cell::Text("Pig".to_string()));
        rt.add_row();
        rt.add_value(&"A".to_string(), Cell::Number(7 as f64));
        rt.add_value(&"B".to_string(), Cell::Number(8.0));
        rt.add_value(&"Species".to_string(), Cell::Text("Human".to_string()));

        let answer = vec![1.0,f64::NAN,7.0];
        assert_eq!(answer,rt.get_column_as_floats("A".to_string()).unwrap());
    }
    
    #[test]
    fn get_third_column_entitled_species_as_strings() {
        /*
            A,B,Species
            1,2.0,Chicken
            4,5.0,Pig
            7,8.0,Human
        */
        let mut rt = ResultsTable::new("Table".to_string());
        rt.add_row();
        rt.add_value(&"A".to_string(), Cell::Number(1 as f64));
        rt.add_value(&"B".to_string(), Cell::Number(2.0));
        rt.add_value(&"Species".to_string(), Cell::Text("Chicken".to_string()));
        rt.add_row();
        rt.add_value(&"A".to_string(), Cell::Number(4 as f64));
        rt.add_value(&"B".to_string(), Cell::Number(5.0));
        rt.add_value(&"Species".to_string(), Cell::Text("Pig".to_string()));
        rt.add_row();
        rt.add_value(&"A".to_string(), Cell::Number(7 as f64));
        rt.add_value(&"B".to_string(), Cell::Number(8.0));
        rt.add_value(&"Species".to_string(), Cell::Text("Human".to_string()));

        let answer = vec!["Chicken".to_string(),"Pig".to_string(),"Human".to_string()];
        assert_eq!(answer,rt.get_column_as_strings("Species".to_string()).unwrap());
    }
}
