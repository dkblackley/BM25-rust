//! main.rs - the main entrypoint into the calculator.

/// bm_calc.rs - crate responsible for calculating top-k and BM25 searching.
pub(crate) mod bm_calc;
/// Crate that loads in data and puts it into a vector. Useful for the format the BM25 crate expects it.
pub(crate) mod dataloader;
/// error.rs - this holds a single enum that we can put our errors into.
pub(crate) mod error;
pub(crate) mod plotter;

use std::collections::HashSet;

use clap::{arg, command, Parser};
use tracing::info;
use crate::plotter::print_table;

/// Clap structure used to quickly parse cmd args
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The number of choices to use in d-choice hashsing
    #[arg(short, long, default_value_t = 10)]
    d: usize,

    /// K parameter for top-k
    #[arg(short, long, default_value_t = 10)]
    k: usize,

    /// Filter K parameter. If a result returns less than this value it is discarded. i.e. if we set this to 2 and get a top-k value of 1 it is ignored.
    #[arg(long, default_value_t = 5)]
    filter_k: usize,

    /// Path to the jsonl file to read
    #[arg(short, long)]
    file: String,

    /// The key in the JSON which holds the file/text we want to search over.
    #[arg(long, long, default_value = "text")]
    key: String,
}


#[derive(Copy, Clone, Debug)]
pub struct Config {
    pub k: usize,
    pub d: usize,
    pub max_bins: usize,
    pub filter_k: usize,
    pub max_load_factor: usize,
    pub min_overlap_factor: usize,
    pub save_result: bool,
}


impl Default for Config {
    fn default() -> Self {
        Config {
            k: 10,
            d: 4,
            max_bins: 1024,
            filter_k: 1,
            max_load_factor: 1,
            min_overlap_factor: 1,
            save_result: true,
        }
    }
}


fn main() {
    tracing_subscriber::fmt()
        .with_test_writer()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let args = Args::parse();

    let k = args.k;
    let filter_k = args.filter_k;

    info!("Starting BM25 calculation");
    let corpus = dataloader::return_data_as_string(&args.file, &args.key).unwrap();

    let alphabet = bm_calc::get_alphabet(&corpus).unwrap();

    info!(
        "The total number of files is {} and the alphabet size is {}",
        corpus.len(),
        alphabet.len()
    );

    let search = bm_calc::build_search_engine(corpus);


    let top_k_res = bm_calc::top_k(k, &search, &alphabet, filter_k);
    info!("Top K Done");
    plotter::fullness_histogram(
        top_k_res.values().cloned().collect(),
        true,
        &"Top K (No bins)".to_string(),
        top_k_res.values().len() as i32,
    )
    .expect("TODO: panic message");

    let max_bins = top_k_res.values().len() / 10;

    let mut config = Config::default();

    config.d = 1;
    config.k = 10;
    config.max_bins = max_bins;
    config.filter_k = filter_k;
    config.max_load_factor = 0;
    config.min_overlap_factor = 0;
    config.save_result = true;

    let no_choice_bins =
        bm_calc::top_k_bins(&search, &alphabet, config)
            .expect("TODO: panic message");
    plotter::fullness_histogram(
        no_choice_bins.1.clone(),
        true,
        &format!("Top K 1-choice {max_bins}-bins"),
        max_bins as i32,
    )
    .expect("TODO: panic message");

    config.d = 2;
    config.min_overlap_factor = 1;

    let two_choice_bins =
        bm_calc::top_k_bins(&search, &alphabet, config)
            .expect("TODO: panic message");
    plotter::fullness_histogram(
        two_choice_bins.1.clone(),
        true,
        &format!("Top K 2-choice {max_bins}-bins").to_string(),
        max_bins as i32,
    )
    .expect("TODO: panic message");

    config.d = 3;
    config.min_overlap_factor = 2;

    let three_choice_bins =
        bm_calc::top_k_bins(&search, &alphabet, config)
            .expect("TODO: panic message");
    plotter::fullness_histogram(
        three_choice_bins.1.clone(),
        true,
        &format!("Top K 3-choice {max_bins}-bins").to_string(),
        max_bins as i32,
    )
    .expect("TODO: panic message");

    config.d = 3;
    config.min_overlap_factor = 1;
    config.max_load_factor = 1;

    let three_choice_bins_remove_one =
        bm_calc::top_k_bins(&search, &alphabet,config)
            .expect("TODO: panic message");
    plotter::fullness_histogram(
        three_choice_bins_remove_one.1.clone(),
        true,
        &format!("3-choice, {max_bins}-bins and 1 max-load bin removed"),
        max_bins as i32,
    )
    .expect("TODO: panic message");

    config.d = 2;
    config.min_overlap_factor = 0;
    config.max_load_factor = 1;

    let two_choice_bins_max_load =
        bm_calc::top_k_bins(&search, &alphabet, config)
            .expect("TODO: panic message");
    plotter::fullness_histogram(
        two_choice_bins_max_load.1.clone(),
        true,
        &format!("Top K 2-choice {max_bins}-bins, minimising load"),
        max_bins as i32,
    )
    .expect("TODO: panic message");

    config.d = 100;
    config.max_load_factor = 10;
    config.min_overlap_factor = 89;
    let hundred_choice_ten_max_load =
        bm_calc::top_k_bins(&search, &alphabet, config)
            .expect("TODO: panic message");
    plotter::fullness_histogram(
        hundred_choice_ten_max_load.1.clone(),
        true,
        &format!("Top K 100-choice {max_bins}-bins, remove 10 max load bins"),
        max_bins as i32,
    )
    .expect("TODO: panic message");

    config.d = 4;
    config.min_overlap_factor = 1;
    config.max_load_factor = 1;

    // do 4-choice remove max load and min overlap. Store in other 2.
    let four_choice_min_overlap_max_overlap =
        bm_calc::top_k_bins(&search, &alphabet, config)
            .expect("TODO: panic message");
    plotter::fullness_histogram(
        four_choice_min_overlap_max_overlap.1.clone(),
        true,
        &format!("Top K 100-choice {max_bins}-bins, remove 10 max load bins"),
        max_bins as i32,
    )
        .expect("TODO: panic message");

    let mut format_strings = Vec::new();
    let mut results = Vec::new();

    // Collect all format strings
    format_strings.extend_from_slice(&[
        format!("1-choice {max_bins}-bins"),
        format!("2-choice {max_bins}-bins"),
        format!("3-choice {max_bins}-bins"),
        format!("3-choice, {max_bins}-bins and 1 max-load bin removed"),
        format!("2-choice {max_bins}-bins, minimising load"),
        format!("100-choice {max_bins}-bins, remove 10 max load bins"),
        format!("4-choice {max_bins}-bins, remove 1 min overlap, 1 max load"),
    ]);

    // Collect all results
    results.extend_from_slice(&[
        no_choice_bins.0,
        two_choice_bins.0,
        three_choice_bins.0,
        three_choice_bins_remove_one.0,
        two_choice_bins_max_load.0,
        hundred_choice_ten_max_load.0,
        four_choice_min_overlap_max_overlap.0,
    ]);

    print_table(&format_strings, &results).unwrap();
}

