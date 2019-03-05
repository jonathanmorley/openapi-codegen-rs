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


#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use tc_core::{Container, Image};
    use tc_generic::{GenericImage, WaitFor};
    use testcontainers::*;
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
        std::process::Command::new("docker")
                  .args(&["build", "-t=test-apisprout", "."])
                  .output()
                  .expect("failed to execute process");

        let testcontainer_docker = clients::Cli::default();
        let image = GenericImage::new("test-apisprout:latest")
            .with_wait_for(WaitFor::message_on_stdout("Sprouting"));
        let server = testcontainer_docker.run(image);
        let host_port = server.get_host_port(8000).unwrap();
        let url = format!("http://localhost:{}", host_port);
        let configuration = Configuration::new(url);
        super::PetsApiClient::new(configuration)
    }
}

