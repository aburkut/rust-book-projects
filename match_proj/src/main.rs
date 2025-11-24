#[derive(Debug)]
enum UsState {
    Alaba,
    Alaska,
    Texas,
    //...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
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
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    println!("Penny cents: {}", value_in_cents(Coin::Penny));
    println!("Nickel cents: {}", value_in_cents(Coin::Nickel));
    println!("Dime cents: {}", value_in_cents(Coin::Dime));
    println!("Quarter cents: {}", value_in_cents(Coin::Quarter(UsState::Texas)));
}
