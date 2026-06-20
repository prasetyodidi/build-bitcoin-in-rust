fn main() {
    println!("Hello, world!");
    let my_name: &str = "John";
    println!("My name is: {}!", my_name);

    // This is single-line comment
    /*
        This is multi-line comments
        Multiline comments can be nested
    */

    let mut mutable_name: &str = "Alice";
    println!("Mutable name: {}", mutable_name);
    mutable_name = "Bob";
    println!("Mutable name \
            after \
            change: {}", mutable_name);
}
