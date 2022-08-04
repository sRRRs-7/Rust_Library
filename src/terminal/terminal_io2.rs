use std::io;

enum Option<T> {
    None,
    Some(T),
}

fn safe_div(n: i32, d: i32) -> Option<i32> {
    if d == 0 {
        return Option::None;
    }
    Option::Some(n / d)
}

pub fn main() {
    println!("Please input your numerator.");
    let mut numerator = String::new();
    io::stdin().read_line(&mut numerator).unwrap();

    println!("Please input your denominator.");
    let mut denominator = String::new();
    io::stdin().read_line(&mut denominator).unwrap();

    let num = numerator.trim().parse().expect("Wrong number format!");

    let deno = numerator.trim().parse().expect("Wrong number format!");

    match safe_div(num, deno) {
        Option::None => println!("Can't divide by zero!"),
        Option::Some(v) => println!("Quotient is {}", v),
    }
}
