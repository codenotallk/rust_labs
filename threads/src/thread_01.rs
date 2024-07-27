use std::{thread, time::Duration};

fn main() {
    thread::spawn(|| loop {
        println!("Hello World");
        thread::sleep(Duration::from_secs(1));
    })
    .join()
    .unwrap();
}
