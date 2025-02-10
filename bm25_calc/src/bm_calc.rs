use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::error::Result;
use bm25::{DefaultTokenizer, Language, SearchEngine, SearchEngineBuilder, Tokenizer};
use indicatif::ProgressBar;
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

/// Gets the "alphabet" or the entire set of possible keywords. Returns a hashset of the keywords
///
/// # Arguments
/// * `corpus` - Vector of documents  (as strings)to tokenize
///
/// # Returns
/// * `Result<HashSet<String>>` - Set of unique tokens
#[allow(clippy::ptr_arg)] // allow this for test cases
pub fn get_alphabet(corpus: &Vec<String>) -> Result<HashSet<String>> {
    let mut set = HashSet::new();
    info!("Making alphabet");

    let tokenizer = default_tokenizer!();

    info!("scanning alphabet");
    debug!("Bar init");
    let bar = ProgressBar::new(corpus.len() as u64);
    for document in corpus.iter() {
        bar.inc(1);
        let tokens = tokenizer.tokenize(document);
        set.extend(tokens);
    }
    bar.finish();
    Ok(set)
}

/// Builds a search engine from a corpus of documents (See the BM25 crate girhubpage/documentation)
///
/// # Arguments
/// * `corpus` - Collection of documents that can be converted to Strings. Usually just passed in as a string.
///
/// # Returns
/// * `SearchEngine<u32>` - Search engine initialized and ready to search through the entire corpus
pub fn build_search_engine(corpus: Vec<impl Into<String>>) -> SearchEngine<u32> {
    SearchEngineBuilder::<u32>::with_corpus(Language::English, corpus).build()
}

/// Performs top-k search for each word in the alphabet and filters results. Doesn't do any choice hashing or anything speical, just returns top-k. Theoretic return size is O(k * alphabet), i.e. each bin has 10 full results in each bin
///
/// # Arguments
/// * `k` - Number of results to retrieve per word. the k in top-k
/// * `search_engine` - Search engine to query (See Rust BM25 crate)
/// * `alphabet` - The keyword space
/// * `filter_k` - Minimum number of results required to keep a word. I.e. if this is 2, then allr esults with a top-k of only 1 while be discarded
///
/// # Returns
/// * `HashMap<String, HashSet<u32>>` - Map of words to sets of matching document IDs. The ID matches the index in the corpus array (See BM25 crate)
#[allow(clippy::map_entry)] // allow this because debugging is easier when using insert
pub fn top_k(
    k: usize,
    search_engine: &SearchEngine<u32>,
    alphabet: &HashSet<String>,
    filter_k: usize,
) -> HashMap<String, HashSet<u32>> {
    let mut results = HashMap::new();

    let bar = ProgressBar::new(alphabet.len() as u64);

    let mut counting_duplicates = HashMap::new();
    let mut num_items = 0;

    for word in alphabet {
        let search_results = search_engine.search(word, k);
        bar.inc(1);
        if search_results.len() < filter_k {
            // filter out low results
            continue;
        }

        for result in search_results {
            results
                .entry(word.to_string())
                .or_insert_with(HashSet::new)
                .insert(result.document.id);
            num_items += 1; // increment the total number of items in bins for logging
            if counting_duplicates.contains_key(&result.document.id) {
                // if this item was already previously inserted, count it as a duplicate
                *counting_duplicates.get_mut(&result.document.id).unwrap() += 1;
            } else {
                // if this is the first time we're seeing this document ID, insert it as a new item
                counting_duplicates.insert(result.document.id, 0);
            }
        }
    }

    bar.finish();

    info!(
        "Top-K done without d-choice. Total number of duplicates: {}, total items in bins: {}",
        counting_duplicates.values().sum::<i32>(),
        num_items
    );

    info!(
        "The average number of items in bins is {}",
        results.values().map(|set| set.len()).sum::<usize>() as f64 / results.len() as f64
    );

    results
}

/// Deterministic function that can generate a hash value from a string and number
///
/// # Arguments
/// * `s` - String to hash
/// * `n` - Number to combine with string hash
///
/// # Returns
/// * `u64` - Combined hash value
fn get_hash(s: &str, n: &usize) -> u64 {
    let mut hasher = DefaultHasher::new();
    trace!("about to hash {} and {}", s, n);
    s.hash(&mut hasher);
    n.hash(&mut hasher);
    hasher.finish()
}

