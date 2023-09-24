#![allow(unused)]
fn main() {
    let _four = IpAddrKind::V4(String::from("asht"));
    let _six = IpAddrKind::V6(String::from("thas"));

    route(IpAddrKind::V4(String::from("asht")));
    route(IpAddrKind::V6(String::from("asht")));

    let my_coin = Coin::Quarter(UsState::Alabama);
    let amount = value_in_cents(my_coin);

    println!("my_coin is worth: {amount}");

    let coin0 = Coin::Penny;
    let coin1 = Coin::Nickel;
    let coin2 = Coin::Dime;
    let coin3 = Coin::Quarter(UsState::Alaska);

    let mut non_quarter_count = 0;

    count_non_quarters(coin0, &mut non_quarter_count);
    count_non_quarters(coin1, &mut non_quarter_count);
    count_non_quarters(coin2, &mut non_quarter_count);
    count_non_quarters(coin3, &mut non_quarter_count);

    println!("non_quarter_count: {non_quarter_count}");
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

fn count_non_quarters(coin: Coin, count: &mut i32) {
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        *count += 1;
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => todo!(),
        Some(i) => Some(i + 1),
    }
}