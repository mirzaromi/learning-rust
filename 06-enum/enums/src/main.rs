// fn main() {
//     enum IpAddrKind {
//         V4,
//         V6
//     }

//     struct IpAddr {
//         kind: IpAddrKind,
//         address: String
//     }

//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1")
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1")
//     };
// }

// fn main() {
//     enum IpAddr {
//         V4(String),
//         V6(String)
//     }

//     let home = IpAddr::V4(String::from("127.0.0.1"));

//     let loopback = IpAddr::V6(String::from("::1"));
// }

// fn main() {
//     enum IpAddr {
//         V4(u8, u8, u8, u8),
//         V6(String)
//     }

//     let home = IpAddr::V4(127, 0, 0, 1);

//     let loopback = IpAddr::V6(String::from("::1"));
// }

// fn main() {
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5);

//     let sum = x + y.unwrap_or(10);
// }


// #[derive(Debug)] // so we can inspect the state in a minute
// enum IdState {
//     Banten,
//     DkiJakarta,
//     JawaBarat,
//     JawaTengah,
//     JawaTimur
// }

// #[derive(Debug)]
// enum Coin {
//     Penny,
//     Nickle,
//     Dime,
//     Quarter(IdState)
// }

// fn value_in_cents(coin: &Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickle => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("the state is {:?}", state);
//             25
//         }
//     }
// }

// fn main() {
//     let coin = Coin::Quarter(IdState::JawaTimur);

//     println!("the value of coin {:?} is {}", coin, value_in_cents(&coin));
// }


fn main() {
    let five = Some(5);
    let six = plus_one_if_exist(five);

    let none = plus_one_if_exist(None);

    println!("the value is {:?}", six);
    println!("the value is {:?}", none);
}

fn plus_one_if_exist(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(i) => Some(i + 1)
    }
}