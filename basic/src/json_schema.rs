use std::{collections::HashMap, fs};

use anyhow::{anyhow, Result};
use askama::Template;
use heck::{AsPascalCase, AsSnakeCase};
use litrs::Literal;
use proc_macro::TokenStream;
use serde::{Deserialize, Serialize};

pub fn get_string_literal(input: TokenStream) -> Result<String> {
    input
        .into_iter()
        .next()
        .and_then(|v| Literal::try_from(v).ok())
        .and_then(|v| match v {
            Literal::String(s) => Some(s.value().to_string()),
            _ => None,
        })
        .ok_or_else(|| anyhow!("Only string literals are allowed"))
}

#[derive(Template)]
#[template(path = "code.j2")]
pub struct StructsTemplate {
    structs: Vec<St>,
}

impl StructsTemplate {
    fn try_new(filename: &str) -> Result<Self> {
        let content = fs::read_to_string(filename)?;
        let schema: Schema = serde_json::from_str(&content)?;
        Ok(Self {
            structs: schema.into_vec_st(),
        })
    }

    pub fn render(filename: &str) -> Result<String> {
        let template = Self::try_new(filename)?;
        Ok(template.render()?)
    }
}

/// input data
#[derive(Debug, Default, Serialize, Deserialize)]
struct Schema {
    title: Option<String>,
    #[serde(rename = "type")]
    ty: String,
    properties: Option<HashMap<String, Schema>>,
}

impl Schema {
    pub fn into_vec_st(&self) -> Vec<St> {
        let mut structs = vec![];
        match self.ty.as_str() {
            "object" => {
                // &self 是self的引用 properties是Option拿Option里面的东西是引用，需要 as_ref
                let fields = self
                    .properties
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| process_type(&mut structs, k.as_str(), v))
                    .collect();
                structs.push(St::new(p(self.title.as_ref().unwrap()), fields));
                structs
            }
            _ => panic!("Not supported yet"),
        }
    }
}

fn process_type(structs: &mut Vec<St>, k: &str, v: &Schema) -> Fd {
    let name = n(k);
    match v.ty.as_str() {
        "object" => {
            // need to create a new St, field type is St name
            let sts = v.into_vec_st();
            structs.extend(sts);
            Fd::new(name, p(v.title.as_deref().unwrap_or(k)))
        }
        "integer" => Fd::new(name, "i64"),
        "float" => Fd::new(name, "f64"),
        "boolean" => Fd::new(name, "bool"),
        "string" => Fd::new(name, "String"),
        v => panic!("Unsupported schema type: {}", v),
    }
}

// pascal case
fn p(s: &str) -> String {
    AsPascalCase(s).to_string()
}

// snake case
fn n(s: &str) -> String {
    AsSnakeCase(s).to_string()
}

/// output structure
pub struct St {
    /// structure name
    name: String,
    /// a list of structure fields
    fields: Vec<Fd>,
}

impl St {
    pub fn new(name: impl Into<String>, fields: Vec<Fd>) -> Self {
        Self {
            name: name.into(),
            fields,
        }
    }
}

pub struct Fd {
    /// field name
    name: String,
    /// field type
    ty: String,
}

impl Fd {
    pub fn new(name: impl Into<String>, ty: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ty: ty.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const PERSON1: &str = include_str!("../fixtures/person1.json");
    const PERSON2: &str = include_str!("../fixtures/person2.json");

    #[test]
    fn schema_should_be_converted_to_st() {
        let schema: Schema = serde_json::from_str(PERSON1).unwrap();
        let mut structs = schema.into_vec_st();
        assert_eq!(structs.len(), 1);
        let st = structs.pop().unwrap();

        assert_eq!(st.name, "Person");
        assert_eq!(st.fields.len(), 2);
        // assert_eq!(st.fields[0].name, "first_name"); // HashMap 顺序不确定
        let mut names = st.fields.iter().map(|f| f.name.clone()).collect::<Vec<_>>();
        names.sort();

        assert_eq!(&names, &["first_name", "last_name"]);
        assert_eq!(names, ["first_name", "last_name"]);
        assert_eq!(st.fields[0].ty, "String");
        assert_eq!(st.fields[1].ty, "String");
    }

    #[test]
    fn schema_with_nested_object_should_be_converted_to_st() {
        let schema: Schema = serde_json::from_str(PERSON2).unwrap();
        let mut structs = schema.into_vec_st();
        assert_eq!(structs.len(), 2);
        let st = structs.pop().unwrap();

        assert_eq!(st.name, "Person");
        assert_eq!(st.fields.len(), 3);

        let mut names = st.fields.iter().map(|f| f.name.clone()).collect::<Vec<_>>();
        names.sort();

        assert_eq!(&names, &["first_name", "last_name", "skill"]);
    }

    #[test]
    fn schema_render_should_work() {
        let result = StructsTemplate::render("fixtures/person2.json").unwrap();
        println!("result: {:#?}", result);
    }
}
