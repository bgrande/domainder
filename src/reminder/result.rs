use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Reminder {
    pub email: String,
    pub domain: String,
    pub expiry: String,
    pub remind_time: String,
    pub sent: u8,
}
