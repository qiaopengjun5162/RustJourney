use std::thread;

fn main() {
    const CHUNK_SIZE: usize = 10;
    let numbers: Vec<u32> = (1..10000).collect();
    let chunks = numbers.chunks(CHUNK_SIZE);

    let total_sum = thread::scope(|s| {
        let mut handles = Vec::new();

        for chunk in chunks {
            let handle = s.spawn(move || chunk.iter().sum::<u32>());
            handles.push(handle);
        }

        // let mut total_sum = 0;
        // for handle in handles {
        //     total_sum += handle.join().unwrap();
        // }

        // println!("Total sum: {total_sum}"); // Total sum: 49995000

        handles.into_iter().map(|h| h.join().unwrap()).sum::<u32>()
    });

    println!("Total sum: {total_sum}"); // Total sum: 49995000
}

// use std::{thread, time::Duration};

// fn main() {
//     // let a = String::from("Hello");

//     // ’scope 作用域线程可在此生命周期生成和运行
//     // 'scope 生命周期比 scope 函数内闭包的生命周期长
//     // 所以作用域线程可能活的比闭包长
//     thread::scope(|s| {
//         for i in 0..5 {
//             s.spawn(move || {
//                 thread::sleep(Duration::from_secs(1));
//                 println!("Scoped thread: {i}");
//             });
//         }
//     }); // non-'static
//     // 'env 'ENV 生命周期代表 被作用域线程借用的那些数据的生命周期 必须要长于 scope 的调用周期

//     // thread::spawn(||); 'static
// }

// fn main() {
//     thread::scope(|s| {
//         for i in 0..5 {
//             s.spawn(move || {
//                 thread::sleep(Duration::from_secs(1));
//                 println!("Scoped thread: {i}");
//             });
//         }
//     });
// }

// fn main() {
//     let mut handles = vec![];

//     for i in 0..5 {
//         let handle = thread::spawn(move || {
//             thread::sleep(Duration::from_secs(1));
//             println!("Normal thread: {i}");
//         });
//         handles.push(handle);
//     }

//     // for handle in handles {
//     //     handle.join().unwrap();
//     // }

//     handles
//         .into_iter()
//         .for_each(|handle| handle.join().unwrap());
// }
