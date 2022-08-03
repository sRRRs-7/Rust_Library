
pub fn new_str(s: &str) -> String {
    String::from(s)
}

pub fn convert_int(s: String) -> i32 {
    let num: i32 = s.trim().parse().expect("Wrong number format!");
    return num;
}

pub fn convert_int2(s: String) -> i32 {
    let num: i32 = s.parse().unwrap();
    return num;
}

pub fn main() {
    let s = new_str("2");
    let s2 = new_str("3");
    let n = convert_int(s);
    let n2 = convert_int2(s2);

    assert_eq!(n, 2);
    assert_eq!(n2, 3);
}