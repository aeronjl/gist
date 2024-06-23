# gist - Abstract Retrieval and Summarization CLI

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

`gist` is a command-line interface (CLI) tool built in Rust that fetches and summarizes academic abstracts. It's designed primarily for use with PubMed abstracts and should be considered experimental for other sources.

## Features

- Fetch full abstracts from PubMed URLs
- Generate concise summaries of abstracts using gpt-3.5-turbo (powered by [val.town](https://www.val.town/v/aeronjl/gist))

## Installation

To install `gist`, you need to have Rust and Cargo installed on your system. If you don't have Rust installed, you can get it from [rustup.rs](https://rustup.rs/).

Once Rust is installed, clone this repository and build the project:

```bash
git clone https://github.com/yourusername/gist.git
cd gist
cargo build --release
```

The compiled binary will be available in `target/release/gist`.

## Usage

To use `gist`, run it from the command line with the following options:

```bash
gist --url <URL_OF_PUBMED_ABSTRACT> [--short]
```

- `--url` or `-u`: Specify the URL of the PubMed abstract you want to fetch.
- `--short` or `-s`: (Optional) Get a summarized version of the abstract.

Examples:

1. Fetch full abstract:
   ```bash
   gist --url https://pubmed.ncbi.nlm.nih.gov/12345678/
   ```

2. Fetch summarized abstract:
   ```bash
   gist --url https://pubmed.ncbi.nlm.nih.gov/12345678/ --short
   ```

## Limitations

- This tool is primarily designed for and tested with PubMed abstracts. It may not work correctly with abstracts from other sources.
- The summarization feature uses a val.town endpoint for demonstration and may be subject to rate limiting or service availability issues. In personal use, you can set up your own solution for text summarization with an LLM.
- The tool's effectiveness can vary depending on the structure and content of the abstract.

## Contributing

Contributions to improve `gist` are welcome! Please feel free to submit issues or pull requests on the GitHub repository.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Disclaimer

This tool is provided for research and educational purposes only. Always respect the terms of service of the websites you're accessing and be mindful of any copyright or usage restrictions on the abstracts you're fetching and summarizing.
