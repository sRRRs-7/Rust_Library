
pub fn trait1() {
    let a = 10;
    let mut b = a;
    copy_trait_check(b);
    b = 20;
    assert_eq!(a, 10);
    assert_eq!(b, 20);

    let c = 10;
    let d = &c;
    let e = d;
    copy_trait_check(d);
    assert_eq!(c, 10);
    assert_eq!(*d, 10);
    assert_eq!(*e, 10);

    let s = String::from("hello");  // not implement copy trait because string is primitive type
    assert_eq!(s, "hello");

    let v = 'a';
    copy_trait_check(v);
    assert_eq!(v, 'a');
}

fn copy_trait_check<T: Copy>(_: T) {}