use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token {
    anime_sn: u32,
    login: bool,
    // promote: [],
    /// unknown value meaning
    r18: u32,
    src: String,
    /// unknown value meaning
    time: u32,
    /// is vip account
    vip: bool,
}

impl Token {
    pub fn vip(&self) -> bool {
        self.vip
    }

    pub fn time(&self) -> u32 {
        self.time
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenError {
    code: String,
    message: String,
}

impl Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[code]: {}, [message]: {}", self.code, self.message)
    }
}

impl std::error::Error for TokenError {}
