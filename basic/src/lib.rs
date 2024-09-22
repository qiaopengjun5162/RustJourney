mod json_schema;

use json_schema::get_string_literal;
use proc_macro::TokenStream;

use crate::json_schema::StructsTemplate;

#[proc_macro]
pub fn sql(_input: TokenStream) -> TokenStream {
    "fn hello() { println!(\"Hello, world!\"); }"
        .parse()
        .unwrap()
}

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    let filename = get_string_literal(input).unwrap();
    println!("filename: {}", filename);

    let result = StructsTemplate::render(&filename).unwrap();

    result.parse().unwrap()
}

// output
// struct Person {
//     first_name: string,
//     last_name: string,
// }
