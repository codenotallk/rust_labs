use std::{thread, time::Duration};

#[derive(Debug)]
struct User {
    active: bool,
    username: &'static str,
    password: &'static str,
    login_count: u64,
}

const USER: User = User {
    active: true, 
    username: "user",
    password: "password",
    login_count: 10u64,
};

fn main() {
    let handle = thread::spawn(|| {
        println!("{:?}", USER);
    });

    let handle2 = thread::spawn(|| {
        println!("{:?}", USER);
    });

    handle.join().unwrap();
    handle2.join().unwrap();
}
