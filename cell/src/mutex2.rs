use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let data = Arc::new(Mutex::new(0));

    {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;
            panic!();
        })
        .join()
        .unwrap_err();
    }

    {
        let data = Arc::clone(&data);
        thread::spawn(move || match data.lock() {
            Ok(mut guard) => {
                println!("Thread 2: OK!");
                *guard += 10000;
            }
            Err(poisoned) => {
                println!("Thread 2: Poisoned!");
                let mut guard = poisoned.into_inner();
                *guard += 1;
                println!("Thread 2: New value {}", *guard);
            }
        })
        .join()
        .unwrap();
    }
}
