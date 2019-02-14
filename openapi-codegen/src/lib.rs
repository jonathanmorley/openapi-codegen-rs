pub mod client;

pub use client::client;
use heck::{CamelCase, SnakeCase};
use lazy_static::lazy_static;
use openapiv3::ParameterData;
use openapiv3::RequestBody;
use openapiv3::Schema as SchemaV3;
use openapiv3::SchemaVariant;
use openapiv3::{IntegerFormat, NumberFormat, ReferenceOr, StringFormat, VariantOrUnknownOrEmpty};
use regex::Regex;
use serde_derive::Serialize;
use std::borrow::Borrow;
use std::fmt;

lazy_static! {
    static ref STRICT_KEYWORDS: [&'static str; 36] = [
        "as", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false",
        "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
        "ref", "return", "Self", "self", "static", "struct", "super", "trait", "true", "type",
        "unsafe", "use", "where", "while",
    ];
}

lazy_static! {
    static ref RESERVED_KEYWORDS: [&'static str; 15] = [
        "abstract", "async", "await", "become", "box", "do", "final", "macro", "override", "priv",
        "try", "typeof", "unsized", "virtual", "yield",
    ];
}

lazy_static! {
    static ref RAW_INCOMPATIBLE_KEYWORDS: [&'static str; 5] =
        ["crate", "extern", "self", "Self", "super"];
}

lazy_static! {
    static ref INVALID_PATTERNS: Regex = Regex::new(r"[^a-zA-Z0-9_]").unwrap();
}

/// These are potentically keywords, so should be prefixed with r# for safety
#[derive(Debug, Serialize)]
pub struct RustSnakeIdentifier(String);

impl From<String> for RustSnakeIdentifier {
    fn from(s: String) -> Self {
        let identifier = INVALID_PATTERNS.replace_all(&s, " ").to_snake_case();

        if RAW_INCOMPATIBLE_KEYWORDS.contains(&identifier.borrow()) {
            RustSnakeIdentifier(format!("{}_", identifier))
        } else {
            RustSnakeIdentifier(identifier)
        }
    }
}

impl fmt::Display for RustSnakeIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// These are potentically keywords, so should be prefixed with r# for safety
#[derive(Debug, Serialize)]
pub struct RustPascalIdentifier(String);

impl From<String> for RustPascalIdentifier {
    fn from(s: String) -> Self {
        let identifier = INVALID_PATTERNS.replace_all(&s, "_").to_camel_case();

        if RAW_INCOMPATIBLE_KEYWORDS.contains(&identifier.borrow()) {
            RustPascalIdentifier(format!("{}_", identifier))
        } else {
            RustPascalIdentifier(identifier)
        }
    }
}

impl fmt::Display for RustPascalIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize)]
pub struct RustType(String);

impl From<&SchemaVariant> for RustType {
    fn from(schema_variant: &SchemaVariant) -> RustType {
        // https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.0.md#dataTypes
        RustType(match schema_variant {
            SchemaVariant::String { format, .. } => match format {
                VariantOrUnknownOrEmpty::Item(StringFormat::Date) => "String",
                VariantOrUnknownOrEmpty::Item(StringFormat::DateTime) => "String",
                VariantOrUnknownOrEmpty::Item(StringFormat::Password) => "String",
                VariantOrUnknownOrEmpty::Item(StringFormat::Byte) => "String",
                VariantOrUnknownOrEmpty::Item(StringFormat::Binary) => "String",
                VariantOrUnknownOrEmpty::Unknown(_) => "String",
                VariantOrUnknownOrEmpty::Empty => "String",
            }
            .into(),
            SchemaVariant::Number { format, .. } => match format {
                VariantOrUnknownOrEmpty::Item(NumberFormat::Float) => "f32",
                VariantOrUnknownOrEmpty::Item(NumberFormat::Double) => "f64",
                VariantOrUnknownOrEmpty::Unknown(_) => "f32",
                VariantOrUnknownOrEmpty::Empty => "f32",
            }
            .into(),
            SchemaVariant::Integer { format, .. } => match format {
                VariantOrUnknownOrEmpty::Item(IntegerFormat::Int32) => "i32",
                VariantOrUnknownOrEmpty::Item(IntegerFormat::Int64) => "i64",
                VariantOrUnknownOrEmpty::Unknown(_) => "i32",
                VariantOrUnknownOrEmpty::Empty => "i32",
            }
            .into(),
            SchemaVariant::Object { .. } => "Value".into(),
            SchemaVariant::Array { items, .. } => format!("Vec<{}>", RustType::from(items)),
            SchemaVariant::Boolean { .. } => "bool".into(),
        })
    }
}

