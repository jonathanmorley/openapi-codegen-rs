use std::borrow::Borrow;

use failure;
use hyper;

#[allow(unused_imports)]
use serde_json::Value;

use super::request as _internal_request;
use super::configuration::Configuration;

#[allow(unused_imports)]
use super::super::models::*;

pub struct PetsApiClient {
    configuration: Configuration,
}

impl PetsApiClient {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            configuration: configuration,
        }
    }

    pub fn r#list_pets(
        &self,
        r#limit: i32,
    ) -> Result<Pets, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/pets".to_string(),
        )
        .with_query_param("limit".to_string(), r#limit.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#create_pets(
        &self,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/pets".to_string(),
        )
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#show_pet_by_id(
        &self,
        r#pet_id: String,
    ) -> Result<Pets, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/pets/{petId}".to_string(),
        )
        .with_path_param("petId".to_string(), r#pet_id.to_string())
        .execute(self.configuration.borrow())
    }
}


