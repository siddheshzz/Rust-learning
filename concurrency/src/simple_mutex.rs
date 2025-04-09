use std::sync::{Arc, Mutex};

use crate::prelude::*;

pub fn simple_mutex(){
    // if we try to do this without Arc and Mutex the program will give 0 as output
    // this is because everytime we will spawn a thread it will point to new counter everytime
    // rather we want it to share the memory with other threads
    // thats why we use Arc and mutex to Lock the memory

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num +=1;
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    let result = counter.lock().unwrap();
    println!("Result : {}",*result);
}