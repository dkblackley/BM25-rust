use crate::error::Result;
use serde_json::Value;
use std::{
    fs::{self, File},
    io::{BufRead as _, BufReader},
};

pub fn return_data_as_string(filename: &str) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let result_vec: Vec<String> = reader
        .lines()
        .map(|line| -> Result<String> {
            let line = line?;
            let json_val: Value = serde_json::from_str(&line)?;
            Ok(json_val["text"].to_string())
        })
        .collect::<Result<_>>()?;

    Ok(result_vec)
}

#[cfg(test)]
mod tests {
    use tracing::info;

    use super::*;

    #[test]
    fn test_name() {
        let corpus = return_data_as_string("../scifact/corpus.jsonl").unwrap();
        info!("{}", corpus[0]);
    }
}
