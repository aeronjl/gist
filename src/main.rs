use clap::Parser;
use reqwest;
use scraper::{Html, Selector};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let abstract_text = fetch_abstract(&args.url).await?;

    if args.verbose {
        println!("Full Abstract:");
        println!("{}", abstract_text);
    } else {
        let summary = summarize_abstract(&abstract_text).await?;
        println!("Summary:");
        println!("{}", summary);
    }

    Ok(())
}

async fn fetch_abstract(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&response);

    let selector = Selector::parse("div.abstract").unwrap();
    let abstract_div = document.select(&selector).next().ok_or("Abstract not found")?;

    Ok(abstract_div.text().collect::<Vec<_>>().join(" "))
}

async fn summarize_abstract(abstract_text: &str) -> Result<String, Box<dyn std::error::Error>> {
    // This is a placeholder for the API call.
    // In a real implementation, you would send the abstract to an API and get the summary.
    // For demonstration, we'll just return a shortened version of the abstract.
    Ok(abstract_text.chars().take(100).collect::<String>() + "...")
}