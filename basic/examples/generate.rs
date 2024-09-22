use std::collections::HashMap;

use generated::{Person, Skill};
use serde::{Deserialize, Serialize};

pub mod generated {
    use basic::generate;
    generate!("fixtures/person.json");
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Schema {
    title: Option<String>,
    #[serde(rename = "type")]
    ty: String,
    properties: Option<HashMap<String, Schema>>,
}

fn main() {
    let person = Person {
        first_name: "John".into(),
        last_name: "Doe".into(),
        skill: Skill {
            name: "Rust".into(),
        },
    };
    println!("{:#?}", person);
}
