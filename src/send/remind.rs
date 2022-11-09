use std::{env, fs};
use chrono::Utc;
use crate::reminder::result::Reminder;
use serde_json::{self, Result, Value};
use crate::send::send::send_email;

const BASE_PATH: &str = "data/reminder";

pub(crate) fn remind() {
   /* let path = env::current_dir()?;
    let base_path = path.to_str().ok_or("nothing to remind so far");

    Ok(base_path.unwrap().to_string() + "/" + BASE_PATH)
*/
    let path = env::current_dir().unwrap();
    let paths = fs::read_dir(String::from(path.to_str().unwrap()) + "/" + BASE_PATH).unwrap();

    for file_path in paths {
        let data = fs::read_to_string(file_path.unwrap().path());

        let json: Reminder = serde_json::from_str(&data.unwrap()).unwrap();
        if should_remind(&json) {
            send_email(&json);
        }
        // @todo:
        // + iterate through reminder files
        // 2. calculate if a domain has to be reminded
        // 3. send reminder (to email)
        // 4. mark as reminded
        // 5. remove is_reminded mark if we're > reminding time before the expiry date, again (or after the expiry date)
    }
}

fn should_remind(reminder: &Reminder) -> bool {
    let now = Utc::now();
   true
    // https://rust-lang-nursery.github.io/rust-cookbook/datetime/duration.html#perform-checked-date-and-time-calculations
}