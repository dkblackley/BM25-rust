use crate::error::Result;
use bm25::{
    DefaultTokenizer, Embedder, EmbedderBuilder, Embedding, Language, Scorer, TokenEmbedding,
    Tokenizer,
};

pub struct BM25Scorer {}

fn pre_calc_bm(corpus: &[&str]) -> Result<BM25Scorer> {
    let mut scorer = Scorer::<usize>::new();
    let embedder: Embedder = EmbedderBuilder::with_fit_to_corpus(Language::English, corpus).build();

    for (i, document) in corpus.iter().enumerate() {
        let document_embedding = embedder.embed(document);
        scorer.upsert(&i, document_embedding);
    }
}

fn map_words_to_scores(alphabet: Vec<&str>, tokenizer: Tokenizer) {}

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
        let mut scorer = Scorer::<usize>::new();

        let tokenizer = DefaultTokenizer::builder()
            .language_mode(Language::English)
            .normalization(true) // Normalize unicode (e.g., 'é' -> 'e', '🍕' -> 'pizza', etc.)
            .stopwords(true) // Remove common words with little meaning (e.g., 'the', 'and', 'of', etc.)
            .stemming(true) // Reduce words to their root form (e.g., 'running' -> 'run')
            .build();

        let test_alphabet = "one two three Evgenios";
        let tokens = tokenizer.tokenize(test_alphabet);

        info!("{:?}", tokens);
    }
}
