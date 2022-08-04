
#[cfg(test)]
#[test]
fn equal(){
    assert_eq!(2 * 2, 4);
}

#[test]
#[should_panic]
fn error() {
    panic!()
}