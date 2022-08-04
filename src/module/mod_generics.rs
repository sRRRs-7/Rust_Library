use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn main() {
    let point = Point { x: 2, y: 1 };
    let sum = point.add(point);

    assert_eq!(sum.x, 4);
    assert_eq!(sum.y, 2);
}
