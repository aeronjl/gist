use clap::Parser;
use reqwest;
use scraper::{Html, Selector};

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
    
    // First, clean the whitespace
    let cleaned_text = clean_text(&raw_text);
    
    // Then, remove the word "Abstract" if it's at the beginning
    Ok(cleaned_text.trim_start_matches(|c: char| "abstract".contains(c.to_ascii_lowercase())).trim().to_string())
}

fn clean_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

async fn summarize_abstract(abstract_text: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(abstract_text.chars().take(100).collect::<String>() + "...")
}