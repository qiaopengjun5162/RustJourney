// const MAX_POINTS: u32 = 100_000;

fn main() {
    // const MAX_POINTS: u32 = 100_000;
    println!("Hello, world!");

    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The length of spaces is {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number");

    println!("The guess is {}", guess);

    // let x = 2.0; // f64

    // let y: f32 = 3.0; // f32

    // let sum = 5 + 10;

    // let difference = 95.5 - 4.3;

    // let product = 4 * 30;

    // let quotient = 56.7 / 32.2;

    // let reminder = 54 % 5;

    // let t = true;

    // let f: bool = false;

    // let x = 'z';
    // let y: char = 'a';
    // let z = '😘';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);

    // let a = [1, 2, 3, 4, 5];

    // let months = [
    //     "January",
    //     "February",
    //     "March",
    //     "April",
    //     "May",
    //     "June",
    //     "July",
    //     "August",
    //     "September",
    //     "October",
    //     "November",
    //     "December",
    // ];

    // let first = months[0];
    // let second = months[1];

    // let index = 15;
    // let index = [12, 13, 14, 15];
    // let month = months[index[1]];
    // println!("{}", month);

    another_function(5, 6); // argument

    // let y = 6; // 语句
    // let x = (let y = 6); // 

    // let x = 5;

    // let y = {
    //     let x = 1;
    //     x + 3
    // };

    // println!("The value of y is {}", y)

    let x = plus_five(6);
    println!("The value of x is {}", x);

    
    
}

fn another_function(x: i32, y: i32) { // parameter
    println!("Another function");
    println!("the value of x is {}", x);
    println!("the value of y is {}", y);
}

// This is a function
fn plus_five(x: i32) -> i32 {
    x + 5
}

