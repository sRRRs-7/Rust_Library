use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn main() {
    let concurrent = thread::spawn(|| {
        for i in 1..10 {
            println!("thread2: {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 1..5 {
        println!("thread1: {}", i);
        thread::sleep(Duration::from_secs(2));
    }
    // sync await
    concurrent.join().unwrap();

    // mutex
    control();

    // arc
    arc();
}

fn control() {
    let a = Mutex::new(5);
    {
        let mut b = a.lock().unwrap();
        *b = 10;
        println!("mutex: {:?}", b);
    }
    println!("mutex: {:?}", a);
}

// thread share
fn arc() {
    let counter = Arc::new(Mutex::new(0));
    let (send, recv) = channel();
    {
        for _ in 1..=7 {
            let (counter, send) = ((Arc::clone(&counter)), send.clone());
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
                if *num == 7 {
                    send.send(()).unwrap();
                };
                println!("number: {}", num);
            });
            handle.join().unwrap();
        }
    }
    recv.recv().unwrap();
    println!("counter: {:?}", counter);
}
