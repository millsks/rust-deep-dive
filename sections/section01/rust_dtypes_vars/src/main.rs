fn main() {
    let x = 5;          // immutable variable
    let mut y = 10;     // mutable variable
    y += 1;             // modifying mutable variable

    // Shadowing - redeclareing a variable with the same name
    let x = x + 1;      // x is now 6

    println!("x = {}, y = {}", x, y);

    // Basic data types
    let integer: i32 = -42;             // 32-bit signed integer
    let unsigned: u64 = 1_000_000;      // 64-bit unsigned integer
    let float: f64 = 3.14159;           // 64-bit floating point
    let boolean: bool = true;           // boolean type
    let character: char = '🦀';         // character type
    let text: &str = "Hello";           // string slice
    let owned_string: String = String::from("Hello, owned String!");    // owned string

    println!("Integer: {}, Unsigned: {}, Float: {}, Boolean: {}, Character: {}, Text: {}, Owned String: {}",
             integer, unsigned, float, boolean, character, text, owned_string);
}