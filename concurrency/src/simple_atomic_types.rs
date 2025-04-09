// Atomics types
// what & why
// enable thread safe operations without lock using cpu level instructions
// memory ordering option defining how and when memory update become available/visible to other threads

use std::sync::{atomic::AtomicU32, Arc, Mutex};

use crate::prelude::*;

pub fn simple_atomic_types(){
    // if we try to do this without Arc and Mutex the program will give 0 as output
    // this is because everytime we will spawn a thread it will point to new counter everytime
    // rather we want it to share the memory with other threads
    // thats why we use Arc and mutex to Lock the memory

    let counter = Arc::new(AtomicU32::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("Result : {}",counter.load(std::sync::atomic::Ordering::SeqCst));
}
