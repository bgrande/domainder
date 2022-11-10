use std::{env, fs};
use chrono::{DateTime, Duration, Utc};
use crate::reminder::result::Reminder;
use serde_json::{self, from_str};
use crate::send::send::send_email;

const BASE_PATH: &str = "data/reminder";

enum DurationTypes {
    Months,
    Weeks,
    Days
}

pub(crate) fn remind() {
    let path = env::current_dir().unwrap();
    let paths = fs::read_dir(String::from(path.to_str().unwrap()) + "/" + BASE_PATH).unwrap();

    for file_path in paths {
        let data = fs::read_to_string(file_path.unwrap().path());

        let json: Reminder = from_str(&data.unwrap()).unwrap();
        if should_remind(&json) {
            if send_email(&json) {
                // todo: update status to sent -> file writing logic (might share this from reminder package)
            }
        }
        // @todo:
        // + iterate through reminder files
        // + calculate if a domain has to be reminded
        // + send reminder (to email)
        // 4. mark as reminded
        // 5. remove is_reminded mark if we're > reminding time before the expiry date, again (or after the expiry date)
    }
}

fn should_remind(reminder: &Reminder) -> bool {
    let now = Utc::now();
    let expiry = DateTime::parse_from_rfc3339(reminder.expiry.replace("Z", "+00:00").as_str());
    let duration_type = get_duration_type(&reminder);
    let duration_time = get_duration_time(&reminder);

    let duration = match duration_type {
        DurationTypes::Months => {
            // very simple months to weeks conversion here:
            let simple_weeks = 4 * duration_time;
            Duration::weeks(simple_weeks)
        },
        DurationTypes::Weeks => Duration::weeks(duration_time),
        DurationTypes::Days => Duration::days(duration_time),
    };

    let remind_date = expiry.unwrap().checked_sub_signed(duration);

    if remind_date.unwrap().le(&now) {
        return true
    }

    false
}

fn get_duration_type(reminder: &Reminder) -> DurationTypes {
    if reminder.remind_time.ends_with("m") {
        return DurationTypes::Months
    }

    if reminder.remind_time.ends_with("w") {
        return DurationTypes::Weeks
    }

    if reminder.remind_time.ends_with("d") {
        return DurationTypes::Days
    }

    DurationTypes::Months
}

fn get_duration_time(reminder: &Reminder) -> i64 {
    reminder.remind_time.trim_end_matches(|p| p == 'm' || p == 'w' || p == 'd').parse().unwrap_or(1)
}