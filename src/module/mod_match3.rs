
#[derive(Debug)]
struct Sea {
    water: String,
    creature: String,
    seaweed: String,
}

pub fn main() {
    let s: Option<String> = Some(String::from("coffee"));
    match s {
        Some(ref v) => println!("{}", v),
        None => (),
    }
    println!("{:?}", s);

    main2();
    substitute();
}

// refactoring
fn main2() {
    let s = Some(3);
    if let Some(3) = s {
        println!("{:?}", s);
    }
}

fn substitute() {
    let sea = Sea {
        water: String::from("coffee"),
        creature: String::from("coffee"),
        seaweed: String::from("coffee"),
    };

    let Sea{ref water, ref creature, ref seaweed} = sea;
    println!("{:?}", sea);
    assert_eq!(water, &sea.water);
    assert_eq!(creature, &sea.creature);
    assert_eq!(seaweed, &sea.seaweed);
}