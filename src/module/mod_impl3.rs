struct Space {
    x: f64,
    y: f64,
    z: f64,
}

// constructor -> initialize function
impl Space {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

pub fn main() {
    let space = Space::new(3.0, 5.0, 2.0);
    assert_eq!(space.x, 3.0);
    assert_eq!(space.y, 5.0);
    assert_eq!(space.z, 2.0);
}
