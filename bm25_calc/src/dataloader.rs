use crate::error::Result;
use regex::Regex;
use serde_json::Value;
use std::{
    fs::File,
    io::{BufRead as _, BufReader},
};

/// Removes all numeric values from a string
///
/// # Arguments
/// * `input` - String to process
///
/// # Returns
/// * `String` - Input with all numbers removed
#[allow(unused)]
fn remove_numbers(input: &str) -> String {
    let re = Regex::new(r"\d+[,\d]*\.?\d*").unwrap();
    re.replace_all(input, "").to_string()
}

/// Reads JSON lines file and extracts text field values
///
/// # Arguments
/// * `filename` - Path to JSON lines file
/// * `key` - The key in the JSON string that holds the main body of text
///
/// # Returns
/// * `Result<Vec<String>>` - Vector of text values
///
/// # Errors
/// Returns error if file cannot be read or JSON is invalid
pub fn return_data_as_string(filename: &str, key: &str) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let result_vec: Vec<String> = reader
        .lines()
        .map(|line| -> Result<String> {
            let line = line?;
            let json_val: Value = serde_json::from_str(&line)?;
            Ok(json_val[key].to_string())
        })
        .collect::<Result<_>>()?;

    Ok(result_vec)
}
