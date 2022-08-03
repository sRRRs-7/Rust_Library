
struct Edges {
    width: i32,
    height: i32,
}
struct Rectangle {
    edge: Edges,
}
struct Square {
   edge: Edges,
}
trait Area {
    fn area(&self) -> i32;
    fn can_hold(&self, other:&Edges) -> bool;
}
impl Area for Rectangle {
    fn area(&self) -> i32 {
        self.edge.width * self.edge.height
    }
    fn can_hold(&self, other:&Edges) -> bool {
        self.edge.width > other.width && self.edge.height > other.height
    }
}
impl Area for Square {
    fn area(&self) -> i32 {
        self.edge.width * self.edge.height
    }
    fn can_hold(&self, other:&Edges) -> bool {
        self.edge.width > other.width && self.edge.height > other.height
    }
}

pub fn main() {
    let rectangle = Rectangle{
        edge: Edges { width: 10, height: 8 }
    };
    let area = rectangle.area();
    assert_eq!(area, 80);

    let square = Square{
        edge: Edges { width: 7, height: 7 }
    };
    let area = square.area();
    assert_eq!(area, 49);

    if rectangle.can_hold(&square.edge) {
        println!("rectangle can hold square");
    }
}

