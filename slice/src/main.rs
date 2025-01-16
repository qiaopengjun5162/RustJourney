fn main() {
    let my_string  = String::from("Hello world");
    let word_index = first_world(&my_string[..]);

    println!("{}", word_index);

    let my_string_literal = "hello world";
    let word_literal = first_world(my_string_literal);

    println!("word_literal: {}", word_literal);
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
