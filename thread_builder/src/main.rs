use std::thread;

fn main() {
    let handle = thread::Builder::new()
        .name("Thread 1".into())
        .stack_size(1024 * 1024 * 4)
        .spawn(another_thread)
        .unwrap();

    handle.join().unwrap();
}

fn another_thread() {
    println!(
        "Hello from another thread, my name is {}",
        thread::current().name().unwrap()
    );
}

/*
RustJourney/thread_builder on  main [?] is 📦 0.1.0 via 🦀 1.88.0 on 🐳 v28.2.2 (orbstack)
➜ cargo build
   Compiling thread_builder v0.1.0 (/Users/qiaopengjun/Code/Rust/RustJourney/thread_builder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.63s

RustJourney/thread_builder on  main [?] is 📦 0.1.0 via 🦀 1.88.0 on 🐳 v28.2.2 (orbstack)
➜ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/thread_builder`
Hello from another thread, my name is Thread 1

RustJourney/thread_builder on  main [?] is 📦 0.1.0 via 🦀 1.88.0 on 🐳 v28.2.2 (orbstack)
➜ tree . -L 6 -I
tree: Missing argument to -I option.

RustJourney/thread_builder on  main [?] is 📦 0.1.0 via 🦀 1.88.0 on 🐳 v28.2.2 (orbstack)
➜ tree . -L 6 -I "target"
.
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs

2 directories, 3 files

*/
