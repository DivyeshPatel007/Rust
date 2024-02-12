enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
enum Message {
    Quit,
    Movev { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_fn() {
        println!("Hello world");
    }
}

// struct IpAddr{
//     kind:IpAddrKind,
//     address:String,
// }

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let localhost = IpAddr{
    //     kind:IpAddrKind::V4,
    //     address:String::from("127.0.0.0"),
    // };

    // let localhostV6 = IpAddrKind::V6(String::from("127.0.0"));
    // let localhostV4 = IpAddrKind::V4(127, 0, 0, 1);

    // let some_number = Some(5);
    // let some_string = Some("a string");
    // let absent_number: Option<i32> = None;

    // let x = 5;
    // let y = Some(5);

    // let sum = x + y.unwrap_or(0);

    // Match Function using enums
    // value_in_cents(Coin::Quarter(UsState::California));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // None=>None,
        Some(i) => Some(i + 1),
        _ => None,
    }
}
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     Arizona,
//     Arkansas,
//     California,
// }
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("state quater from {:?}",state);
//             25
//         },
//     }
// }
