use std::{
    sync::{Arc, RwLock},
    thread,
};

fn main() {
    let counter = Arc::new(RwLock::new(0));

    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let value = counter.read().unwrap();
            println!("Thread {i} read the value {value}");
        });
        handles.push(handle);
    }

    {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut value = counter.write().unwrap();
            *value += 1;
            println!("Thread wrote the value {value}");
        });
        handles.push(handle);
    }

    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());

    println!("Result: {}", *counter.read().unwrap());
}
