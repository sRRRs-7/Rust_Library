pub fn borrow_constraint() {
    let mut a = 10;
    let _b = &a;
    let c = &mut a;
    *c = 20;
    assert_eq!(a, 20);
}

pub fn borrow2() {
    let a = 10;
    let mut b = a;
    let c = &mut b;
    *c = 20;
    assert_eq!(b, 20);
}

pub fn borrow3() {
    let mut a = 10;
    let b = &mut a;
    *b = 30;
    assert!(*b == a);
    assert_eq!(a, 30);
}
