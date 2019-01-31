#[allow(unused_imports)]
use serde_json::Value;

use std::borrow::Borrow;

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
