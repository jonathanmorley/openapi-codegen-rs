use crate::{RustPascalIdentifier, RustSnakeIdentifier, RustType};
use openapiv3::Schema;
use openapiv3::{AnySchema, ReferenceOr, SchemaVariant};
use serde_derive::Serialize;
use std::borrow::Borrow;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum DataType {
    Struct(Struct),
    Enum(Enum),
    NewType(NewType),
}

impl From<(String, ReferenceOr<Schema>)> for DataType {
    fn from((name, reference_or_schema): (String, ReferenceOr<Schema>)) -> Self {
        match reference_or_schema {
            ReferenceOr::Reference { reference } => unimplemented!(),
            ReferenceOr::Item(item) => match item {
                Schema::Any(any_schema) => {
                    DataType::Struct((name, any_schema.borrow() as &AnySchema).into())
                }
                Schema::Schema(schema_variant) => match schema_variant.borrow() {
                    SchemaVariant::String { .. } => {
                        DataType::Enum((name, schema_variant.borrow()).into())
                    }
                    SchemaVariant::Object { .. } => {
                        DataType::Struct((name, schema_variant.borrow() as &SchemaVariant).into())
                    }
                    SchemaVariant::Array { .. } => {
                        DataType::NewType((name, schema_variant.borrow() as &SchemaVariant).into())
                    }

                    e => {
                        dbg!(e);
                        unimplemented!()
                    }
                },
                _ => unimplemented!(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Struct {
    pub pascal_id: RustPascalIdentifier,
    pub snake_id: RustSnakeIdentifier,
    pub optional_fields: Vec<Field>,
    pub required_fields: Vec<Field>,
}

impl From<(String, &SchemaVariant)> for Struct {
    fn from((name, schema_variant): (String, &SchemaVariant)) -> Self {
        if let SchemaVariant::Object {
            properties,
            required,
            ..
        } = schema_variant
        {
            Struct {
                pascal_id: name.clone().into(),
                snake_id: name.into(),
                optional_fields: properties
                    .into_iter()
                    .filter(|(name, _)| !required.contains(name))
                    .map(|entry| entry.into())
                    .collect::<Vec<Field>>(),
                required_fields: properties
                    .into_iter()
                    .filter(|(name, _)| required.contains(name))
                    .map(|entry| entry.into())
                    .collect::<Vec<Field>>(),
            }
        } else {
            unimplemented!()
        }
    }
}

impl From<(String, &AnySchema)> for Struct {
    fn from((name, any_schema): (String, &AnySchema)) -> Self {
        Struct {
            pascal_id: name.clone().into(),
            snake_id: name.into(),
            optional_fields: any_schema
                .properties
                .iter()
                .filter(|(name, _)| !any_schema.required.contains(name))
                .map(|entry| entry.into())
                .collect::<Vec<Field>>(),
            required_fields: any_schema
                .properties
                .iter()
                .filter(|(name, _)| any_schema.required.contains(name))
                .map(|entry| entry.into())
                .collect::<Vec<Field>>(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NewType {
    pub pascal_id: RustPascalIdentifier,
    pub snake_id: RustSnakeIdentifier,
    pub inner: Field,
}

impl From<(String, &SchemaVariant)> for NewType {
    fn from((name, schema_variant): (String, &SchemaVariant)) -> Self {
        if let SchemaVariant::Array { items, .. } = schema_variant {
            let mut inner: Field = (&String::from("inner"), items).into();
            inner.r#type = RustType(format!("Vec<{}>", inner.r#type.0));
            inner.borrowed_type = inner.r#type.borrowed();

            NewType {
                pascal_id: name.clone().into(),
                snake_id: name.into(),
                inner,
            }
        } else {
            unimplemented!()
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Field {
    pub api_id: String,
    pub snake_id: RustSnakeIdentifier,
    pub r#type: RustType,
    pub borrowed_type: RustType,
    pub nullable: bool,
    pub default: String,
}

impl From<(&String, &ReferenceOr<Box<Schema>>)> for Field {
    fn from((name, reference_or_schema): (&String, &ReferenceOr<Box<Schema>>)) -> Self {
        let r#type: RustType = reference_or_schema.into();

        Field {
            api_id: name.to_owned(),
            snake_id: name.to_owned().into(),
            borrowed_type: r#type.borrowed(),
            nullable: match reference_or_schema {
                ReferenceOr::Reference { .. } => false,
                ReferenceOr::Item(schema) => match schema.borrow() {
                    Schema::Schema(schema_variant) => match schema_variant.borrow() {
                        SchemaVariant::String { schema_data, .. }
                        | SchemaVariant::Number { schema_data, .. }
                        | SchemaVariant::Integer { schema_data, .. }
                        | SchemaVariant::Object { schema_data, .. }
                        | SchemaVariant::Array { schema_data, .. }
                        | SchemaVariant::Boolean { schema_data, .. } => schema_data.nullable,
                    },
                    Schema::OneOf { one_of } => one_of.is_empty(),
                    _ => unimplemented!(),
                },
            },
            default: match r#type.0.borrow() {
                "String" => format!("\"{}\".into()", name),
                t => format!("{}::default()", t),
            },
            r#type,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Enum {
    pub pascal_id: RustPascalIdentifier,
    pub snake_id: RustSnakeIdentifier,
    pub variants: Vec<Variant>,
}

impl From<(String, &SchemaVariant)> for Enum {
    fn from((name, schema_variant): (String, &SchemaVariant)) -> Self {
        if let SchemaVariant::String { enumeration, .. } = schema_variant {
            Enum {
                pascal_id: name.clone().into(),
                snake_id: name.into(),
                variants: enumeration
                    .into_iter()
                    .map(|value| value.into())
                    .collect::<Vec<Variant>>(),
            }
        } else {
            unimplemented!()
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Variant {
    pub api_id: String,
    pub pascal_id: RustPascalIdentifier,
}

impl From<String> for Variant {
    fn from(name: String) -> Self {
        Variant {
            api_id: name.clone(),
            pascal_id: name.into(),
        }
    }
}

impl From<&String> for Variant {
    fn from(name: &String) -> Self {
        Variant {
            api_id: name.clone(),
            pascal_id: name.to_owned().into(),
        }
    }
}
