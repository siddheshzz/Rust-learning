use polars::prelude::*;

fn main(){
    
}








// use polars::prelude::*;
// use rayon::prelude::*;
// use std::error::Error;
// use std::fs::File;
// use std::io::Write;

// fn create_primary_key(df: &DataFrame, cols: &[&str]) -> Result<Series, Box<dyn Error>> {
//     let mut key = Vec::new();
//     for col in cols {
//         let series = df.column(*col)?.utf8()?.to_vec(); // Assuming text data
//         key.push(series);
//     }
//     let combined = Series::new("primary_key", key.into_iter().flatten().collect::<Vec<_>>());
//     Ok(combined)
// }

// fn compare_dataframes(df1: &DataFrame, df2: &DataFrame) -> Result<(), Box<dyn Error>> {
//     if df1.shape() != df2.shape() {
//         println!("DataFrames have different shapes: {:?} vs {:?}", df1.shape(), df2.shape());
//         return Ok(());
//     }

//     let mut report = Vec::new();

//     // Compare rows
//     for row in 0..df1.height() {
//         let row1 = df1.get_row(row)?;
//         let row2 = df2.get_row(row)?;

//         for (col, (val1, val2)) in row1.iter().zip(row2.iter()).enumerate() {
//             if val1 != val2 {
//                 report.push(format!("Difference found at row {}, column {}: {:?} vs {:?}", row, col, val1, val2));
//             }
//         }
//     }

//     // Write the report to a file
//     let mut file = File::create("comparison_report.txt")?;
//     for entry in report {
//         writeln!(file, "{}", entry)?;
//     }

//     Ok(())
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     // Read the CSV files
//     let df1 = CsvReader::from_path("file1.csv")?.has_header(true).finish()?;
//     let df2 = CsvReader::from_path("file2.csv")?.has_header(true).finish()?;

//     // Define primary key columns for sorting
//     let key_columns = vec!["column1", "column2"]; // Adjust based on your schema

//     // Create primary keys
//     let key1 = create_primary_key(&df1, &key_columns)?;
//     let key2 = create_primary_key(&df2, &key_columns)?;

//     // Add primary key as a new column
//     let df1 = df1.hstack(&[key1])?;
//     let df2 = df2.hstack(&[key2])?;

//     // Sort DataFrames by primary key
//     let df1 = df1.sort("primary_key", false)?;
//     let df2 = df2.sort("primary_key", false)?;

//     // Compare the DataFrames
//     compare_dataframes(&df1, &df2)?;

//     Ok(())
// }
