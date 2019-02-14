use crate::client::api::Api;
use crate::client::api::Method;
use crate::client::model::DataType;
use failure::Error;
use handlebars::Handlebars;
use openapiv3::OpenAPI;
use openapiv3::Operation;
use openapiv3::Parameter;
use openapiv3::PathItem;
use openapiv3::ReferenceOr;
use serde_derive::Serialize;
use serde_yaml;
use std::collections::HashSet;
use std::fs::{DirBuilder, File};
use std::io::Write;
use std::path::Path;

mod api;
mod model;

pub fn client(api_path: &str, output_dir: &str, tests: bool) -> Result<(), Error> {
    let mut reg = Handlebars::new();
    reg.register_escape_fn(handlebars::no_escape);
    reg.register_template_string("api_mod", include_str!("resources/api_mod.mustache"))?;
    reg.register_template_string("api", include_str!("resources/api.mustache"))?;
    reg.register_template_string("model_enum", include_str!("resources/model_enum.mustache"))?;
    reg.register_template_string("model_mod", include_str!("resources/model_mod.mustache"))?;
    reg.register_template_string(
        "model_struct",
        include_str!("resources/model_struct.mustache"),
    )?;
    reg.register_template_string(
        "model_newtype",
        include_str!("resources/model_newtype.mustache"),
    )?;
    reg.register_template_string("mod", include_str!("resources/mod.mustache"))?;

    let dest_path = Path::new(&output_dir);
    DirBuilder::new().recursive(true).create(&dest_path)?;

    let spec: OpenAPI = serde_yaml::from_reader(File::open(api_path)?)?;

    DirBuilder::new()
        .recursive(true)
        .create(&dest_path.join("apis"))?;

    let mut configuration = File::create(&dest_path.join("apis/configuration.rs"))?;
    configuration.write_all(include_bytes!("resources/configuration.rs"))?;

    let mut request = File::create(&dest_path.join("apis/request.rs"))?;
    request.write_all(include_bytes!("resources/request.rs"))?;

    let apis = spec_apis(&spec, tests);

    let api_mod = File::create(&dest_path.join("apis/mod.rs"))?;
    reg.render_to_write("api_mod", &apis, api_mod)?;

    for api in &apis {
        let api_file = File::create(&dest_path.join(format!("apis/{}_api.rs", api.snake_id)))?;
        reg.render_to_write("api", &api, api_file)?;
    }

    let models = match spec.components {
        Some(components) => components
            .schemas
            .into_iter()
            .map(|entry| entry.into())
            .collect::<Vec<DataType>>(),
        None => vec![],
    };

    let models_path = dest_path.join("models");
    DirBuilder::new().recursive(true).create(&models_path)?;

    let models_mod = File::create(models_path.join("mod.rs"))?;
    reg.render_to_write("model_mod", &models, models_mod)?;

    for model in &models {
        match model {
            DataType::Struct(_struct) => {
                let model = File::create(models_path.join(format!("{}.rs", _struct.snake_id)))?;
                reg.render_to_write("model_struct", &_struct, model)?;
            }
            DataType::NewType(newtype) => {
                let model = File::create(models_path.join(format!("{}.rs", newtype.snake_id)))?;
                reg.render_to_write("model_newtype", &newtype, model)?;
            }
            DataType::Enum(_enum) => {
                let model = File::create(models_path.join(format!("{}.rs", _enum.snake_id)))?;
                reg.render_to_write("model_enum", &_enum, model)?;
            }
        }
    }

    let mod_file = if output_dir == "src" {
        File::create(dest_path.join("lib.rs"))?
    } else {
        File::create(dest_path.join("mod.rs"))?
    };

    let r#mod = Mod {
        root: output_dir == "src",
    };

    reg.render_to_write("mod", &r#mod, mod_file)?;

    Ok(())
}

#[derive(Debug, Serialize)]
struct Mod {
    root: bool,
}

fn spec_apis(spec: &OpenAPI, tests: bool) -> Vec<Api> {
    paths_tags(spec)
        .into_iter()
        .map(|tag| Api {
            snake_id: tag.clone().unwrap_or("untagged".to_string()).into(),
            pascal_id: tag.clone().unwrap_or("untagged".to_string()).into(),
            methods: spec
                .paths
                .iter()
                .filter(|(_path, reference_or_operations)| {
                    operations_tags(reference_or_operations).contains(&tag)
                })
                .flat_map(|(path, reference_or_operations)| {
                    operations_methods(path, reference_or_operations)
                })
                .collect(),
            tests,
        })
        .collect()
}

