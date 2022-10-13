use std::io::Read;
use clap::Parser;

pub(crate) mod domain;

#[derive(Parser, Debug)]
#[command(name = "domainder", author = "Benedikt Grande", version = "0.1", about = "reminds you about any domain expiry", long_about = None)]
struct Args {
    #[arg(short, long)]
    domain: String,

    #[arg(short, long)]
    email: String,

    /// m => month(s), w => week(s), d => day(s)
    #[arg(short, long, value_name = "1m")]
    time: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("value: {}", args.domain);

    let result = domain::whois::whois_domain(&args.domain).await;

    println!("whois lookup: {:?}", result.await);
}
