use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct WhoisResult {
    pub domain: String,
    pub server: String,
    pub updated: String,
    pub expiry: String,
    pub created: String,
}