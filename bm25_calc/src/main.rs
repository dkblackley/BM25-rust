pub(crate) mod bm_calc;
pub(crate) mod dataloader;
pub(crate) mod error;

use tracing::{debug, info};

fn main() {
    tracing_subscriber::fmt()
        .with_test_writer()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let d = 4;
    let k = 10;
    let max_bins = 1500;

    info!("Starting BM25 calculation");
    // TODO replace with cmd args
    //let corpus = dataloader::return_data_as_string("../scifact/corpus.jsonl").unwrap();
    let corpus = dataloader::return_data_as_string("../arxiv-metadata-oai-snapshot.json").unwrap();

    let alphabet = bm_calc::get_alphabet(&corpus).unwrap();

    info!(
        "The total number of files is {} and the alphabet size is {}",
        corpus.len(),
        alphabet.len()
    );
    let items: Vec<_> = alphabet.iter().collect();

    debug!("len: {}", items.len());

    for i in 0..1000 {
        debug!("{}", &items[i]);
    }

    let search = bm_calc::build_search_engine(corpus).unwrap();

    let top_k = bm_calc::top_k(k, &search, &alphabet);
    info!("Top K Done");
    let top_k_bins = bm_calc::top_k_bins(k, &search, &alphabet, d, max_bins);
    info!("Top k into bin done");

    let bin_lengths = vec![0; max_bins];
    let mut largest = k;
    let mut total_items: usize = 0;

    for i in 0..max_bins {
        let length = top_k_bins[i].len();
        total_items = total_items + length;
        //debug!("Length is {}", length);
        if length > largest {
            largest = length;
        }
    }

    let mut total_length = 0;

    for results in top_k.values() {
        total_length += results.len();
        //debug!("indices in the bins: {:?}", results);
    }

    let duplicates = 0;

    // Is this correct?
    // for i in 0..5200 {
    //     for results in top_k.values() {
    //         for index in results {
    //             if i == *index {
    //                 duplicates += 1;
    //             }
    //         }
    //     }
    // }

    info!("In the non-colliding version there are a total of  {} bins. (total of {} items across all bins). duplicates counted: {}", top_k.len(), total_length, duplicates);
    info!(
        "In the {} choice version with {} bins, there was {} items distributed across the buckets",
        d, max_bins, total_items
    );
    info!("Largest bin was {}", largest);
    info!(
        "We saved roughly {}% by using {} choice",
        duplicates / total_items,
        d
    );
}
