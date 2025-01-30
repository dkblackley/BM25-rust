use crate::error::Result;
use regex::Regex;
use serde_json::Value;
use std::{
    fs::File,
    io::{BufRead as _, BufReader},
};

fn remove_numbers(input: &str) -> String {
    let re = Regex::new(r"\d+[,\d]*\.?\d*").unwrap();
    re.replace_all(input, "").to_string()
}

pub fn return_data_as_string(filename: &str) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let result_vec: Vec<String> = reader
        .lines()
        .map(|line| -> Result<String> {
            let line = line?;
            let json_val: Value = serde_json::from_str(&line)?;
            //            Ok(remove_numbers(&json_val["text"].to_string()))
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

    #[test]
    fn test_arxiv() {
        let corpus = return_data_as_string("../arxiv-metadata-oai-snapshot.json").unwrap();
        info!("{}", corpus[0]);
    }
}
