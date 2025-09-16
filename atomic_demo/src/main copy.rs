use std::{
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::Duration,
};

fn main() {
    let done = AtomicUsize::new(0);

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                for _ in 0..100 {
                    thread::sleep(Duration::from_millis(20));
                    // let current = done.load(Ordering::Relaxed);
                    // done.store(current + 1, Ordering::Relaxed);

                    done.fetch_add(1, Ordering::Relaxed);
                }
            });
        }

        loop {
            let n = done.load(Ordering::Relaxed);
            if n == 1000 {
                break;
            }
            println!("Progress: {n}/1000 done!");
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("All done!");
}
