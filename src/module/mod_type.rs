pub fn type1() {
    let (a, b, c) = (1, 2, 3);
    let [x, y, z] = [10, 20, 30];
    println!("{}, {}, {}, {}, {}, {}", a, b, c, x, y, z);
}

// shadowing
pub fn shadowing() {
    let a = 10;
    {
        let mut a = 10;
        a += 40;
        assert_eq!(a, 50);
    }
    assert_eq!(a, 10);
}

// type convert
pub fn type_as() {
    let num1 = 12u32;
    let num2 = 6u8;
    let sum = num1 + num2 as u32;
    assert_eq!(sum, 18);
}
