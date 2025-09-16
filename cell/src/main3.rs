use std::cell::OnceCell;

fn main() {
    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    let value = cell.get_or_init(|| "Hello, World!".to_string());
    assert_eq!(value, "Hello, World!");
    assert_eq!(cell.get(), Some(&"Hello, World!".to_string()));
    assert_eq!(cell.get(), Some(value));
    assert!(cell.get().is_some());
}