fn operations_methods(path: &str, reference_or_operations: &ReferenceOr<PathItem>) -> Vec<Method> {
    match reference_or_operations {
        ReferenceOr::Reference { .. } => unimplemented!(),
        ReferenceOr::Item(operations) => {
            let options =
                vec![
                    operations.get.as_ref().map(|operation| {
                        operation_method("GET".into(), path.to_owned(), &operation)
                    }),
                    operations.post.as_ref().map(|operation| {
                        operation_method("POST".into(), path.to_owned(), &operation)
                    }),
                    operations.put.as_ref().map(|operation| {
                        operation_method("PUT".into(), path.to_owned(), &operation)
                    }),
                    operations.patch.as_ref().map(|operation| {
                        operation_method("PATCH".into(), path.to_owned(), &operation)
                    }),
                    operations.delete.as_ref().map(|operation| {
                        operation_method("DELETE".into(), path.to_owned(), &operation)
                    }),
                ];

            options.into_iter().filter_map(|o| o).collect()
        }
    }
}

fn operation_method(method: String, path: String, operation: &Operation) -> Method {
    Method {
        snake_id: match operation.operation_id.as_ref() {
            Some(operation_id) => operation_id.to_owned(),
            None => format!("{}/{}", method, path),
        }
        .into(),
        path: path,
        http_method: method,
        path_parameters: operation
            .parameters
            .iter()
            .filter_map(|reference_or_parameter| {
                if let ReferenceOr::Item(parameter) = reference_or_parameter {
                    Some(parameter)
                } else {
                    None
                }
            })
            .filter_map(|parameter| {
                if let Parameter::Path { parameter_data, .. } = parameter {
                    Some(parameter_data)
                } else {
                    None
                }
            })
            .map(|parameter_data| parameter_data.into())
            .collect(),
        query_parameters: operation
            .parameters
            .iter()
            .filter_map(|reference_or_parameter| {
                if let ReferenceOr::Item(parameter) = reference_or_parameter {
                    Some(parameter)
                } else {
                    None
                }
            })
            .filter_map(|parameter| {
                if let Parameter::Query { parameter_data, .. } = parameter {
                    Some(parameter_data)
                } else {
                    None
                }
            })
            .map(|parameter_data| parameter_data.into())
            .collect(),
        body: operation
            .request_body
            .as_ref()
            .map(|parameter| parameter.into()),
        returns: operation
            .responses
            .responses
            .get("200")
            .and_then(|reference_or_response| match reference_or_response {
                ReferenceOr::Item(response) => response.content.get("application/json"),
                _ => unimplemented!(),
            })
            .and_then(|reference_or_mediatype| match reference_or_mediatype {
                ReferenceOr::Item(mediatype) => mediatype.schema.as_ref(),
                _ => unimplemented!(),
            })
            .map(|reference_or_schema| reference_or_schema.into()),
    }
}

fn paths_tags(spec: &OpenAPI) -> HashSet<Option<String>> {
    spec.paths
        .iter()
        .flat_map(|(_, operations)| operations_tags(operations))
        .collect()
}

fn operations_tags(reference_or_operations: &ReferenceOr<PathItem>) -> Vec<Option<String>> {
    match reference_or_operations {
        ReferenceOr::Reference { .. } => unimplemented!(),
        ReferenceOr::Item(operations) => {
            let mut tags = operation_tags(operations.get.as_ref());
            tags.append(&mut operation_tags(operations.post.as_ref()));
            tags.append(&mut operation_tags(operations.put.as_ref()));
            tags.append(&mut operation_tags(operations.patch.as_ref()));
            tags.append(&mut operation_tags(operations.delete.as_ref()));

            tags
        }
    }
}

fn operation_tags(operation: Option<&Operation>) -> Vec<Option<String>> {
    match operation {
        Some(operation) => {
            if operation.tags.is_empty() {
                vec![None]
            } else {
                operation.tags.iter().map(|s| Some(s.to_string())).collect()
            }
        }
        None => Vec::new(),
    }
}
