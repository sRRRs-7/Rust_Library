pub fn closure() {
    let c = |x| x + 1;
    let n = c(2);
    assert_eq!(n, 3);

    // move ownership
    let x = 2;
    let b = move |x| x * x;
    let num = b(x);
    assert_eq!(num, 4);

    let mut a = 2;
    let z = &mut a;
    *z = 5;
    println!("{}", a);
}
