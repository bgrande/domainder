use crate::domain::domain::write_domain;
use crate::reminder::remind::{hydrate_reminder, write_reminder};
use crate::send::remind::remind;
use clap::{Parser, Subcommand};
pub(crate) mod domain;
pub(crate) mod reminder;
pub(crate) mod send;

const DEFAULT_TIME: &str = "1m";

#[derive(Parser)]
#[command(name = "domainder", author = "Benedikt Grande", version = "0.1", about = "reminds you about any domain expiry", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(short, long)]
        domain: String,

        /// m => month(s), w => week(s), d => day(s)
        #[arg(short, long, value_name = "1m")]
        time: Option<String>,

        #[arg(short, long)]
        email: String,
    },
    Update {
        #[arg(short, long)]
        domain: String,

        /// m => month(s), w => week(s), d => day(s)
        #[arg(short, long, value_name = "1m")]
        time: Option<String>,

        #[arg(short, long)]
        email: String,
    },
    Remind {
        #[arg(short, long)]
        domain: Option<String>,
    },
}

fn get_value<'a>(time: &'a Option<String>, default: &'a str) -> &'a str {
    match time.as_deref() {
        Some(value) => value,
        None => default,
    }
}

async fn process_add(domain: &String, email: &String, time: &str) {
    // @todo move this logic into a separate module so it can be used from different endpoints (console, web, ...)
    let whois = domain::whois::whois_domain(&domain).await;
    let await_whois = whois.await;

    //println!("whois lookup: {:?}", result.await);

    write_domain(&await_whois).expect("could not write domain info");

    let reminder = hydrate_reminder(
        &domain,
        &time.to_string(),
        &await_whois.expiry.clone(),
        email,
    );
    write_reminder(&reminder).expect("could not write reminder");
}

fn process_update(domain: &String, email: &String, time: &str) {
    // todo finish me
}

fn process_remind(domain: &Option<String>) {
    // todo implement only updating specified domain
    remind();
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Add {
                 domain,
                 time,
                 email,
             }) => {
            let time_string = get_value(&time, DEFAULT_TIME);
            process_add(domain, email, time_string).await;
        }
        Some(Commands::Update {
                 domain,
                 time,
                 email,
             }) => {
            let time_string = get_value(&time, DEFAULT_TIME);
            process_update(domain, email, time_string);
        }
        Some(Commands::Remind { domain }) => {
            process_remind(domain);
        }
        None => {
            print!("I really don't know what to do!");
            print!("Please use '--help' to get more command information!")
        }
    }
    /* @todo:
       + convert to json (via filling struct with the data we need?)
       + write required data into domains/domain_name.json (name, registry, registration_date, expiration_date)
       + create reminder in reminder/[reminder_date].json
       + send email to remind about domains
       + send emails conditionally (by time)
       5. cleanup domains that don't exist anymore
       6. (auto) update the domains dates when marked as reminded
       7. only remind specified domain
       8. add readme on how to use and create cronjob
    */
}
