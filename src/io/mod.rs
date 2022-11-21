pub mod file_info;

pub mod image_reader;
pub mod raw_reader;
pub mod image_writer;
///
/// Example of tabular data in STAR format
///
///    ```text
///    data_MUSIC
///    #
///    loop_
///    _rockBand.id
///    _rockBand.value
///    _rockBand.name
///    1    10    'The Stones'
///    2    20    'Deep Purple'
///    3    30    Stromae
///    #
///    ```
///
///   # Source code
///    ```rust
///    // Create DataSet
///    let mut db_music = DataSet::new("MUSIC".to_string());
///    // Create table
///    let mut table = Table::new("rockBand".to_string());
///    // Add the column headers
///    table.add_column_head("id".to_string());
///    table.add_column_head("value".to_string());
///    table.add_column_head("name".to_string());
///
///    // Create the rows
///    let mut row0 = Row::new(0);
///    row0.push(Field::Number(1.0));
///    row0.push(Field::Number(10.0));
///    row0.push(Field::Text("The Stones".to_string()));
///    table.add_row(row0);
///    let mut row1 = Row::new(1);
///    row1.push(Field::Number(2.0));
///    row1.push(Field::Number(20.0));
///    row1.push(Field::Text("Deep Purple".to_string()));
///    table.add_row(row1);
///    let mut row2 = Row::new(2);
///    row2.push(Field::Number(3.0));
///    row2.push(Field::Number(30.0));
///    row2.push(Field::Text("Stromae".to_string()));
///    table.add_row(row2);
///
///    // Add table in db
///    db_music.push(Container::Table(table));
///    ```
///
pub mod star;
pub mod star_parser;
pub mod star_reader;
pub mod text_reader;
