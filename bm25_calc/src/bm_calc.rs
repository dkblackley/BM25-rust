use std::collections::HashSet;

use crate::error::Result;
use bm25::{
    DefaultTokenizer, Embedder, EmbedderBuilder, Embedding, Language, Scorer, TokenEmbedding,
    Tokenizer,
};

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

    let embedder: Embedder =
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

fn top_k(k: usize) {}

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
