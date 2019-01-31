#[allow(unused_imports)]
use serde_json::Value;

use std::borrow::Borrow;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#Repository {
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    r#owner: Option<super::super::models::User>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    r#slug: Option<String>,
}

impl r#Repository {
    pub fn new(
    ) -> Self {
        Self {
          r#owner: None,
          r#slug: None,
        }
    }

    pub fn set_owner(&mut self, r#owner: super::super::models::User) {
        self.r#owner = Some(r#owner);
    }

    pub fn with_owner(mut self, r#owner: super::super::models::User) -> Self {
        self.r#owner = Some(r#owner);
        self
    }

    pub fn r#owner(&self) -> Option<&super::super::models::User> {
        self.r#owner.as_ref().map(|x| x.borrow())
    }

    pub fn reset_owner(&mut self) {
        self.r#owner = None;
    }

    pub fn set_slug(&mut self, r#slug: String) {
        self.r#slug = Some(r#slug);
    }

    pub fn with_slug(mut self, r#slug: String) -> Self {
        self.r#slug = Some(r#slug);
        self
    }

    pub fn r#slug(&self) -> Option<&str> {
        self.r#slug.as_ref().map(|x| x.borrow())
    }

    pub fn reset_slug(&mut self) {
        self.r#slug = None;
    }
}
