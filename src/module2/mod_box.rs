
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn main() {
    let list = Cons(2, Box::new(Cons(5, Box::new(Cons(3, Box::new(Nil))))));
    print_list(&list);

    let list2 = append(&list, 7);
    print_list(&list2);
}

fn print_list(list: &List) {
    match list {
        Cons(val, ls) => {
            println!("val: {}", val);
            print_list(ls);
        }
        Nil => {}
    }
}

fn append(list: &List, val: i32) -> List {
    match list {
        Cons(x, ls) => {
            Cons(*x, Box::new(append(ls, val)))
        },
        Nil => {
            Cons(val, Box::new(Nil))
        }
    }
}