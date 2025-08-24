use std::{sync::Mutex, thread};

static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

fn main() {
    let mut handles = Vec::new();
    for _ in 0..20 {
        let handle = thread::spawn(move || {
            let mut numbers = NUMBERS.lock().unwrap();
            numbers.push(42);
        });
        handles.push(handle);
    }

    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());

    let numbers = NUMBERS.lock().unwrap();
    println!("{:?}", numbers);
}
