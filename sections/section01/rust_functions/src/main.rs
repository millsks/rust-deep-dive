fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = implicit return
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let sum = add(3, 4);
    println!("Sum: {}", sum);
    println!("{}", greet("Alice"));
}