pub(crate) mod bm_calc;
pub(crate) mod dataloader;
pub(crate) mod error;

use tracing::{debug, info};

fn main() {
    tracing_subscriber::fmt()
        .with_test_writer()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let d = 1;
    let k = 10;
    let filter_k = 2;

    info!("Starting BM25 calculation");
    // TODO replace with cmd args
    //let corpus = dataloader::return_data_as_string("../scifact/corpus.jsonl").unwrap();
    //let corpus = dataloader::return_data_as_string("../arxiv-metadata-oai-snapshot.json").unwrap();
    let corpus = dataloader::return_data_as_string("../nyt_processed_regex.jsonl").unwrap();

    let max_bins = corpus.len() / 100;

    let alphabet = bm_calc::get_alphabet(&corpus).unwrap();

    info!(
        "The total number of files is {} and the alphabet size is {}",
        corpus.len(),
        alphabet.len()
    );
    let items: Vec<_> = alphabet.iter().collect();

    debug!("len: {}", items.len());

    let search = bm_calc::build_search_engine(corpus).unwrap();

    let top_k = bm_calc::top_k(k, &search, &alphabet, filter_k);
    info!("Top K Done");
    let top_k_bins = bm_calc::top_k_bins(k, &search, &alphabet, d, max_bins, filter_k);
    let top_k_bins = bm_calc::top_k_bins(k, &search, &alphabet, d * 2, max_bins, filter_k);
    let top_k_bins = bm_calc::top_k_bins(k, &search, &alphabet, d * 2 * 2, max_bins, filter_k);
}
