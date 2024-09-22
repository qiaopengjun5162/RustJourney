use adder;

mod common;

#[test]
fn it_add() {
    common::setup();
    assert_eq!(5, adder::add(2, 3));
}
