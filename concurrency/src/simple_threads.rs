use std::thread::sleep;

use crate::prelude::*;
pub fn simpleThreads() {
    let mut threads = vec![];
    for i in 0..10 {
        let th = thread::spawn(move|| {
            sleep(Duration::from_millis(100));
            println!("Hello, world! from {}",i);
        });
        threads.push(th)
    }

    for t in threads{
        t.join();
    }
    println!("Main thread");
}