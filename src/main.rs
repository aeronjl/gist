use clap::Parser;
use scraper::{Html, Selector};
use serde_json::json;
use thiserror::Error;
use log::{info, warn, error, debug};
use env_logger::{Env, Builder};

#[derive(Error, Debug)]
pub enum GistError {
    #[error("Failed to fetch abstract: {0}")]
    FetchError(#[from] reqwest::Error),

    #[error("Abstract not found")]
    AbstractNotFound,

    #[error("Failed to parse HTML: {0}")]
    ParseError(String),

    #[error("Summary not found in response")]
    SummaryNotFound,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, GistError>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(short, long)]
    short: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Application started");
    debug!("Parsing command line arguments");
    let args = Args::parse();

    let abstract_text = fetch_abstract(&args.url).await?;

    if args.short {
        let summary = summarize_abstract(&abstract_text).await?;
        println!("Summary:");
        println!("{}", summary);
    } else {
        println!("Full Abstract:");
        println!("{}", abstract_text);
    }

    Ok(())
}

async fn fetch_abstract(url: &str) -> Result<String> {
    debug!("Fetching abstract from URL: {}", url);

    let response = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&response);

    let selector = Selector::parse("div.abstract")
        .map_err(|_| GistError::ParseError("Failed to parse HTML selector".to_string()))?;
    let abstract_div = document
        .select(&selector)
        .next()
        .ok_or(GistError::AbstractNotFound)?;

    let raw_text = abstract_div.text().collect::<Vec<_>>().join(" ");

    let cleaned_text = clean_text(&raw_text);

    info!("Successfully fetched and parsed abstract from {}", url);
    Ok(cleaned_text
        .trim_start_matches(|c: char| "abstract".contains(c.to_ascii_lowercase()))
        .trim()
        .to_string())
}

fn clean_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

async fn summarize_abstract(abstract_text: &str) -> Result<String> {
    debug!("Summarizing abstract of length: {}", abstract_text.len());
    let client = reqwest::Client::new();
    let response = client
        .post("https://aeronjl-gist.web.val.run")
        .json(&json!({
            "text": abstract_text
        }))
        .send()
        .await?;

    let response_json: serde_json::Value = response.json().await?;

    response_json["summary"]
        .as_str()
        .ok_or(GistError::SummaryNotFound)
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_text() {
        let input = "This   is  a   test   string  with   extra   spaces";
        let expected = "This is a test string with extra spaces";
        assert_eq!(clean_text(input), expected);
    }

    #[tokio::test]
    async fn test_fetch_abstract() {
        // This test requires network access and might be flaky
        let url = "https://pubmed.ncbi.nlm.nih.gov/21150120/";
        let result = fetch_abstract(url).await;
        assert!(result.is_ok());
        let abstract_text = result.unwrap();
        assert!(abstract_text.contains("peroxisome proliferator-activated receptor signaling"));
    }

    #[tokio::test]
    async fn test_summarize_abstract() {
        let abstract_text =
            "This is a test abstract. It contains multiple sentences. The content is not real.";
        let result = summarize_abstract(abstract_text).await;
        assert!(result.is_ok());
        let summary = result.unwrap();
        assert!(!summary.is_empty());
        assert!(summary.len() < abstract_text.len());
    }
}