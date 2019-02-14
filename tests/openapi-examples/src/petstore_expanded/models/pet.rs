#[allow(unused_imports)]
use serde_json::Value;

use std::borrow::Borrow;

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#Pet {
}

impl r#Pet {
    pub fn new(
    ) -> Self {
        Self {
        }
    }
}
