use crate::RustType;
use openapiv3::ReferenceOr;
use openapiv3::RequestBody;
use serde_derive::Serialize;

use crate::{RustPascalIdentifier, RustSnakeIdentifier};

#[derive(Debug, Serialize)]
pub struct Api {
    pub snake_id: RustSnakeIdentifier,
    pub pascal_id: RustPascalIdentifier,
    pub methods: Vec<Method>, // Will be vec<Method>
}

#[derive(Debug, Serialize)]
pub struct Method {
    pub snake_id: RustSnakeIdentifier,
    pub path: String,
    pub http_method: String,
    pub path_parameters: Vec<Parameter>,
    pub query_parameters: Vec<Parameter>,
    pub body: Option<Parameter>,
    pub returns: Option<RustType>,
}

#[derive(Debug, Serialize)]
pub struct Parameter {
    api_id: String,
    snake_id: RustSnakeIdentifier,
    r#type: RustType,
    test_value: String,
}

impl From<&openapiv3::ParameterData> for Parameter {
    fn from(parameter_data: &openapiv3::ParameterData) -> Parameter {
        let r#type: RustType = parameter_data.into();

        Parameter {
            api_id: parameter_data.name.to_owned(),
            snake_id: parameter_data.name.to_owned().into(),
            test_value: match r#type.0.as_str() {
                "String" => format!("\"{}\".into()", parameter_data.name),
                "Vec<String>" => format!("vec![\"{}1\".into(), \"{}2\".into()]", parameter_data.name, parameter_data.name),
                t => format!("{}::default()", t),
            },
            r#type,
        }
    }
}

impl From<&ReferenceOr<RequestBody>> for Parameter {
    fn from(reference_or_requestbody: &ReferenceOr<RequestBody>) -> Parameter {
        let r#type: RustType = reference_or_requestbody.into();

        Parameter {
            api_id: "body".to_owned(),
            snake_id: "body".to_owned().into(),
            test_value: match r#type.0.as_str() {
                "String" => "\"body\".into()".into(),
                "Vec<String>" => "vec![\"body1\".into(), \"body2\".into()]".into(),
                t => format!("{}::default()", t),
            },
            r#type,
        }
    }
}
