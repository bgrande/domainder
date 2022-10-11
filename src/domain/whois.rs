use super::config;
use whois_rust::WhoIs;
use crate::domain::config::get_server_file_path;

fn whois_domain(domain: String) {
    /*if !config::get_server_file() {
        // @todo return with error
    } */

    let whois = WhoIs::from_path(get_server_file_path());
    /* @todo:
        1. convert to json (via filling struct with the data we need?)
        2. write required data into domains/domain_name.json (name, registry, registration_date, expiration_date)
        3. create reminder in reminder/[reminder_date].json
     */
}