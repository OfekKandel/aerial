use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[derive(Serialize, Deserialize, Clone)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Duration,
    pub time_set: SystemTime,
    pub refresh_token: String,
}

impl Token {
    pub fn as_auth(&self) -> String {
        format!("{}  {}", self.token_type, self.access_token)
    }

    pub fn is_valid(&self) -> bool {
        let experation_time = self.time_set + self.expires_in;
        return experation_time > SystemTime::now();
    }
}
