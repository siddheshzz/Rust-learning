use polars::prelude::*;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;

fn df_to_row_set(df: &DataFrame) -> HashSet<Vec<String>> {
    let mut rows = HashSet::new();
    for i in 0..df.height() {
        let row: Vec<String> = df
            .get_columns()
            .iter()
            .map(|s| s.get(i).unwrap().to_string())
            .collect();
        rows.insert(row);
    }
    rows
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load the CSV files into DataFrames
    let df1 = CsvReader::from_path("c1.csv")?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    let df2 = CsvReader::from_path("c2.csv")?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    // Convert to sets of rows
    let set1 = df_to_row_set(&df1);
    let set2 = df_to_row_set(&df2);

    // Rows in df1 not in df2 and vice versa
    let only_in_1 = set1.difference(&set2);
    let only_in_2 = set2.difference(&set1);

    // Combine differences
    let all_diffs: Vec<Vec<String>> = only_in_1.chain(only_in_2).cloned().collect();

    if all_diffs.is_empty() {
        println!("No differences found.");
        return Ok(());
    }

    // Rebuild a DataFrame from rows
    let headers: Vec<String> = df1.get_column_names().iter().map(|s| s.to_string()).collect();
    let columns: Vec<Series> = (0..headers.len())
        .map(|col_idx| {
            let col_values: Vec<&str> = all_diffs.iter().map(|row| row[col_idx].as_str()).collect();
            Series::new(&headers[col_idx], col_values)
        })
        .collect();

    let mut diff_df = DataFrame::new(columns)?;

    // Write the differences to a CSV
    let mut file = File::create("diff.csv")?;
    CsvWriter::new(&mut file)
        .include_header(true)
        .finish(&mut diff_df)?;

    println!("Differences written to diff.csv");

    Ok(())
}
