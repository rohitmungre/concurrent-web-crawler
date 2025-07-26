mod cli;
mod crawler;
mod parser;

use anyhow::Result;
use cli::Opts;
use crawler::Crawler;
use tokio::runtime::Builder;
use url::Url;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    let start_url = Url::parse(&opts.url)?;
    let mut crawler = Crawler::new(start_url, /*concurrency=*/20);
    crawler.run(opts.limit).await?;
    Ok(())
}
