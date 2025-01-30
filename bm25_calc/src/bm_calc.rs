use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::Duration;

use crate::error::Result;
use bm25::{
    DefaultTokenizer, Embedder, EmbedderBuilder, Language, Scorer, SearchEngine,
    SearchEngineBuilder, Tokenizer,
};
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use tracing::{debug, info, trace};

#[macro_export]
macro_rules! default_tokenizer {
    () => {
        DefaultTokenizer::builder()
            .language_mode(Language::English)
            .normalization(true)
            .stopwords(true)
            .stemming(true)
            .build()
    };
}

pub struct BM25Scorer {
    alphabet: HashSet<String>,
    scorer: Scorer<usize>,
    tokenizer: DefaultTokenizer,
}

pub fn get_alphabet(corpus: &Vec<String>) -> Result<HashSet<String>> {
    //let mut scorer = Scorer::<usize>::new();
    let mut set = HashSet::new();

    info!("Making alphabet");

    let tokenizer = DefaultTokenizer::builder()
        .language_mode(Language::English)
        .normalization(true)
        .stopwords(true)
        .stemming(true)
        .build();

    info!("scanning alphabet");
    debug!("Bar init");
    let bar = ProgressBar::new(corpus.len() as u64);
    for (i, document) in corpus.iter().enumerate() {
        bar.inc(1);
        let tokens = tokenizer.tokenize(&new_tokens);
        set.extend(tokens);
    }
    bar.finish();
    Ok(set)
}

pub fn build_search_engine(corpus: Vec<impl Into<String>>) -> Result<SearchEngine<u32>> {
    // TODO Flesh this out more

    Ok(SearchEngineBuilder::<u32>::with_corpus(Language::English, corpus).build())
}

pub fn top_k(
    k: usize,
    search_engine: &SearchEngine<u32>,
    alphabet: &HashSet<String>,
) -> HashMap<String, HashSet<u32>> {
    let mut results = HashMap::new();

    let bar = ProgressBar::new(alphabet.len() as u64);

    let mut counting_duplicates = HashMap::new();
    let mut num_items = 0;

    for word in alphabet {
        let search_results = search_engine.search(word, k);
        bar.inc(1);

        for result in search_results {
            results
                .entry(word.to_string())
                .or_insert_with(HashSet::new)
                .insert(result.document.id);
            num_items = num_items + 1;
            if counting_duplicates.contains_key(&result.document.id) {
                *counting_duplicates.get_mut(&result.document.id).unwrap() += 1;
            } else {
                counting_duplicates.insert(result.document.id, 0);
            }
            debug!(
                "id: {} item in bin: {:?}  ",
                result.document.id, counting_duplicates
            );
        }
    }

    info!(
        "Total number of duplicates: {}, total items in bins: {}",
        counting_duplicates.values().sum::<i32>(),
        num_items
    );

    bar.finish();

    results
}

fn get_hash(s: &str, n: &usize) -> u64 {
    let mut hasher = DefaultHasher::new();
    trace!("about to hash {} and {}", s, n);
    s.hash(&mut hasher);
    n.hash(&mut hasher);
    hasher.finish()
}

pub fn top_k_bins(
    k: usize,
    search_engine: &SearchEngine<u32>,
    alphabet: &HashSet<String>,
    d: usize,
    max_bins: usize,
) -> Vec<HashSet<u32>> {
    info!(
        "Starting top {} into {} bins with {} choice hashsing",
        k, max_bins, d
    );

    let mut results = vec![HashSet::new(); max_bins];
    let bar = ProgressBar::new(alphabet.len() as u64);

    for word in alphabet {
        let search_results = search_engine.search(word, k);
        let document_ids: HashSet<u32> = search_results
            .iter()
            .map(|result| result.document.id)
            .collect();

        // let mut rng = rand::thread_rng();

        //  while document_ids.len() < k {
        //      document_ids.insert(rng.gen_range(0..=5100) as u32);
        //  }
        let mut best_bin_index = 0;
        let mut max_overlap = 0;

        bar.inc(1);

        for choice in 0..d {
            let index: usize = (get_hash(word, &choice) % (max_bins as u64))
                .try_into()
                .unwrap();

            trace!("Got index {}", index);
            let overlap = results[index].intersection(&document_ids).count();

            if overlap > max_overlap || max_overlap == 0 {
                if overlap > 0 {
                    //debug!("Best bin updated {}", overlap);
                }
                max_overlap = overlap;
                best_bin_index = index;
            }
        }

        results[best_bin_index].extend(document_ids);
    }

    bar.finish();

    results
}

#[cfg(test)]
mod tests {
    use bm25::Tokenizer;

    use tracing::info;

    use super::*;

    use super::*;
    static CORPUS: [&str; 4] = [
        "The sky blushed pink as the sun dipped below the horizon.",
        "Apples, oranges, papayas, and more papayas.",
        "She found a forgotten letter tucked inside an old book.",
        "A single drop of rain fell, followed by a thousand more.",
    ];

    #[test]
    fn get_top_k() {
        tracing_subscriber::fmt()
            .with_test_writer()
            .with_max_level(tracing::Level::DEBUG)
            .try_init();

        let search = build_search_engine(CORPUS.iter().map(|&s| s.to_string()).collect()).unwrap();
        let alphabet = get_alphabet(&CORPUS.iter().map(|&s| s.to_string()).collect()).unwrap();
        let top_k = top_k(10, &search, &alphabet);
    }

    #[test]
    fn test_overlap() {
        let corpus_str: [&str; 4] = [
            "The sky blushed pink as the sun dipped below the horizon.",
            "The sky blushed pink as the sun dipped below the horizon.",
            "The sky blushed pink as the sun dipped below the horizon.",
            "The sky blushed pink as the sun dipped below the horizon.",
        ];

        let corpus = corpus_str.iter().map(|&s| s.to_string()).collect();

        tracing_subscriber::fmt()
            .with_test_writer()
            .with_max_level(tracing::Level::TRACE)
            .try_init();

        let d = 4;
        let k = 4;
        let max_bins = 4;

        info!("Testing overlap");
        let alphabet = get_alphabet(&corpus).unwrap();

        info!(
            "The total number of files is {} and the alphabet size is {}",
            corpus.len(),
            alphabet.len()
        );

        let search = build_search_engine(corpus).unwrap();
        let top_k_bins = top_k_bins(k, &search, &alphabet, d, max_bins);

        let bin_lengths = vec![0; max_bins];

        for i in 0..max_bins {
            let length = top_k_bins[i].len();
            debug!("Length is {}", length);
            assert!(length == 0 || length == 4);
        }
        panic!();
    }
}
