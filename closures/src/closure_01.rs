fn main() {
    let string = String::from("Test");

    let fn_closure = || {
        println!("{string}");
    };

    fn_closure();

    println!("{string}");
}
