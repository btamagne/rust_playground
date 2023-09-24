#![allow(unused)]
fn main() {
    let _four = IpAddrKind::V4(String::from("asht"));
    let _six = IpAddrKind::V6(String::from("thas"));

    route(IpAddrKind::V4(String::from("asht")));
    route(IpAddrKind::V6(String::from("asht")));

    let my_coin = Coin::Quarter(UsState::Alabama);
    let amount = value_in_cents(my_coin);

    println!("my_coin is worth: {amount}");
}

enum IpAddrKind {
    V4(String),
    V6(String),
}

fn route(_ip_kind: IpAddrKind) {}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => todo!(),
        Some(i) => Some(i + 1),
    }
}