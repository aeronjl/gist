use clap::Parser;
use reqwest;
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
    let abstract_div = document.select(&selector).next().ok_or("Abstract not found")?;

    let raw_text = abstract_div.text().collect::<Vec<_>>().join(" ");
    
    let cleaned_text = clean_text(&raw_text);
    
    Ok(cleaned_text.trim_start_matches(|c: char| "abstract".contains(c.to_ascii_lowercase())).trim().to_string())
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
        .ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Summary not found in response")) as Box<dyn std::error::Error>)
        .map(|s| s.to_string())
}