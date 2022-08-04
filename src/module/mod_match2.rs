#[derive(Debug)]
struct Fruits {
    apple: Colors,
    banana: Colors,
    peach: Colors,
    strawberry: Colors,
}
#[derive(Debug)]
enum Colors {
    Red(String),
    Yellow(String),
    Pink(String),
    Berry(String),
}

impl Fruits {
    fn new(apple: Colors, banana: Colors, peach: Colors, strawberry: Colors) -> Self {
        Self {
            apple,
            banana,
            peach,
            strawberry,
        }
    }
}

pub fn main() {
    let fruit = Fruits::new(
        Colors::Red(String::from("red")),
        Colors::Yellow(String::from("yellow")),
        Colors::Pink(String::from("pink")),
        Colors::Berry(String::from("berry")),
    );

    let color = fruit.apple;
    match color {
        Colors::Red(_) => println!("apple"),
        Colors::Yellow(_) => println!("banana"),
        Colors::Pink(_) => println!("peach"),
        Colors::Berry(_) => println!("strawberry"),
    }

    println!("{:?}", fruit.banana);
    println!("{:?}", fruit.peach);
    println!("{:?}", fruit.strawberry);
}
