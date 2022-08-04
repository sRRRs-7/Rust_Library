#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum Option<T> {
    Some(T),
    None,
}

pub fn main() {
    let penny = arm(Coin::Penny);
    let nickel = arm(Coin::Nickel);
    let dime = arm(Coin::Dime);
    let quarter = arm(Coin::Quarter);
    assert_eq!(penny, 1);
    assert_eq!(nickel, 5);
    assert_eq!(dime, 10);
    assert_eq!(quarter, 25);

    let some = Option::Some(3);
    switch(some);
    switch(Option::None);
}

fn arm(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn switch(x: Option<i32>) -> Option<i32> {
    match x {
        Option::Some(x) => Option::Some(x),
        Option::None => Option::None,
    }
}
