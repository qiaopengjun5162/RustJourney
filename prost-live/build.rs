use prost_build::Config;

// https://docs.rs/prost-build/latest/prost_build/
fn main() {
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html
    println!("cargo:rerun-if-changed=person.proto");
    println!("cargo:rerun-if-changed=build.rs");
    Config::new()
        .out_dir("./src/pb")
        // .bytes(&["."])
        .btree_map(&["scopes"])
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .field_attribute("data", "#[serde(skip_serializing_if = \"Vec::is_empty\")]")
        .compile_protos(&["person.proto"], &["."])
        .unwrap();
}
