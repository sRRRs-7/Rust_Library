use std::fmt;
// unit like struct
struct Password(String);

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.chars().map(|_| '*').collect::<String>())
    }
}

pub fn main() {
    let p = String::from("1234567");
    assert_eq!(p, "1234567");

    let p2 = Password(String::from("1234567"));
    println!("{}", p2);
}
