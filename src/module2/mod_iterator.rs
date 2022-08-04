pub fn main() {
    let arr1 = [1, 2, 3];
    let arr2 = [3, 4, 5];

    // sum
    let sum: i32 = arr1.iter().sum();
    assert_eq!(sum, 6);

    // zip
    let mut iter = arr1.iter().zip(arr2.iter());
    println!("{:?}", iter.next());
    println!("{}", iter.count());

    // chain
    let iter2 = arr1.iter().chain(arr2.iter());
    println!("{:?}", iter2);
    println!("{}", iter2.count());

    // map
    let mut m = arr1.iter().map(|x| *x + 2);
    println!("{:?}", m.next());
    println!("{}", m.count());

    // filter
    let arr3 = [7, 8, 9];
    let f: Vec<&i32> = arr3.iter().filter(|&x| *x > 7).collect();
    println!("{:?}", f);
    println!("{}", f.iter().count());

    // fold
    let sum = arr1.iter().fold(0, |acc, x| acc + x);
    println!("{}", sum);

    // collect
    let arr4 = [10, 20, 30];
    let c: Vec<i32> = arr4.iter().map(|x| x * 2).collect();
    println!("{:?}", c);

    // enumerate
    let arr5 = [1, 3, 5, 7, 11];
    let mut enu = arr5.iter().enumerate();
    println!("{:?}", enu.next());

    // inspect
    let arr6 = [1, 3, 5, 7, 11];
    let sum = arr6
        .iter()
        .cloned()
        .inspect(|x| {
            println!("{:?}", x);
        })
        .fold(0, |sum, i| sum + i);
    println!("{:?}", sum);

    // for
    for v in arr6.iter().enumerate() {
        println!("{:?} {} {}", v, v.0, v.1);
    }
}
