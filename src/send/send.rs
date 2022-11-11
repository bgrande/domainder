use crate::reminder::result::Reminder;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use std::{env, fs};

const CONFIG_PATH: &str = "config";
const CONFIG_FILE: &str = "/email.json";

#[derive(Serialize, Deserialize, Debug)]
struct EmailConfig {
    from_name: String,
    from_email: String,
    smtp_host: String,
    smtp_user: String,
    smtp_pass: String,
    smtp_port: String,
}

pub(crate) fn send_email(reminder: &Reminder) -> bool {
    let email = &reminder.email;

    let body = format!(
        "Hi!\r\n\r\nYou asked me to remind you about your domain \"{}\".\r\n\r\n\
        The domain is due to expire at {}.\r\n\r\n\
        Now, ask yourself! \
        Do you still need that domain?\r\n\
        If it's still important you might want to renew.\r\n\
        Otherwise just drop it and save some money.\r\n\
        Or you could even try to sell it!\r\n\r\n\
        All the best for your journey!",
        reminder.domain,
        reminder.expiry // todo format with a more readable display
    );

    let path = env::current_dir().unwrap();
    let config_file_path =
        String::from(path.to_str().unwrap()) + "/" + CONFIG_PATH + "/" + CONFIG_FILE;

    let data = fs::read_to_string(config_file_path);
    let json_config: EmailConfig = serde_json::from_str(&data.unwrap()).unwrap();

    let from = json_config.from_name + " <" + &json_config.from_email + ">";
    let subject = String::from("Your domain ") + &reminder.domain + " is about to expire";
    let to = String::from("<") + email.as_str() + ">";

    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap();

    let creds = Credentials::new(json_config.smtp_user, json_config.smtp_pass);

    let mailer = SmtpTransport::relay(&json_config.smtp_host)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => {
            println!("Email sent successfully!");
            true
        }
        Err(e) => {
            println!("Could not send email: {:?}", e);
            false
        }
    }
}