pub fn calculate_emd(bins1: &[HashSet<u32>], bins2: &[HashSet<u32>]) -> f64 {
    // Get distributions (number of items in each bin)
    let mut dist1: Vec<usize> = bins1.iter().map(|bin| bin.len()).collect();
    let mut dist2: Vec<usize> = bins2.iter().map(|bin| bin.len()).collect();

    // Sort the distributions to minimize total distance
    dist1.sort_unstable();
    dist2.sort_unstable();

    // Make sure distributions have same length
    let num_bins = dist1.len().max(dist2.len());
    dist1.resize(num_bins, 0);
    dist2.resize(num_bins, 0);

    // Calculate total items (should be same in both distributions)
    let total_items1: usize = dist1.iter().sum();
    let total_items2: usize = dist2.iter().sum();

    if total_items1 != total_items2 {
        println!(
            "Warning: Distributions have different total items: {} vs {}",
            total_items1, total_items2
        );
    }

    // Calculate EMD
    let mut total_work = 0.0;
    let mut running_sum = 0.0;

    // Calculate cumulative difference between distributions
    for i in 0..num_bins {
        running_sum += dist1[i] as f64 - dist2[i] as f64;
        total_work += running_sum.abs();
    }

    // Normalize by total number of items
    total_work / total_items1.max(total_items2) as f64
}

// Helper function to print comparison stats
pub fn print_distribution_comparison(bins1: &[HashSet<u32>], bins2: &[HashSet<u32>]) {
    let sizes1: Vec<usize> = bins1.iter().map(|bin| bin.len()).collect();
    let sizes2: Vec<usize> = bins2.iter().map(|bin| bin.len()).collect();

    let total_items1: usize = sizes1.iter().sum();
    let total_items2: usize = sizes2.iter().sum();

    let avg_size1 = total_items1 as f64 / bins1.len() as f64;
    let avg_size2 = total_items2 as f64 / bins2.len() as f64;

    info!("Distribution comparison:");
    info!("Distribution 1:");
    info!("  Total items: {}", total_items1);
    info!("  Number of bins: {}", bins1.len());
    info!("  Average bin size: {:.2}", avg_size1);
    info!("Distribution 2:");
    info!("  Total items: {}", total_items2);
    info!("  Number of bins: {}", bins2.len());
    info!("  Average bin size: {:.2}", avg_size2);
    info!(
        "EMD between distributions: {:.4}",
        calculate_emd(bins1, bins2)
    );
}