impl From<&ReferenceOr<Box<SchemaV3>>> for RustType {
    fn from(reference_or_schema: &ReferenceOr<Box<SchemaV3>>) -> RustType {
        match reference_or_schema {
            ReferenceOr::Reference { reference } => {
                let type_name = reference
                    .trim_start_matches("#/components/schemas/")
                    .trim_start_matches("#/components/requestBodies/")
                    .to_camel_case();
                RustType(format!("{}", type_name))
            }
            ReferenceOr::Item(schema) => match schema.borrow() {
                SchemaV3::Schema(schema_variant) => {
                    (schema_variant.borrow() as &SchemaVariant).into()
                }
                SchemaV3::Any(any_schema) => {
                    // struct
                    dbg!(any_schema);
                    unimplemented!()
                }
                SchemaV3::OneOf { one_of } => {
                    if one_of.is_empty() {
                        RustType("Value".into())
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            },
        }
    }
}

impl From<&ReferenceOr<SchemaV3>> for RustType {
    fn from(reference_or_schema: &ReferenceOr<SchemaV3>) -> RustType {
        match reference_or_schema {
            ReferenceOr::Reference { reference } => {
                let type_name = reference
                    .trim_start_matches("#/components/schemas/")
                    .trim_start_matches("#/components/requestBodies/")
                    .to_camel_case();
                RustType(format!("{}", type_name))
            }
            ReferenceOr::Item(schema) => match schema {
                SchemaV3::Schema(schema_variant) => {
                    (schema_variant.borrow() as &SchemaVariant).into()
                }
                SchemaV3::Any(any_schema) => {
                    // struct
                    dbg!(any_schema);
                    unimplemented!()
                }
                SchemaV3::OneOf { one_of } => {
                    if one_of.is_empty() {
                        RustType("Value".into())
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            },
        }
    }
}

impl From<&ParameterData> for RustType {
    fn from(parameter_data: &ParameterData) -> RustType {
        match &parameter_data.format {
            openapiv3::ParameterSchemaOrContent::Content(_) => unimplemented!(),
            openapiv3::ParameterSchemaOrContent::Schema(ref reference_or_schema) => {
                reference_or_schema.into()
            }
        }
    }
}

impl From<&ReferenceOr<RequestBody>> for RustType {
    fn from(reference_or_requestbody: &ReferenceOr<RequestBody>) -> RustType {
        match reference_or_requestbody {
            ReferenceOr::Reference { reference } => {
                let type_name = reference
                    .trim_start_matches("#/components/schemas/")
                    .trim_start_matches("#/components/requestBodies/")
                    .to_camel_case();
                RustType(format!("{}", type_name))
            }
            ReferenceOr::Item(requestbody) => match requestbody.content.get("application/json") {
                Some(mediatype) => match mediatype.schema {
                    Some(ref reference_or_schema) => reference_or_schema.into(),
                    None => unimplemented!(),
                },
                None => {
                    dbg!(requestbody);
                    unimplemented!()
                }
            },
        }
    }
}

impl RustType {
    pub fn borrowed(&self) -> RustType {
        RustType(match self.0.as_str() {
            "String" => "&str".into(),
            x => format!("&{}", x),
        })
    }
}

impl fmt::Display for RustType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
