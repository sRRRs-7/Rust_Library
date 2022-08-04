pub fn syntax() {
    // while condition not return
    let mut n = 3;
    while n != 10 {
        n += 1;
    }
    assert_eq!(n, 10);

    // for iter
    let arr = [1, 2, 3, 4, 5];
    let mut sum = 0;
    for v in arr {
        sum += v
    }
    assert_eq!(sum, 15);

    // loop can return
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    assert_eq!(count, 10);
    assert_eq!(result, 20);
}
