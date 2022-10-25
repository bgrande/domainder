use std::future::{Future, ready};
use super::config;
use whois_rust::{WhoIs, WhoIsLookupOptions};
use crate::domain::config::get_server_file_path;
use super::result::{WhoisResult};

fn hydrate_whois(lookup: String, domain: &str) -> WhoisResult {
    let split_by_newline = lookup.split("\n");
    
    let mut whois = WhoisResult {
        domain: domain.to_string(),
        server: "".to_string(),
        updated: "".to_string(),
        expiry: "auto".to_string(),
        created: "".to_string()
    };

    for newline_split in split_by_newline {
        let trimmed = String::from(newline_split).trim().to_string();
        if !trimmed.contains(":") {
            continue;
        }

        let values: Vec<&str> = trimmed.split(":").collect();

        if String::from(values[0]).contains("Server") {
            whois.server = String::from(values[1]);
        }

        if String::from(values[0]).contains("Updated") || String::from(values[0]).contains("Changed") {
            whois.updated = String::from(values[1]) + ":" + values[2] + ":" + values[3];
        }

        if String::from(values[0]).contains("Creation") {
            whois.created = String::from(values[1]) + ":" + values[2] + ":" + values[3];
        }

        if String::from(values[0]).contains("Expiry") {
            whois.expiry = String::from(values[1]) + ":" + values[2] + ":" + values[3];
        }
    }

    whois
}

pub(crate) async fn whois_domain(domain: &String) ->  impl Future<Output=WhoisResult> {
    if !config::get_server_file().await {
        // @todo return with error
    }

    let whois = WhoIs::from_path(get_server_file_path()).unwrap();

    let lookup = whois.lookup(WhoIsLookupOptions::from_string(domain).unwrap()).unwrap().to_string();

    if lookup.contains("Status: free") || lookup.contains("No match for domain") {
        println!("domain {} does not exist", domain);
        // @todo skip right here (how to resolve the future here?)
        // return;
    }

    ready(hydrate_whois(lookup, domain))
}