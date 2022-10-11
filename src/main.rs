use clap::Parser;

pub mod domain;

#[derive(Parser, Debug)]
#[command(name = "domainder", author = "Benedikt Grande", version = "0.1", about = "reminds you about any domain expiry", long_about = None)]
struct Args {
    #[arg(short, long)]
    domain: String,

    #[arg(short, long)]
    email: String,

    /// m => month(s), w => week(s), d => day(s)
    #[arg(short, long, value_name = "1m")]
    count: Option<String>,
}

fn main() {
    let args = Args::parse();

    println!("value: {}", args.domain)
}
