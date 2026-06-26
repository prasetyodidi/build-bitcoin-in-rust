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

    let my_ref_hello = my_ref();
    println!("My reference: {}", my_ref_hello);

    // create string slice from other string slice without copying data by re-borrowing
    let my_str = "Hello, world!";
    let hell = &my_str[0..4];
    println!("{} {}", my_str, hell);

    // this does not compile because string literals are always immutable
    // let mut_str: &mut str = "Hello, world!";

    let my_string = String::from("Hello, world!");
    // both work
    my_fun1(&my_string);
    my_fun2(&my_string);
    // however this wouldn't work
    // my_fun1("Hello!"); <- type mismatch, expected `&String`, got `&str`

    let owned_string = String::from("I'm not static, but i'm owned");
    print_static(owned_string);

    let text = String::from("Hello, world!");
    let holder = Holder { reference: &text };
    println!("{}", holder.reference);

    let x = String::from("Hello, world!");
    {
        let y = x;
        println!("y: {}", y);
    }
    // y goes out of scope here, meaning it is dropped, and
    // deallocated
    // println!("x: {}", x); Error: x was moved
    let a = 5; // i32 implements Copy, and so it can be copied
    {
        let b = a; // a is copied to b
        println!("b: {}", b);
    }
    println!("a: {}", a); // a is still valid

    let outer;
    {
        let owned = String::from("Hello, world!");
        let reference = &owned;
        outer = reference; // Error: `owned` does not live long enough
        println!("{}", reference);
    } // `owned` is dropped here
    // println!("{}", outer); // Would not compile
    let mut ref1 = &5;
    {
        let x = 10;
        ref1 = &x; // ref1 now points to x
        println!("{}", ref1);
    } // x is dropped here, ref1 is now dangling
    // println!("{}", ref1); // Would not compile

    let hello = Hello;
    (&hello).say_hi();
    (&&hello).say_hi();
    hello.say_hi();

    let mut broken = MyStruct {
        remainder: Some("Hello")
    };
    for _ in 0..5 {
        println!("{:?}", broken.pop_first_char_as_string());
    }

    if let Some(val) = Some(95) {
        println!("Value: {}", val);
    } 
    let (first, second) = (1, 2);
    println!("First: {}, Second: {}", first, second);

    let a_pair: (Option<i32>, Result<(), MyErrorEnum>) = (
        Some(45),
        Err(MyErrorEnum::Other(String::from("Database connection lost")))
    );
    if let (Some(45), Err(MyErrorEnum::Other(err))) = a_pair {
        println!("error message: {}", err);
    } else {
        println!("Doesnt match!");
    }

}

enum MyErrorEnum {
    TimeOut,
    NotFound,
    Other(String)
}

// in the business, we call this foreshadowing ;-)
struct MyStruct<'a> {
    remainder: Option<&'a str>,
}
// ↑ here, we have created a struct which holds
// a reference to an Option of a string slice.
//
// string slices without the 'static lifetime are views
// into strings, and cannot outlive them, hence the
// lifetime parameter
// impl blocks are used to create methods for types,
// we need to respect the generic lifetime parameter
impl<'a> MyStruct<'a> {
    // this will keep returning the first character
    fn pop_first_char_as_string(&mut self) -> Option<&str> {
        // surprise! remainder here gets copied,
        // so we are not modifying which pointer is
        // stored in self, but only a copy on the stack
        let remainder = &mut self.remainder?;
        let c = &remainder[0..1];
        if remainder.len() != 1 {
            *remainder = &remainder[1..];
            Some(c)
        } else {
            self.remainder.take()
        }
    }
}

trait SayHi {
    fn say_hi(self);
}

struct Hello;

impl SayHi for Hello {
    fn say_hi(self) {
        println!("This hi! will cost me my life - I'm owned value");
    }
}

impl SayHi for &Hello {
    fn say_hi(self) {
        println!("Hi, I am a reference to Hello!");
    }
}

impl SayHi for &&Hello {
    fn say_hi(self) {
        println!("Hi, I am a double reference to Hello!");
    }
}

struct Holder<'a> {
    reference: &'a str,
}

fn print_static<T: 'static + std::fmt::Debug>(value: T) {
    println!("Value: {:?}", value);
}

// don't
fn my_fun1(_input: &String) {}
// do
fn my_fun2(_input: &str) {}

fn my_ref() -> &'static str {
    "Hello, world!"
}

/*
fn my_ref1() -> &str {
    let my_string = String::from("Hello, world!");
    &my_string
    // ↑ doesn't compile, return value references temporary value
}
 */

/*
fn my_ref2(input: &String) -> &str {
    &input
    // ↑ compiles, since input is known to live longer than the span of this function
}
*/

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
