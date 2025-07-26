use clap::Parser;

#[derive(Parser)]
#[command(name = "crawler")]
pub struct Opts {
    /// Starting URL
    #[arg(short, long)]
    pub url: String,

    /// Max pages to fetch
    #[arg(short, long, default_value_t = 100)]
    pub limit: usize,
}
