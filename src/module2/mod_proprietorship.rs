use std::cell::{Cell, RefCell};
use std::rc::Rc;

// proprietorship duplicate
pub fn main() {
    // Rc put data in heap memory
    let a = Rc::new(10);
    let b = Rc::clone(&a);
    assert_eq!(a, b);
    dbg!(Rc::strong_count(&a));
    dbg!(Rc::strong_count(&b));

    // internal mutability
    let c = Cell::new(100);
    dbg!(c.get());
    c.set(200);
    dbg!(c.get());

    let d = Cell::new(1000);
    let e = d.replace(3000);
    dbg!(d.get());
    dbg!(e);

    let f = d.into_inner();
    dbg!(e);
    dbg!(f);

    // Rc & Cell impl
    let g = Rc::new(Cell::new(100));
    g.set(200);
    dbg!(g.get());

    let h = Rc::clone(&g);
    h.set(300);
    dbg!(g.get());
    dbg!(h.get());

    // borrow ref & ref mut
    let st = Rc::new(RefCell::new(String::from("Mac Dictionary")));
    dbg!(st.borrow());

    *(st.borrow_mut()) = String::from("linux");
    dbg!(st.borrow());

    let st2 = Rc::clone(&st);
    *(st2.borrow_mut()) = String::from("windows");
    dbg!(st2.borrow());
}
