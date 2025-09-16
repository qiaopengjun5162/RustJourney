use std::{sync::OnceLock, thread};

static LOCK: OnceLock<usize> = OnceLock::new();

fn main() {
    assert!(LOCK.get().is_none());
    thread::spawn(|| {
        let value = LOCK.get_or_init(|| 42);
        assert_eq!(*value, 42);
        assert_eq!(value, &42);
        assert_eq!(LOCK.get(), Some(&42));
    })
    .join()
    .unwrap();

    assert_eq!(LOCK.get(), Some(&42));
}
