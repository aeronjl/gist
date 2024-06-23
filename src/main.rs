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
    let clean_abstract = clean_text(&abstract_text);

    if args.short {
        let summary = summarize_abstract(&clean_abstract).await?;
        println!("Summary:");
        println!("{}", summary);
    } else {
        println!("Full Abstract:");
        println!("{}", clean_abstract);
    }

    Ok(())
}

async fn fetch_abstract(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&response);

    let selector = Selector::parse("div.abstract").unwrap();
    let abstract_div = document.select(&selector).next().ok_or("Abstract not found")?;

    let raw_text = abstract_div.text().collect::<Vec<_>>().join(" ");
    
    // Remove the word "Abstract" from the beginning if it exists
    Ok(raw_text.trim_start_matches("Abstract").trim().to_string())
}

fn clean_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

async fn summarize_abstract(abstract_text: &str) -> Result<String, Box<dyn std::error::Error>> {
    // This is a placeholder for the API call.
    // In a real implementation, you would send the abstract to an API and get the summary.
    // For demonstration, we'll just return a shortened version of the abstract.
    Ok(abstract_text.chars().take(100).collect::<String>() + "...")
}