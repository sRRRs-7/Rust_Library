
fn add_one(a: &mut Vec<i32>) {
    for i in 0..a.len() {
        a[i] += 1;
    }
}

pub fn main() {
    let mut a = vec![0, 1, 2];
    add_one(&mut a);
    println!("{} {} {}", a[0], a[1], a[2]);
}