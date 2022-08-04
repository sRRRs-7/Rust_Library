pub fn main() {
    // String
    let mut s = String::new();
    println!("{}", s);
    s = String::from("good code");
    assert_eq!(s, "good code");

    let s1 = String::from("100");
    let num: i32 = s1.parse().unwrap();
    assert_eq!(num, 100);

    // to_string
    let num2 = 107;
    let num2_str = num2.to_string();
    assert_eq!(num2_str, "107");

    // parse
    let s2 = String::from("77");
    let num3 = s2.parse::<i32>().expect("error");
    assert_eq!(num3, 77);

    // format!
    let s4 = String::from("Rust is");
    let s5 = String::from("very safety");
    let fmt = format!("{} {}", s4, s5);
    assert_eq!(fmt, "Rust is very safety");
    println!("{}", fmt);

    // methods
    let mut st = String::from("Rust");
    let st2 = String::from("Go");

    let stg = st.find('s');
    assert_eq!(stg, Some(2));

    let fi = st.rfind('R');
    println!("{:?}", fi);

    st.push('1');
    assert_eq!(st, "Rust1");

    st.push_str(&st2);
    assert_eq!(st, "Rust1Go");

    let p = st.pop().unwrap();
    assert_eq!(p, 'o');
    assert_eq!(st, "Rust1G");

    let b = st.as_bytes();
    println!("{:?}", b);

    st.truncate(4);
    assert_eq!(st, "Rust");

    st.insert(1, 'r');
    assert_eq!(st, "Rrust");

    st.insert_str(4, &st2);
    assert_eq!(st, "RrusGot");

    let stg2 = st.replace("Go", "");
    println!("{}", stg2);
    assert_eq!(stg2, "Rrust");

    st.remove(4);
    st.remove(4);
    st.remove(0);
    assert_eq!(st, "rust");

    let cnt = st.len();
    assert_eq!(cnt, 4);

    let mut sh = st.chars();
    assert_eq!(Some('r'), sh.next());

    let by = st.bytes();
    println!("{:?}", by);

    assert!(st.starts_with("ru"));
    assert!(st.ends_with("st"));

    let st7 = String::from("Nodejs      ");
    let st8 = st7.trim();
    println!("{:?}", st7);
    println!("{:?}", st8);

    st.clear();
    assert!(st.is_empty());

    println!("{:?}", st);
}
