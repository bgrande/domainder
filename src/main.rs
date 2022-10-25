use clap::Parser;
use crate::domain::domain::write_domain;
use crate::reminder::remind::{hydrate_reminder, write_reminder};

pub(crate) mod domain;
pub(crate) mod reminder;

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
    println!("email: {}", args.email);

    let time = match args.time.as_deref() {
        Some(t) => t,
        None => "1m"
    };

    println!("time: {}", time);

    // @todo move this logic into a separate module so it can be used from different endpoints (console, web, ...)
    let whois = domain::whois::whois_domain(&args.domain).await;
    let await_whois = whois.await;

    //println!("whois lookup: {:?}", result.await);

    write_domain(&await_whois).expect("could not write domain info");

    /* @todo:
      + convert to json (via filling struct with the data we need?)
      + write required data into domains/domain_name.json (name, registry, registration_date, expiration_date)
      3. create reminder in reminder/[reminder_date].json
   */
    let reminder = hydrate_reminder(&args.domain, time.to_string(), &await_whois.expiry.clone());
    write_reminder(reminder).expect("could not write reminder");

}
