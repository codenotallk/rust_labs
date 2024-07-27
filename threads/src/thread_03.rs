use std::{thread, time::Duration};

fn main() {
    let mut handles = Vec::new();

    for i in 0..10 {
        let handle = thread::spawn(move || {
            println!("Hello World {i}");
            thread::sleep(Duration::from_millis(100));
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
