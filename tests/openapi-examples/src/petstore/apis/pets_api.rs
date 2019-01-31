use std::borrow::Borrow;

use failure;
use hyper;
use serde_json;

use super::request as _internal_request;
use super::configuration;
use super::configuration::Configuration;

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
    ) -> Result<super::super::models::Pets, failure::Error> {
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
    ) -> Result<super::super::models::Pets, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/pets/{petId}".to_string(),
        )
        .with_path_param("petId".to_string(), r#pet_id.to_string())
        .execute(self.configuration.borrow())
    }
}

#[cfg(test)]
mod tests {
    use super::configuration::Configuration;
    use testcontainers::*;
    use tc_core::{Container, Image};
    use tc_generic::{GenericImage, WaitFor};
    #[test]
    fn r#list_pets() {
        client().r#list_pets(
          i32::default(),
        ).unwrap();
    }

    
    #[test]
    fn r#create_pets() {
        client().r#create_pets(
        ).unwrap();
    }

    
    #[test]
    fn r#show_pet_by_id() {
        client().r#show_pet_by_id(
          "petId".into(),
        ).unwrap();
    }

    

    fn client() -> super::PetsApiClient {
        let docker = clients::Cli::default();
        let image = GenericImage::new("okta-apisprout:latest")
            .with_wait_for(WaitFor::message_on_stdout("Sprouting"));
        let server = docker.run(image);
        let host_port = server.get_host_port(8000).unwrap();
        let url = format!("http://localhost:{}", host_port);
        let configuration = Configuration::new(url);
        super::PetsApiClient::new(configuration)
    }
}
