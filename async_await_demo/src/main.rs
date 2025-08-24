#![allow(unused)]

// Async / await

async fn g1() -> u32 {
    println!("g1 function called");
    1
}

async fn g2() -> u32 {
    println!("g2 function called");
    2
}

async fn g3() -> u32 {
    println!("g3 function called");
    3
}

// Make coffee
async fn f() -> (u32, u32, u32) {
    println!("f function called");

    // Boil water - returns Future
    // let a = g1().await;
    // Grind coffee beans
    // let b = g2().await;

    // Boil water and grind coffee beans simultaneously
    let (a, b) = tokio::join!(g1(), g2());

    // Pour hot water on coffee grinds
    let c = g3().await;

    (a, b, c)
}

#[tokio::main]
async fn main() {
    let (a, b, c) = f().await;
    println!("Coffee is ready: {} {} {}", a, b, c);
}
