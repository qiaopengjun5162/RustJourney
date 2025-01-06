fn main() {
    // let s = String::from("Hello, World!");

    // take_ownership(s);

    // let x = 5;

    // makes_copy(x);

    // println!("x: {}", x);

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
}

// fn take_ownership(some_string: String) {
//     println!("{}", some_string)
// }

// fn makes_copy(some_number: i32) {
    // println!("{}", some_number);
// }

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
