use clap::Parser;
use env_logger::{Builder, Env};
use log::{debug, error, info, warn};
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
    info!("URL: {}, Short mode: {}", args.url, args.short);

    match fetch_abstract(&args.url).await {
        Ok(abstract_text) => {
            info!(
                "Successfully fetched abstract, length: {} characters",
                abstract_text.len()
            );
            if args.short {
                debug!("Summarizing abstract");
                match summarize_abstract(&abstract_text).await {
                    Ok(summary) => {
                        info!(
                            "Successfully generated summary, length: {} characters",
                            summary.len()
                        );
                        println!("Summary:");
                        println!("{}", summary);
                    }
                    Err(e) => {
                        error!("Failed to summarize abstract: {}", e);
                        return Err(e);
                    }
                }
            } else {
                info!("Displaying full abstract");
                println!("Full Abstract:");
                println!("{}", abstract_text);
            }
        }
        Err(e) => {
            error!("Failed to fetch abstract: {}", e);
            return Err(e);
        }
    }

    info!("Application completed successfully");
    Ok(())
}

async fn fetch_abstract(url: &str) -> Result<String> {
    debug!("Fetching abstract from URL: {}", url);

    let response = match reqwest::get(url).await {
        Ok(resp) => {
            info!("Successfully connected to URL");
            resp
        }
        Err(e) => {
            error!("Failed to connect to URL {}: {}", url, e);
            return Err(GistError::FetchError(e));
        }
    };

    let text = match response.text().await {
        Ok(t) => {
            debug!(
                "Successfully retrieved response text, length: {} characters",
                t.len()
            );
            t
        }
        Err(e) => {
            error!("Failed to get response text from {}: {}", url, e);
            return Err(GistError::FetchError(e));
        }
    };

    let document = Html::parse_document(&text);
    debug!("Parsed HTML document");

    let selector = match Selector::parse("div.abstract") {
        Ok(s) => s,
        Err(_) => {
            error!("Failed to parse HTML selector for abstract");
            return Err(GistError::ParseError(
                "Failed to parse HTML selector".to_string(),
            ));
        }
    };

    let abstract_div = match document.select(&selector).next() {
        Some(div) => div,
        None => {
            warn!("Abstract not found in the document");
            return Err(GistError::AbstractNotFound);
        }
    };

    let raw_text = abstract_div.text().collect::<Vec<_>>().join(" ");
    debug!(
        "Extracted raw text from abstract, length: {} characters",
        raw_text.len()
    );

    let cleaned_text = clean_text(&raw_text);
    debug!(
        "Cleaned abstract text, length: {} characters",
        cleaned_text.len()
    );

    info!("Successfully fetched and parsed abstract from {}", url);
    Ok(cleaned_text
        .trim_start_matches(|c: char| "abstract".contains(c.to_ascii_lowercase()))
        .trim()
        .to_string())
}

fn clean_text(text: &str) -> String {
    debug!("Cleaning text of length: {} characters", text.len());
    let cleaned = text.split_whitespace().collect::<Vec<_>>().join(" ");
    debug!("Text cleaned, new length: {} characters", cleaned.len());
    cleaned
}

async fn summarize_abstract(abstract_text: &str) -> Result<String> {
    debug!(
        "Summarizing abstract of length: {} characters",
        abstract_text.len()
    );
    let client = reqwest::Client::new();

    let response = match client
        .post("https://aeronjl-gist.web.val.run")
        .json(&json!({
            "text": abstract_text
        }))
        .send()
        .await
    {
        Ok(resp) => {
            info!("Successfully sent request to summarization API");
            resp
        }
        Err(e) => {
            error!("Failed to send request to summarization API: {}", e);
            return Err(GistError::FetchError(e));
        }
    };

    let response_json: serde_json::Value = match response.json().await {
        Ok(json) => json,
        Err(e) => {
            error!(
                "Failed to parse JSON response from summarization API: {}",
                e
            );
            return Err(GistError::FetchError(e));
        }
    };

    match response_json["summary"].as_str() {
        Some(summary) => {
            info!(
                "Successfully generated summary, length: {} characters",
                summary.len()
            );
            Ok(summary.to_string())
        }
        None => {
            warn!("Summary not found in API response");
            Err(GistError::SummaryNotFound)
        }
    }
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
