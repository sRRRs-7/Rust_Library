
#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

#[derive(Debug)]
struct IpAddress {
    version: IpAddrKind,
    address: String,
}

impl IpAddress {
    fn new(version: IpAddrKind, address: String) -> Self {
        Self{version, address}
    }
}

pub fn main() {
    let ip4 = IpAddress::new(
        IpAddrKind::V4(String::from("4")),
        String::from("127.0.0.1")
    );
    let ip6 = IpAddress::new(
        IpAddrKind::V6(String::from("6")),
        String::from("::1")
    );

    println!("{:?} {:?}", ip4.version, ip4.address);
    println!("{:?} {:?}", ip6.version, ip6.address);

    switch()
}

enum Foo {
    _Bar,
    Baz(i32),
}

fn switch() {
    let foo = Foo::Baz(1);
    match foo {
        Foo::Baz(i) => println!("{}", i),
        _ => {}
    };
}