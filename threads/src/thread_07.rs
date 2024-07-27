use std::{sync::{Arc, Mutex}, thread};

struct Repository {
    counter_a: u32,
    counter_b: u32,
}

fn main() {

    let counter = Arc::new(Mutex::new(Repository {counter_a: 0, counter_b: 0}));

    let counter_a = Arc::clone(&counter);
    let counter_b = Arc::clone(&counter);

    thread::spawn(move || loop {
        
        let mut repository = counter_a.lock().unwrap();
        repository.counter_a += 1;
        
    });

    thread::spawn(move || loop {
        
        let mut repository = counter_b.lock().unwrap();
        repository.counter_b += 1;
        
    });

    loop {
        let counter = counter.lock().unwrap();
        println!("A: {}, B: {}", counter.counter_a, counter.counter_b);
    }
}
