
// heap memory allocation
pub fn main() {
    // allocate heap memory
    let mem = Box::new(10);
    let mem2 = Box::<i32>::new(20);

    // heap -> stack
    let stack = 30;
    let heap = Box::new(stack);
    let deref = *heap;

    println!("{} {} {}", mem, mem2, deref);
}