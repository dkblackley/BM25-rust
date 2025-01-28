use std::collections::HashSet;
use std::hash::Hash;

use crate::error::Result;
use bm25::{
    DefaultTokenizer, Embedder, EmbedderBuilder, Embedding, Language, Scorer, SearchEngineBuilder,
    TokenEmbedding, Tokenizer,
};

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

fn pre_calc_bm(corpus: &[&str]) -> Result<BM25Scorer> {
    let mut scorer = Scorer::<usize>::new();
    let mut set = HashSet::new();

    let tokenizer = DefaultTokenizer::builder()
        .language_mode(Language::English)
        .normalization(true)
        .stopwords(true)
        .stemming(true)
        .build();

    let embedder: Embedder<DefaultTokenizer> =
        EmbedderBuilder::with_tokenizer_and_fit_to_corpus(tokenizer, corpus).build();

    let tokenizer = DefaultTokenizer::builder()
        .language_mode(Language::English)
        .normalization(true)
        .stopwords(true)
        .stemming(true)
        .build();

    for (i, document) in corpus.iter().enumerate() {
        let document_embedding = embedder.embed(document);
        scorer.upsert(&i, document_embedding);

        let tokens = tokenizer.tokenize(&document);
        set.extend(tokens);
    }

    Ok(BM25Scorer {
        alphabet: set,
        scorer,
        tokenizer,
    })
}

fn build_search_engine() {
    
}

fn perform_query(k: usize, query: &str, corpus: &[&str], bm25_scorer: &BM25Scorer) {
    let search_engine = SearchEngineBuilder::<usize>::with_tokenizer_and_corpus(tokenizer, corpus)

    let limit = k;
    let search_results = search_engine.search(query, limit);

    assert_eq!(
        search_results,
        vec![
            SearchResult {
                document: Document {
                    id: 2,
                    contents: String::from("The hedgehog impaled the orange orange."),
                },
                score: 0.4904281,
            },
            SearchResult {
                document: Document {
                    id: 0,
                    contents: String::from("The rabbit munched the orange carrot."),
                },
                score: 0.35667497,
            },
        ]
    );
}

/// Takes in an iterable and then returns a hashset mapping the items to their top-k score
fn top_k<T, I>(k: usize, iter: I) -> HashSet<T>
where
    T: Hash + Eq,
    I: IntoIterator<Item = T>,
{
    let mut result = HashSet::new();
    // Implementation here
    result
}

/// Takes in an iterable and then returns a vector of items in bins, uses d-choice hashing
fn top_k_bins<T, I>(k: usize, bins: usize, choices: usize, iter: I) -> Vec<Vec<T>>
where
    T: Hash + Eq,
    I: IntoIterator<Item = T>,
{
    let mut result = vec![bins];

    // use these as seeds
    let numbers: Vec<i32> = (0..129).collect();

    // Implementation here
    result
}

#[cfg(test)]
mod tests {
    use bm25::Tokenizer;

    use tracing::info;

    use super::*;

    static CORPUS: [&str; 4] = [
        "The sky blushed pink as the sun dipped below the horizon.",
        "Apples, oranges, papayas, and more papayas.",
        "She found a forgotten letter tucked inside an old book.",
        "A single drop of rain fell, followed by a thousand more.",
    ];

    #[test]
    fn extract_embeddings() {
        // Initialize subscriber for tests
        tracing_subscriber::fmt()
            .with_test_writer() // This ensures proper test output formatting
            .with_max_level(tracing::Level::DEBUG)
            .init();

        let bm_score = pre_calc_bm(&CORPUS).unwrap();

        for word in bm_score.alphabet {
            info!("{word}");
        }

        panic!();
    }
}
