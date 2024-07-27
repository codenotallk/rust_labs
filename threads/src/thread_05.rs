use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

fn main() {
    let (tx, rx): (Sender<u32>, Receiver<u32>) = mpsc::channel();

    thread::spawn(move || {
        let mut counter = 0u32;

        loop {
            tx.send(counter).unwrap();

            thread::sleep(Duration::from_secs(1));
            counter += 1;
        }
    });

    loop {
        match rx.recv_timeout(Duration::from_millis(500)) {
            Ok(counter) => println!("{counter}"),
            Err(_) => println!("Processing other stuff"),
        }
    }
}
