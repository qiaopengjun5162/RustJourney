use std::{
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};

fn main() {
    let counter = AtomicUsize::new(0);

    thread::scope(|s| {
        for _ in 0..1000 {
            s.spawn(|| {
                incr(&counter);
            });
        }
    });

    println!("Counted: {}", { counter.load(Ordering::Relaxed) });
}

fn incr(counter: &AtomicUsize) {
    let mut current = counter.load(Ordering::Relaxed);
    loop {
        let new = current + 1;
        match counter.compare_exchange(current, new, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return,
            Err(v) => {
                println!("value changed {current} -> {v}");
                current = v;
            }
        }
    }
}
