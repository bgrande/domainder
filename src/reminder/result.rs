use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Reminder {
    pub domain: String,
    pub expiry: String,
    pub remind_time: String,
}