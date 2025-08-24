use std::{sync::Arc, thread};

fn main() {
    let data = Arc::new([1, 2, 3, 4, 5]);

    let mut handles = Vec::new();

    for _ in 0..4 {
        let local_data = data.clone();
        let h = thread::spawn(move || {
            println!("Data: {local_data:?}");
        });
        handles.push(h);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
}

// static mut COUNTER: u32 = 0;

// fn main() {
//     let mut handles = Vec::new();

//     for _ in 0..10000 {
//         let h = thread::spawn(|| unsafe {
//             COUNTER += 1;
//         });
//         handles.push(h);
//     }

//     handles.into_iter().for_each(|h| h.join().unwrap());
//     println!("Counter: {}", unsafe { COUNTER });
// }

/*
static DATA: [i32; 5] = [1, 2, 3, 4, 5];

fn main() {
    let mut handles = Vec::new();

    for _ in 0..6 {
        let h = thread::spawn(|| {
            println!("Data: {DATA:#?}");
        });
        handles.push(h);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
}

*/
