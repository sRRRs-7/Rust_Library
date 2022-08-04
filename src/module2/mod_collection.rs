pub fn main() {
    // Vec
    let mut arr: Vec<i32> = Vec::new();
    arr.push(1);
    arr.push(3);
    arr.push(5);
    println!("{:?}", arr);

    // initialize
    let mut init = vec![1, 2, 3];
    init.push(4);
    println!("{:?}", init);

    for i in &init {
        println!("{:?}", i);
    }

    // Vec methods
    let mut arr3 = vec![1, 2, 3, 4];
    let mut arr4 = vec![1, 2, 3, 4];
    arr3.push(5);
    arr3.append(&mut arr4);
    arr3.insert(0, 7);
    arr3.sort();
    arr3.remove(1);
    arr3.pop();
    println!("{:?}", arr3);

    let l = arr3.len();
    let first = arr3.first();
    let last = arr3.last();
    if let Some((last, elements)) = arr3.split_last() {
        println!("split last: {:?}", last);
        println!("split rest elements: {:?}", elements);
    }
    println!("first: {:?}", first);
    println!("last: {:?}", last);
    println!("array length: {}", l);

    arr3.clear();
    assert!(arr3.is_empty());
}
