struct Quarter {
    width: u32,
    height: u32,
}

struct Triangle {
    bottom: u32,
    height: u32,
}

trait Geometry {
    fn new(x: u32, y: u32) -> Self;
    fn name(&self) -> &str {
        "Geometry"
    }
    fn area(&self) -> u32;
    fn size_up(&self) -> u32;
}

impl Geometry for Quarter {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    fn name(&self) -> &str {
        "Quarter"
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn size_up(&self) -> u32 {
        self.width * self.height * 2
    }
}

impl Geometry for Triangle {
    fn new(bottom: u32, height: u32) -> Self {
        Self { bottom, height }
    }
    fn name(&self) -> &str {
        "Triangle"
    }
    fn area(&self) -> u32 {
        self.bottom * self.height
    }
    fn size_up(&self) -> u32 {
        self.bottom * self.height * 2
    }
}

pub fn main() {
    let qua = Quarter::new(5, 5);
    let tri = Triangle::new(3, 7);

    let name = qua.name();
    let area = qua.area();
    let size = qua.size_up();
    println!("{} {} {}", name, area, size);

    let name2 = tri.name();
    let area2 = tri.area();
    let size2 = tri.size_up();
    println!("{} {} {}", name2, area2, size2);
}
