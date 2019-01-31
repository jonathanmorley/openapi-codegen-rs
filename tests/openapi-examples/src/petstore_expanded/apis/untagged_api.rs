use std::borrow::Borrow;

use failure;
use hyper;
use serde_json;

use super::configuration;
use super::configuration::Configuration;
use super::request as _internal_request;

pub struct UntaggedApiClient {
    configuration: Configuration,
}

impl UntaggedApiClient {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            configuration: configuration,
        }
    }

    pub fn r#find_pets(
        &self,
        r#tags: Vec<String>,
        r#limit: i32,
    ) -> Result<Vec<super::super::models::Pet>, failure::Error> {
        _internal_request::Request::new(hyper::Method::GET, "/pets".to_string())
            .with_query_param("tags".to_string(), r#tags.to_string())
            .with_query_param("limit".to_string(), r#limit.to_string())
            .execute(self.configuration.borrow())
    }

    pub fn r#add_pet(
        &self,
        r#body: super::super::models::NewPet,
    ) -> Result<super::super::models::Pet, failure::Error> {
        _internal_request::Request::new(hyper::Method::POST, "/pets".to_string())
            .with_body_param(r#body)
            .execute(self.configuration.borrow())
    }

    pub fn r#find_pet_by_id(&self, r#id: i64) -> Result<super::super::models::Pet, failure::Error> {
        _internal_request::Request::new(hyper::Method::GET, "/pets/{id}".to_string())
            .with_path_param("id".to_string(), r#id.to_string())
            .execute(self.configuration.borrow())
    }

    pub fn r#delete_pet(&self, r#id: i64) -> Result<(), failure::Error> {
        _internal_request::Request::new(hyper::Method::DELETE, "/pets/{id}".to_string())
            .with_path_param("id".to_string(), r#id.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }
}

#[cfg(test)]
mod tests {
    use super::configuration::Configuration;
    use tc_core::{Container, Image};
    use tc_generic::{GenericImage, WaitFor};
    use testcontainers::*;
    #[test]
    fn r#find_pets() {
        client()
            .r#find_pets(vec!["tags1".into(), "tags2".into()], i32::default())
            .unwrap();
    }

    #[test]
    fn r#add_pet() {
        client()
            .r#add_pet(super::super::models::NewPet::default())
            .unwrap();
    }

    #[test]
    fn r#find_pet_by_id() {
        client().r#find_pet_by_id(i64::default()).unwrap();
    }

    #[test]
    fn r#delete_pet() {
        client().r#delete_pet(i64::default()).unwrap();
    }

    fn client() -> super::UntaggedApiClient {
        let docker = clients::Cli::default();
        let image = GenericImage::new("okta-apisprout:latest")
            .with_wait_for(WaitFor::message_on_stdout("Sprouting"));
        let server = docker.run(image);
        let host_port = server.get_host_port(8000).unwrap();
        let url = format!("http://localhost:{}", host_port);
        let configuration = Configuration::new(url);
        super::UntaggedApiClient::new(configuration)
    }
}
