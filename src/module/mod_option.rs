
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

pub fn main() {
    let num = Option::Some(1);
    let s = Option::Some("linux");
    let absent: Option<i32> = Option::None;

    println!("{:?} {:?} {:?}", num, s, absent);
}