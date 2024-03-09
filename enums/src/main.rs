#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("cents in a penny: {}", value_in_cents(Coin::Penny));
    println!(
        "cents in a quarter: {}",
        value_in_cents(Coin::Quarter(UsState::Alaska))
    );

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Six: {}", six.unwrap());
    println!("None: {}", none.unwrap());
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
