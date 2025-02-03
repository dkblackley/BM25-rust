//! main.rs - the main entrypoint into the calculator.

/// bm_calc.rs - crate responsible for calculating top-k and BM25 searching.
pub(crate) mod bm_calc;
/// Crate that loads in data and puts it into a vector. Useful for the format the BM25 crate expects it.
pub(crate) mod dataloader;
/// error.rs - this holds a single enum that we can put our errors into.
pub(crate) mod error;

use clap::{arg, command, Parser};
use tracing::{error, info};

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
    #[arg(short, long, default_value = "text")]
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
    // TODO replace with cmd args
    let corpus = dataloader::return_data_as_string(&args.file, &args.key).unwrap();

    let max_bins = corpus.len() / 100;

    let alphabet = bm_calc::get_alphabet(&corpus).unwrap();

    info!(
        "The total number of files is {} and the alphabet size is {}",
        corpus.len(),
        alphabet.len()
    );

    let search = bm_calc::build_search_engine(corpus);

    bm_calc::top_k(k, &search, &alphabet, filter_k);
    info!("Top K Done");

    for i in 0..d {
        if let Err(e) = bm_calc::top_k_bins(k, &search, &alphabet, i, max_bins, filter_k) {
            error!("Error at {i}, {filter_k}: {e}");
        }
    }
}
