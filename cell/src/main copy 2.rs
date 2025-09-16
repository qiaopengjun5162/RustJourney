use std::cell::OnceCell;

fn main() {
    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    let result = cell.set(String::from("hello"));
    assert!(result.is_ok());

    let result = cell.set(String::from("world"));
    assert!(result.is_err());
}
