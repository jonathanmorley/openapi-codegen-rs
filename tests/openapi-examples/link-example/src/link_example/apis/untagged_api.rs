use std::borrow::Borrow;

use failure;
use hyper;

#[allow(unused_imports)]
use serde_json::Value;

use super::request as _internal_request;
use super::configuration::Configuration;

#[allow(unused_imports)]
use super::super::models::*;

pub struct UntaggedApiClient {
    configuration: Configuration,
}

impl UntaggedApiClient {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            configuration: configuration,
        }
    }

    pub fn r#get_repositories_by_owner(
        &self,
        r#username: String,
    ) -> Result<Vec<Repository>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/2.0/repositories/{username}".to_string(),
        )
        .with_path_param("username".to_string(), r#username.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#get_repository(
        &self,
        r#username: String,
        r#slug: String,
    ) -> Result<Repository, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/2.0/repositories/{username}/{slug}".to_string(),
        )
        .with_path_param("username".to_string(), r#username.to_string())
        .with_path_param("slug".to_string(), r#slug.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#get_pull_requests_by_repository(
        &self,
        r#username: String,
        r#slug: String,
        r#state: String,
    ) -> Result<Vec<Pullrequest>, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/2.0/repositories/{username}/{slug}/pullrequests".to_string(),
        )
        .with_path_param("username".to_string(), r#username.to_string())
        .with_path_param("slug".to_string(), r#slug.to_string())
        .with_query_param("state".to_string(), r#state.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#get_pull_requests_by_id(
        &self,
        r#username: String,
        r#slug: String,
        r#pid: String,
    ) -> Result<Pullrequest, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/2.0/repositories/{username}/{slug}/pullrequests/{pid}".to_string(),
        )
        .with_path_param("username".to_string(), r#username.to_string())
        .with_path_param("slug".to_string(), r#slug.to_string())
        .with_path_param("pid".to_string(), r#pid.to_string())
        .execute(self.configuration.borrow())
    }

    pub fn r#merge_pull_request(
        &self,
        r#username: String,
        r#slug: String,
        r#pid: String,
    ) -> Result<(), failure::Error> {
        _internal_request::Request::new(
            hyper::Method::POST,
            "/2.0/repositories/{username}/{slug}/pullrequests/{pid}/merge".to_string(),
        )
        .with_path_param("username".to_string(), r#username.to_string())
        .with_path_param("slug".to_string(), r#slug.to_string())
        .with_path_param("pid".to_string(), r#pid.to_string())
        .returns_nothing()
        .execute(self.configuration.borrow())
    }

    pub fn r#get_user_by_name(
        &self,
        r#username: String,
    ) -> Result<User, failure::Error> {
        _internal_request::Request::new(
            hyper::Method::GET,
            "/2.0/users/{username}".to_string(),
        )
        .with_path_param("username".to_string(), r#username.to_string())
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
    fn r#get_repositories_by_owner() {
        client().r#get_repositories_by_owner(
          "username".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#get_repository() {
        client().r#get_repository(
          "username".into(),
          "slug".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#get_pull_requests_by_repository() {
        client().r#get_pull_requests_by_repository(
          "username".into(),
          "slug".into(),
          "state".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#get_pull_requests_by_id() {
        client().r#get_pull_requests_by_id(
          "username".into(),
          "slug".into(),
          "pid".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#merge_pull_request() {
        client().r#merge_pull_request(
          "username".into(),
          "slug".into(),
          "pid".into(),
        ).unwrap();
    }

    
    #[test]
    fn r#get_user_by_name() {
        client().r#get_user_by_name(
          "username".into(),
        ).unwrap();
    }

    

    fn client() -> super::UntaggedApiClient {
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
        super::UntaggedApiClient::new(configuration)
    }
}

