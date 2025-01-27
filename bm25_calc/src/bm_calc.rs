use bm25::{Embedder, EmbedderBuilder, Embedding, Language, TokenEmbedding};

fn pre_calc_bm() {
    let corpus = [
        "The sky blushed pink as the sun dipped below the horizon.",
        "Apples, oranges, papayas, and more papayas.",
        "She found a forgotten letter tucked inside an old book.",
        "A single drop of rain fell, followed by a thousand more.",
    ];

    let embedder: Embedder =
        EmbedderBuilder::with_fit_to_corpus(Language::English, &corpus).build();
}
