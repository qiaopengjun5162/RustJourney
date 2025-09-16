use std::cell::OnceCell;

fn main() {
    let mut cell = OnceCell::new();
    let _ = cell.set(String::from("hello"));

    if let Some(value_ref) = cell.get_mut() {
        // *value_ref = "!".to_string();
        // value_ref.push('!');
        value_ref.push_str("!");
    }

    let _ = cell.set(String::from("World"));

    if let Some(value) = cell.get() {
        println!("Value: {value}");
    }
}
