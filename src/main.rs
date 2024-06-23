use clap::Parser;
use scraper::{Html, Selector};
use serde_json::json;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(short, long)]
    short: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

async fn fetch_abstract(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&response);

    let selector = Selector::parse("div.abstract").unwrap();
    let abstract_div = document
        .select(&selector)
        .next()
        .ok_or("Abstract not found")?;

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

async fn summarize_abstract(abstract_text: &str) -> Result<String, Box<dyn std::error::Error>> {
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
        .ok_or_else(|| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Summary not found in response",
            )) as Box<dyn std::error::Error>
        })
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