/// Performs top-k search with d-choice hashing into multiple bins. Function is deterministic and should reveal the same results over each run.
///
/// # Arguments
/// * `k` - Number of results to retrieve per word. the k in top-k
/// * `search_engine` - Search engine to query
/// * `alphabet` - The keyword space
/// * `d` - Number of hash choices per word
/// * `max_bins` - Number of bins to distribute results into
/// * `filter_k` - Minimum number of results required to keep a word
///
/// # Returns
/// * `Vec<HashSet<u32>>` - Vector of bins containing document IDs
///
/// # Notes
/// Uses d-choice hashing to minimize collisions. For each word,
/// tries d different hash functions and places results in bin
/// with maximum overlap.
pub fn top_k_bins(
    k: usize,
    search_engine: &SearchEngine<u32>,
    alphabet: &HashSet<String>,
    d: usize,
    max_bins: usize,
    filter_k: usize,
    max_load: bool,
    max_load_factor: usize,
) -> Result<Vec<HashSet<u32>>> {
    info!(
        "Starting top {} into {} bins with {} choice hashing",
        k, max_bins, d
    );

    let mut results = vec![HashSet::new(); max_bins];
    let bar = ProgressBar::new(alphabet.len() as u64);
    let mut total_overlap = 0;

    for word in alphabet {
        let search_results = search_engine.search(word, k);

        // Skip words with too few results
        if search_results.len() < filter_k {
            bar.inc(1);
            continue;
        }

        // Convert search results to document IDs
        let document_ids: HashSet<u32> = search_results
            .iter()
            .map(|result| result.document.id)
            .collect();

        let mut best_bin_index = 0;
        let mut max_overlap = 0;
        let mut bin_choices = Vec::with_capacity(d);

        // Try d different hash functions
        for choice in 0..d {
            let index: usize = (get_hash(word, &choice) % (max_bins as u64)).try_into()?;

            let overlap = results[index].intersection(&document_ids).count();
            let bin_size = results[index].len();

            trace!(
                "Got index {}, overlap: {}, k: {}, bin size: {}",
                index,
                overlap,
                search_results.len(),
                bin_size
            );

            //TODO I think I need to remove this if statement and mess with when the load factor check occurs
            if max_load {
                // Collect information about all bin choices
                bin_choices.push((index, bin_size, overlap));
            } else {
                // For non-max_load case, simply track the bin with maximum overlap
                if overlap > max_overlap || max_overlap == 0 {
                    max_overlap = overlap;
                    best_bin_index = index;
                }
            }
        }

        // Process max_load case after collecting all choices
        if max_load && !bin_choices.is_empty() {
            // Sort bins by size in descending order (fullest first)
            bin_choices.sort_by(|a, b| b.1.cmp(&a.1));

            // Reset max_overlap since we're now looking at a subset
            max_overlap = 0;

            // Skip the max_load_factor fullest bins and find max overlap among remaining
            for &(idx, _, curr_overlap) in bin_choices.iter().skip(max_load_factor) {
                if curr_overlap > max_overlap {
                    max_overlap = curr_overlap;
                    best_bin_index = idx;
                }
            }

            // If all bins were skipped (max_load_factor >= d), use the last bin
            if max_load_factor >= bin_choices.len() {
                let last_choice = bin_choices.last().unwrap();
                best_bin_index = last_choice.0;
                max_overlap = last_choice.2;
            }
        }

        // Add to running total of overlaps saved
        total_overlap += max_overlap;

        // Add document IDs to the selected bin
        results[best_bin_index].extend(document_ids);

        bar.inc(1);
    }

    bar.finish();

    info!(
        "top {} into {} bins with {} choice hashing has finished. We saved roughly {} duplicates",
        k, max_bins, d, total_overlap
    );

    info!(
        "The average number of items in bins is {}",
        results.iter().map(|set| set.len()).sum::<usize>() as f64 / results.len() as f64
    );

    Ok(results)
}

#[cfg(test)]
mod tests {

    use tracing::info;

    use super::*;
    static CORPUS: [&str; 4] = [
        "The sky blushed pink as the sun dipped below the horizon.",
        "Apples, oranges, papayas, and more papayas.",
        "She found a forgotten letter tucked inside an old book.",
        "A single drop of rain fell, followed by a thousand more.",
    ];

    #[test]
    fn get_top_k() {
        let _ = tracing_subscriber::fmt()
            .with_test_writer()
            .with_max_level(tracing::Level::DEBUG)
            .try_init();

        let search = build_search_engine(CORPUS.iter().map(|&s| s.to_string()).collect());
        let alphabet = get_alphabet(&CORPUS.iter().map(|&s| s.to_string()).collect()).unwrap();
        let _top_k = top_k(10, &search, &alphabet, 4);
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

        let _ = tracing_subscriber::fmt()
            .with_test_writer()
            .with_max_level(tracing::Level::TRACE)
            .try_init();

        let d = 10;
        let k = 4;
        let max_bins = 4;

        info!("Testing overlap");
        let alphabet = get_alphabet(&corpus).unwrap();

        info!(
            "The total number of files is {} and the alphabet size is {}",
            corpus.len(),
            alphabet.len()
        );

        let search = build_search_engine(corpus);
        let top_k_bins = top_k_bins(k, &search, &alphabet, d, max_bins, 4).unwrap();

        (0..max_bins).for_each(|i| {
            let length = top_k_bins[i].len();
            debug!("Length is {}", length);
            assert!(length == 0 || length == 4);
        });
    }
}
