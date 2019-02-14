#[allow(unused_imports)]
use serde_json::Value;

use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#Pullrequest {
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    r#author: Option<User>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    r#id: Option<i32>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    r#repository: Option<Repository>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    r#title: Option<String>,
}

impl r#Pullrequest {
    pub fn new(
    ) -> Self {
        Self {
          r#author: None,
          r#id: None,
          r#repository: None,
          r#title: None,
        }
    }

    pub fn set_author(&mut self, r#author: User) {
        self.r#author = Some(r#author);
    }

    pub fn with_author(mut self, r#author: User) -> Self {
        self.r#author = Some(r#author);
        self
    }

    pub fn r#author(&self) -> Option<&User> {
        self.r#author.as_ref().map(|x| x.borrow())
    }

    pub fn reset_author(&mut self) {
        self.r#author = None;
    }

    pub fn set_id(&mut self, r#id: i32) {
        self.r#id = Some(r#id);
    }

    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = Some(r#id);
        self
    }

    pub fn r#id(&self) -> Option<&i32> {
        self.r#id.as_ref().map(|x| x.borrow())
    }

    pub fn reset_id(&mut self) {
        self.r#id = None;
    }

    pub fn set_repository(&mut self, r#repository: Repository) {
        self.r#repository = Some(r#repository);
    }

    pub fn with_repository(mut self, r#repository: Repository) -> Self {
        self.r#repository = Some(r#repository);
        self
    }

    pub fn r#repository(&self) -> Option<&Repository> {
        self.r#repository.as_ref().map(|x| x.borrow())
    }

    pub fn reset_repository(&mut self) {
        self.r#repository = None;
    }

    pub fn set_title(&mut self, r#title: String) {
        self.r#title = Some(r#title);
    }

    pub fn with_title(mut self, r#title: String) -> Self {
        self.r#title = Some(r#title);
        self
    }

    pub fn r#title(&self) -> Option<&str> {
        self.r#title.as_ref().map(|x| x.borrow())
    }

    pub fn reset_title(&mut self) {
        self.r#title = None;
    }
}
