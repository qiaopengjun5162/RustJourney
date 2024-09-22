use adder;

mod common;

#[test]
fn it_adds2() {
    common::setup();
    assert_eq!(7, adder::add(3,4));
}
