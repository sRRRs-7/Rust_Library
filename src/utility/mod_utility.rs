use std::fmt;
use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, point2: Point) -> Point {
        Point {
            x: self.x + point2.x,
            y: self.y + point2.y,
            z: self.z + point2.z,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

// alias type
type Num32 = i32;

pub fn main() {
    // separate substitute
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    };
    assert_eq!(origin.y, origin.z);

    // ops
    let p = Point::new(1, 2, 3);
    let v = p.add(Point { x: 4, y: 5, z: 6 });
    println!("v is {:?}", v);

    // alias type
    let num: Num32 = 3;
    println!("{}", num);

    // dbg!
    let a = 5;
    let b = dbg!(a * 2) + 1;
    assert_eq!(b, 11);

    // Debug & Display
    let point = Point::new(10, 20, 30);
    println!("{}", point);

    // Some
    let s = Some(10);
    if let Some(x) = s {
        dbg!(x);
    }

    // loop & while label
    let mut count = 0;
    'outer: loop {
        'inner: while {
            count += 1;
            if count > 5 {
                break 'outer;
            } else if count == 20 {
                break 'inner;
            }
            count > 10
        } {}
    }
    dbg!(count);
}
