use std::env;
use std::process::exit;

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

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <op> <text>", args[0]);
        exit(1);
    }
    // for arg in args {
    //     println!("{arg}");
    // }
    let op = &args[1];
    let text = &args[2];
    println!("Op: {}, Text: {}", op, text);

    let mut mut_val = String::from("Initial ");
    mut_val.push_str("value");
    println!("Mutable Val: {}", mut_val);
    let mut_val = &mut_val;
    // mut_val.push_str("!");
    println!("Mutable Val: {}", mut_val);

    let mut val = String::from("Initial value");
    println!("Val: {}", val);
    val = "Hello".to_string();
    let val = val;
    println!("Val: {}", val);

    let res = match op.as_str() {
        "reverse" => text.chars().rev().collect::<String>(),
        "invert" => text
            .chars()
            .map(invert)
            .collect::<String>(),
        "uppercase" => text.to_uppercase(),
        "no-spaces" => text
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>(),
        "leet" => text
            .chars()
            .map(|c| match c {
                'a' | 'A' => '4',
                'e' | 'E' => '3',
                'i' | 'I' => '1',
                'o' | 'O' => '0',
                's' | 'S' => '5',
                't' | 'T' => '7',
                _ => c,
            })
            .collect::<String>(),
        "acronym" => text
            .split_whitespace()
            // .filter_map(|word| word.chars().next())
            // .map(|word| word.chars().next().unwrap())
            .collect::<String>()
            .to_uppercase(),
        _ => {
            eprintln!("Invalid operation: {}", op);
            exit(1);
        }
    };
    println!("Result: {}", res);

    let my_string = String::from("Hello, world!");
    print_my_string(&my_string);
    print_my_string(&my_string);
    let string_ref = &my_string;
    print_my_string(string_ref);

    let x = String::from("2345 US Dollar");
    completely_save_storage(x);
    // ↑ ownership of x is moved into the function completely_save_storage
    // println!("x: {}", x);
    // ↑ this does not compile, as we no longer have the ownership

    let y = String::from("Hello world");
    // ↑ allocate and initialize new string y to
    // "Hello, world!"
    // main() now owns y
    let reference = &y;
    // ↑ create a reference to y
    // main() owns this reference
    // we call this "borrowing y (immutably)"
    takes_reference(reference);
    // ↑ reference is moved into takes_reference();
    // y is freed here
    println!("y: {}", y);
    println!("reference: {}", reference);

    let mut bitcoin = String::from("Bitcoin");
    /* 
    Rust is actually pretty smart,
    so if it sees you are not using mut_ref
    after you have created ro_ref, it will
    destroy it early, this is a relatively
    recent change for ergonomics in Rust
    called Non-Lexical Lifetimes
    */
    let mut_ref = &mut bitcoin;
    /* ↑ borrow bitcoin mutably
    mut_ref is of type `&mut String`,
    given that the variable itself is immutable,
    this corresponds to `char* const ptr` in C */
    // let ro_ref = &bitcoin;
    /* ↑ borrow bitcoin immutably
    this is what makes this example not compile
    as bitcoin is already borrowed mutably */
    // println!("{}", ro_ref);
    mut_ref.push_str(", the cryptocurrency");
    // ↑ use the mutable borrow

}

fn takes_reference(my_ref: &String) {
    // <- reference is moved into this function
    println!("My reference: {}", my_ref);
    // ↑ this macro actually takes all arguments by reference
    // so a &&String is created here, which is moved into the
    // internals of the macro

    // <- my_ref is destroyed here
}

fn print_my_string(string: &String) {
    println!("My string: {}", string);
}

// uncompiled function: need to specify a lifetime explicitly through the 'name syntax
// fn give_me_a_ref<'a>() -> &'a String {
//     let my_string = String::from("Hello, world!");
//     &my_string
// }

fn completely_save_storage(value: String) {

}

fn invert(c: char) -> String {
    if c.is_uppercase() {
        c.to_lowercase().to_string()
    } else {
        c.to_uppercase().to_string()
    }
}
