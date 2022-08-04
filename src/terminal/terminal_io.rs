use std::io::{stdin, stdout, Write};

pub fn main() {
    let mut s = String::new();
    print!("enter text: ");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("invalid input");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    };
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    };
    println!("{}", s);
}
