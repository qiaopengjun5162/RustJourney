// enum IpAddrKind {
//     V4,
//     V6,
// }

// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // route(four);
    // route(six);
    // route(IpAddrKind::V6);

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let home = IpAddrKind::V4(127, 0, 0, 1);
    // let loopback = IpAddrKind::V6(String::from("::1"));

    // let q = Message::Quit;
    // let m = Message::Move { x: 12, y: 24 };
    // let w = Message::Write(String::from("hello"));
    // let c = Message::ChangeColor(0, 255, 255);

    // m.call();

    let some_number = Some(5);
    let some_string = Some("A String");

    let absent_number: Option<i32> = None;
}

// fn route(ip_kind: IpAddrKind) {}
