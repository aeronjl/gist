use clap::Parser;
use scraper::{Html, Selector};
use serde_json::json;
use thiserror::Error;

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

// Define a Result type alias for convenience
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

    Ok(cleaned_text
        .trim_start_matches(|c: char| "abstract".contains(c.to_ascii_lowercase()))
        .trim()
        .to_string())
}

fn clean_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

async fn summarize_abstract(abstract_text: &str) -> Result<String> {
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

// ... rest of your code (tests, etc.) ...