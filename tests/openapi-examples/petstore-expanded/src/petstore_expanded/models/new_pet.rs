#[allow(unused_imports)]
use serde_json::Value;

#[allow(unused_imports)]
use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#NewPet {
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    r#tag: Option<String>,
    #[serde(rename = "name")]
    r#name: String,
}

impl r#NewPet {
    pub fn new(
        r#name: String,
    ) -> Self {
        Self {
          r#tag: None,
          r#name: name,
        }
    }

    pub fn set_tag(&mut self, r#tag: String) {
        self.r#tag = Some(r#tag);
    }

    pub fn with_tag(mut self, r#tag: String) -> Self {
        self.r#tag = Some(r#tag);
        self
    }

    pub fn r#tag(&self) -> Option<&str> {
        self.r#tag.as_ref().map(|x| x.borrow())
    }

    pub fn reset_tag(&mut self) {
        self.r#tag = None;
    }

    pub fn set_name(&mut self, r#name: String) {
        self.r#name = r#name;
    }

    pub fn with_name(mut self, r#name: String) -> Self {
        self.r#name = r#name;
        self
    }

    pub fn r#name(&self) -> &str {
        self.r#name.borrow()
    }
}
