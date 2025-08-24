use std::cell::RefCell;

fn main() {
    let rc = RefCell::new(5);
    println!("rc = {rc:#?}");

    {
        let five = rc.borrow();
        let five1 = rc.borrow();
        assert_eq!(*five, 5);
        assert_eq!(*five1, 5);
    }

    let mut f = rc.borrow_mut();
    *f += 10;
    assert_eq!(*f, 15);
    println!("f = {f:#?}");

    let v = rc.try_borrow();
    assert!(v.is_err());

    drop(f);

    // RefMut 实现了 Deref Trait
    *rc.borrow_mut() += 10;

    println!("rc = {rc:#?}");
}
