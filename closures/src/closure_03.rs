fn main() {
    let string = String::from("Test");

    let fnonce_closure = move || {
        println!("{string}");
    };

    fnonce_closure();

    // println! ("{string}");
}
