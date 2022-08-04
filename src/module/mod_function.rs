pub fn main() {
    let args = (1, 2);
    assert_eq!(coordinate(args), 2);

    let args2 = (3, 3);
    let multi = coordinate(args2);
    assert_eq!(multi, 9)
}

fn coordinate((n1, n2): (i32, i32)) -> i32 {
    n1 * n2
}
