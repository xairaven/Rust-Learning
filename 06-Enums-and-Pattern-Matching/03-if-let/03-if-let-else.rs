enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);

    let mut count = 0;
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    count = 0;
    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}