use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dictionary {
    pub key: String,
    pub value: String,
    pub description: String,
    pub timestamp: String,
}

impl Dictionary {
    pub fn new(key: String, value: String, description: String) -> Self {
        Dictionary {
            key: key,
            value: value,
            description: description,
            timestamp: Local::now().to_string(),
        }
    }
}
