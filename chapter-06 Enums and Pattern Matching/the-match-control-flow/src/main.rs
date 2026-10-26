#[derive(Debug)]
enum usState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(usState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter from {state:?}");
            25
        }
    }
}
// The Option<T> match pattern
fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        None => None,
        Some(val) => Some(val + 1),
    }
}

fn main() {
    println!(
        "The value of a dime in cents is: {} cents",
        value_in_cents(Coin::Dime)
    );

    println!(
        "The value of a quarter is: {} cents",
        value_in_cents(Coin::Quarter(usState::Alabama))
    );

    // The Option<T> pattern match
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
