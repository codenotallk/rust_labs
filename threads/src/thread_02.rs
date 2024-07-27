use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| loop {
        println!("Hello World");
        thread::sleep(Duration::from_secs(1));
    });

    handle.join().unwrap();
}
