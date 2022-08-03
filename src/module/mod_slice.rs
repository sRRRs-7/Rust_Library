
pub fn slice() {
    let arr = [1, 2, 3, 4, 5];
    let get_arr = &arr[1..=3];
    println!("{:#?}", get_arr);
    println!("{:?}", get_arr);
    dbg!(get_arr);
}

pub fn sequence() {
    let mut a = 0;
    for i in 1..=100 {
        a += i
    }
    assert_eq!(a, 5050);
}

pub fn extract() {
    let s = String::from("coffee");
    let slice = &s[..];
    println!("{:?}", slice)
}

