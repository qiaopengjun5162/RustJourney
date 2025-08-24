use std::cell::Cell;

fn main() {
    let cell = Cell::new(5);
    assert_eq!(cell.get(), 5);

    assert_eq!(cell.replace(10), 5);
    assert_eq!(cell.get(), 10);

    let ten = cell.into_inner();
    assert_eq!(ten, 10);

    let cell = Cell::new(String::from("hello"));
    assert_eq!(cell.take(), "hello");
    assert_eq!(cell.take(), String::default());

    cell.set(String::from("world"));
    assert_eq!(cell.take(), "world");
}
