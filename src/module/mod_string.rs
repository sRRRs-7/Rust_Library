pub fn str_lib() {
    let s = String::from("starbucks");
    let l = s.len();
    println!("length; {}", l);

    // bool -> integer -> 0 or 1
    let t = true;
    println!("bool: {}", t as u8)
}
