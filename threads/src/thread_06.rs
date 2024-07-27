use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

enum MessageType {
    MessageString,
    MessageInteger,
}

struct Message {
    message_type: MessageType,
    data: Vec<u8>,
}

fn main() {
    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || loop {
        tx1.send(Message {
            message_type: MessageType::MessageString,
            data: "Hello World".as_bytes().to_vec(),
        })
        .unwrap();
        thread::sleep(Duration::from_secs(1));
    });

    thread::spawn(move || {
        let mut counter = 0u32;

        loop {
            let data = counter.to_be_bytes().to_vec();
            tx.send(Message {
                message_type: MessageType::MessageInteger,
                data,
            })
            .unwrap();
            thread::sleep(Duration::from_secs(1));

            counter += 1;
        }
    });

    loop {
        match rx.recv_timeout(Duration::from_millis(500)) {
            Ok(message) => {
                match message.message_type {
                    MessageType::MessageString => {
                        let data = String::from_utf8(message.data).unwrap();
                        println!("{data}");
                    },
                    MessageType::MessageInteger => {
                        let bytes: [u8; 4] = message.data.try_into().unwrap();
                        let value = u32::from_be_bytes(bytes);
                        println!("{value}");
                    },
                }
            },
            Err(_) => println!("Processing other stuff"),
        }
    }
}
