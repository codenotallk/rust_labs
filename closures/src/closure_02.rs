fn main() {
    let mut string = String::from("Test");

    let mut fnmut_closure = || {
        println!("{string}");
        string.push_str(" Mutable");
    };

    fnmut_closure();

    println!("{string}");
}
