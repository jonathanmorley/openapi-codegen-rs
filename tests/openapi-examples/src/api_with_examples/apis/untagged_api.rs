use std::borrow::Borrow;

use failure;
use hyper;
use serde_json;

use super::request as _internal_request;
use super::configuration;
use super::configuration::Configuration;

pub struct UntaggedApiClient {
    configuration: Configuration,
}

impl UntaggedApiClient {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            configuration: configuration,
        }
    }

    pub fn r#list_versionsv2(
        &self,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/".to_string(),
        )
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#get_version_detailsv2(
        &self,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/v2".to_string(),
        )
        .returns_nothing()
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
    fn r#list_versionsv2() {
        client().r#list_versionsv2(
        ).unwrap();
    }

    
    #[test]
    fn r#get_version_detailsv2() {
        client().r#get_version_detailsv2(
        ).unwrap();
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
