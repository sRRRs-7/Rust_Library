pub fn main() {
    let num = Some(7);
    match num {
        Some(v) if v == 0 => println!("v is 0"),
        Some(v) if v == 7 => println!("v is 7"),
        Some(v) => println!("v is {}", v),
        None => (),
    };
}
