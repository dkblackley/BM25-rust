# BM25 Calculator

A Rust implementation for calculating BM25 scores across a keyword space using d-choice hashing. This tool processes JSON files and calculates BM25 scores for text entries.

## Prerequisites

- Rust (latest stable version)
- Cargo
- BM25 crate

## Installation

```bash
git clone https://github.com/dkblackley/BM25-rust.git
cd bm25_calc
cargo build --release
```

## Usage

```bash
cargo run -- -d <d-value> -k <k-value> --filter-k <filter-value> -f <file-path> --key <json-key>
```

### Parameters

- `-d, --d <VALUE>`: The number of choices to use in d-choice hashing (default: 10)
- `-k, --k <VALUE>`: K parameter for top-k results (default: 10)
- `--filter-k <VALUE>`: Filter K parameter. If a result returns less than this value it is discarded. For example, if set to 2 and a top-k value of 1 is returned, it is ignored. (default: 2)
- `-f, --file <PATH>`: Path to the JSONL file to read
- `--key <KEY>`: The key in the JSON which holds the text we want to search over (default: "text")

### Example

```bash
cargo run -- -d 10 -k 10 --filter-k 2 -f path/to/nyt_corpus.jsonl --key text
```

## Input Format

The input file should be a JSONL (JSON Lines) file where each line is a valid JSON object containing a text field. For example:

```json
{"text": "This is the first document"}
{"text": "This is another document"}
```


## License

MIT License
