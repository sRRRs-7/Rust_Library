#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    email: String,
    active: bool,
}

#[derive(Debug)]
struct Number(i32, i32, i32);

pub fn main() {
    let user = User {
        name: String::from("John"),
        age: 12,
        email: String::from("johnjohn@email.com"),
        active: true,
    };
    let name = String::from("michel");
    let age = 45;
    let email = String::from("michel@email.com");
    let user2 = user_constructor(name, age, email);
    println!("{} {} {} {}", user.name, user.age, user.email, user.active);
    println!(
        "{} {} {} {}",
        user2.name, user2.age, user2.email, user2.active
    );

    let user3 = emit_constructor(user2);
    println!(
        "{} {} {} {}",
        user3.name, user3.age, user3.email, user3.active
    );

    // tuple
    imp_tuple()
}

fn user_constructor(name: String, age: i32, email: String) -> User {
    User {
        name,
        age,
        email,
        active: true,
    }
}

fn emit_constructor(u: User) -> User {
    User { ..u }
}

fn imp_tuple() {
    let tuple = Number(1, 2, 3);
    let Number(x, y, z) = Number(10, 20, 30);

    println!("{} {} {}", tuple.0, tuple.1, tuple.2);
    println!("{} {} {}", x, y, z);
}
