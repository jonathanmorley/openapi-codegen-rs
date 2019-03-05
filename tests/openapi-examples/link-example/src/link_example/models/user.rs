#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#User {
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    r#username: Option<String>,
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    r#uuid: Option<String>,
}

impl r#User {
    pub fn new(
    ) -> Self {
        Self {
          r#username: None,
          r#uuid: None,
        }
    }

    pub fn set_username(&mut self, r#username: String) {
        self.r#username = Some(r#username);
    }

    pub fn with_username(mut self, r#username: String) -> Self {
        self.r#username = Some(r#username);
        self
    }

    pub fn r#username(&self) -> Option<&str> {
        self.r#username.as_ref().map(|x| x.borrow())
    }

    pub fn reset_username(&mut self) {
        self.r#username = None;
    }

    pub fn set_uuid(&mut self, r#uuid: String) {
        self.r#uuid = Some(r#uuid);
    }

    pub fn with_uuid(mut self, r#uuid: String) -> Self {
        self.r#uuid = Some(r#uuid);
        self
    }

    pub fn r#uuid(&self) -> Option<&str> {
        self.r#uuid.as_ref().map(|x| x.borrow())
    }

    pub fn reset_uuid(&mut self) {
        self.r#uuid = None;
    }
}
