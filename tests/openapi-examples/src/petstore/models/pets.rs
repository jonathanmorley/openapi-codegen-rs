#[allow(unused_imports)]
use serde_json::Value;

use std::borrow::Borrow;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#Pets(Vec<super::super::models::Pet>);

impl r#Pets {
    pub fn new(inner: Vec<super::super::models::Pet>) -> Self {
        Self(inner)
    }

    pub fn set_inner(&mut self, inner: Vec<super::super::models::Pet>) {
        self.0 = inner;
    }

    pub fn with_inner(mut self, inner: Vec<super::super::models::Pet>) -> Self {
        self.0 = inner;
        self
    }

    pub fn inner(&self) -> &Vec<super::super::models::Pet> {
        self.0.borrow()
    }
}
