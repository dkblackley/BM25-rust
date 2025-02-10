//! main.rs - the main entrypoint into the calculator.

/// bm_calc.rs - crate responsible for calculating top-k and BM25 searching.
pub(crate) mod bm_calc;
/// Crate that loads in data and puts it into a vector. Useful for the format the BM25 crate expects it.
pub(crate) mod dataloader;
/// error.rs - this holds a single enum that we can put our errors into.
pub(crate) mod error;
pub(crate) mod plotter;

use clap::{arg, command, Parser};
use tracing::{info};

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
    #[arg(long, default_value_t = 2)]
    filter_k: usize,

    /// Path to the jsonl file to read
    #[arg(short, long)]
    file: String,

    /// The key in the JSON which holds the file/text we want to search over.
    #[arg(long, long, default_value = "text")]
    key: String,
}

fn main() {
    tracing_subscriber::fmt()
        .with_test_writer()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let args = Args::parse();

    let d = args.d;
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
    plotter::fullness_histogram(top_k_res.values().map(|set| set.clone()).collect(), true, &"Top K (No bins)".to_string(), top_k_res.values().len() as i32);

    let max_bins = top_k_res.values().len() / 10;

    let no_choice_bins = bm_calc::top_k_bins(k, &search, &alphabet, 1, max_bins, filter_k, false, 0).expect("TODO: panic message");
    plotter::fullness_histogram(no_choice_bins, true, &format!("Top K 1-choice {max_bins}-bins").to_string(), max_bins as i32).expect("TODO: panic message");

    let two_choice_bins = bm_calc::top_k_bins(k, &search, &alphabet, 2, max_bins, filter_k, false, 0).expect("TODO: panic message");
    plotter::fullness_histogram(two_choice_bins, true, &format!("Top K 2-choice {max_bins}-bins").to_string(), max_bins as i32).expect("TODO: panic message");

    let three_choice_bins = bm_calc::top_k_bins(k, &search, &alphabet, 3, max_bins, filter_k, false, 0).expect("TODO: panic message");
    plotter::fullness_histogram(three_choice_bins, true, &format!("Top K 3-choice {max_bins}-bins").to_string(), max_bins as i32).expect("TODO: panic message");

    let three_choice_bins_remove_one = bm_calc::top_k_bins(k, &search, &alphabet, 3, max_bins, filter_k, false, 1).expect("TODO: panic message");
    plotter::fullness_histogram(three_choice_bins_remove_one, true, &format!("3-choice, {max_bins}-bins and 1 max-load bin removed").to_string(), max_bins as i32).expect("TODO: panic message");

    let two_choice_bins_max_load = bm_calc::top_k_bins(k, &search, &alphabet, 2, max_bins, filter_k, true, 0).expect("TODO: panic message");
    plotter::fullness_histogram(two_choice_bins_max_load, true, &format!("Top K 2-choice {max_bins}-bins, minimising load").to_string(), max_bins as i32).expect("TODO: panic message");


    // for i in 1..d {
    //     if let Err(e) = bm_calc::top_k_bins(k, &search, &alphabet, i * 10, max_bins, filter_k, true, 0)
    //     {
    //         error!("Error at {i}: {e}");
    //         error!("Error at {i}, {filter_k}: {e}");
    //     }
    // }
}
