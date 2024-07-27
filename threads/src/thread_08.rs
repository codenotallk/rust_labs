use std::{sync::{Arc, RwLock}, thread};

struct Repository {
    counter_a: u32,
    counter_b: u32,
}

fn main() {

    let counter = Arc::new(RwLock::new(Repository {counter_a: 0, counter_b: 0}));

    let counter_a = Arc::clone(&counter);
    let counter_b = Arc::clone(&counter);

    thread::spawn(move || loop {
        
        let mut repository = counter_a.write().unwrap();
        repository.counter_a += 1;
        
    });

    thread::spawn(move || loop {
        
        let mut repository = counter_b.write().unwrap();
        repository.counter_b += 1;
        
    });

    loop {
        let counter = counter.read().unwrap();
        println!("A: {}, B: {}", counter.counter_a, counter.counter_b);
    }
}
