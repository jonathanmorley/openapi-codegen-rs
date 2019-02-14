#[allow(unused_imports)]
use serde_json::Value;

use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#Error {
    #[serde(rename = "code")]
    r#code: i32,
    #[serde(rename = "message")]
    r#message: String,
}

impl r#Error {
    pub fn new(
        r#code: i32,
        r#message: String,
    ) -> Self {
        Self {
          r#code: code,
          r#message: message,
        }
    }

    pub fn set_code(&mut self, r#code: i32) {
        self.r#code = r#code;
    }

    pub fn with_code(mut self, r#code: i32) -> Self {
        self.r#code = r#code;
        self
    }

    pub fn r#code(&self) -> &i32 {
        self.r#code.borrow()
    }

    pub fn set_message(&mut self, r#message: String) {
        self.r#message = r#message;
    }

    pub fn with_message(mut self, r#message: String) -> Self {
        self.r#message = r#message;
        self
    }

    pub fn r#message(&self) -> &str {
        self.r#message.borrow()
    }
}
