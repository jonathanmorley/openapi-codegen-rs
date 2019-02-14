#[allow(unused_imports)]
use serde_json::Value;
use std::borrow::Borrow;
use super::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct r#Pets(Vec<Pet>);

impl r#Pets {
    pub fn new(inner: Vec<Pet>) -> Self {
        Self(inner)
    }

    pub fn set_inner(&mut self, inner: Vec<Pet>) {
        self.0 = inner;
    }

    pub fn with_inner(mut self, inner: Vec<Pet>) -> Self {
        self.0 = inner;
        self
    }

    pub fn inner(&self) -> &Vec<Pet> {
        self.0.borrow()
    }
}
