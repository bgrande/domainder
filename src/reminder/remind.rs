use crate::reminder::result::Reminder;
use std::{env, fs};

const BASE_PATH: &str = "data/reminder";

pub(crate) fn write_reminder(record: &Reminder) -> std::io::Result<()> {
    std::fs::write(
        get_reminder_path(record.domain.clone()).unwrap(),
        serde_json::to_string_pretty(&record).unwrap(),
    )
}

pub(crate) fn hydrate_reminder(
    domain: &String,
    time: &String,
    expiry: &String,
    email: &String,
) -> Reminder {
    Reminder {
        email: email.trim().to_string(),
        domain: domain.trim().to_string(),
        expiry: expiry.trim().to_string(),
        remind_time: time.trim().to_string(),
        sent: 0,
    }
}

fn get_reminder_path(name: String) -> std::result::Result<String, std::io::Error> {
    let path = env::current_dir()?;
    let base_path = path.to_str().ok_or("could not unwind base path");

    fs::create_dir_all(base_path.unwrap().to_string() + "/" + BASE_PATH)
        .expect("failed creating the reminder path");

    Ok(format!("{}/{}/{}", base_path.unwrap(), BASE_PATH, name))
}
