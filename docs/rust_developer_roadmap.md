# 🦀 The Complete Rust Developer Roadmap
### From Zero to Expert — A Hands-On Learning Journey

---

## Table of Contents

1. [Section 1: Getting Started with Rust](#section-1-getting-started-with-rust)
2. [Section 2: Ownership, Borrowing & Lifetimes](#section-2-ownership-borrowing--lifetimes)
3. [Section 3: Structs, Enums & Pattern Matching](#section-3-structs-enums--pattern-matching)
4. [Section 4: Error Handling](#section-4-error-handling)
5. [Section 5: Collections & Iterators](#section-5-collections--iterators)
6. [Section 6: Traits & Generics](#section-6-traits--generics)
7. [Section 7: Closures & Functional Programming](#section-7-closures--functional-programming)
8. [Section 8: Modules, Crates & Cargo](#section-8-modules-crates--cargo)
9. [Section 9: Working with Files](#section-9-working-with-files)
10. [Section 10: Concurrency & Async Programming](#section-10-concurrency--async-programming)
11. [Section 11: Advanced Rust — Macros, Unsafe & FFI](#section-11-advanced-rust--macros-unsafe--ffi)
12. [Capstone Project: Real-World CLI Task Manager](#capstone-project-real-world-cli-task-manager)

---

## Section 1: Getting Started with Rust

### 1.1 What is Rust?

Rust is a systems programming language focused on three goals: **safety**, **speed**, and **concurrency**. Unlike C or C++, Rust guarantees memory safety without a garbage collector, using a unique ownership model enforced at compile time.

**Why learn Rust?**
- No null pointer exceptions or dangling pointers
- No garbage collector — predictable performance
- Fearless concurrency — the compiler prevents data races
- Growing ecosystem used by Mozilla, Microsoft, Amazon, and Linux kernel developers

---

### 1.2 Installing Rust

Rust is installed via `rustup`, the official toolchain manager.

```bash
# Install rustup (Linux/macOS)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows: Download and run rustup-init.exe from https://rustup.rs

# Verify installation
rustc --version
cargo --version
```

---

### 1.3 Your First Rust Program

```rust
fn main() {
    println!("Hello, Rustacean!");
}
```

Compile and run:

```bash
rustc main.rs
./main
```

Or use Cargo (recommended):

```bash
cargo new hello_rust
cd hello_rust
cargo run
```

---

### 1.4 Variables, Mutability & Data Types

By default, variables in Rust are **immutable**. You must explicitly opt into mutability.

```rust
fn main() {
    let x = 5;          // immutable
    let mut y = 10;     // mutable
    y += 1;

    // Shadowing — redeclare with the same name
    let x = x + 1;     // x is now 6

    println!("x = {}, y = {}", x, y);

    // Basic types
    let integer: i32 = -42;
    let unsigned: u64 = 1_000_000;
    let float: f64 = 3.14159;
    let boolean: bool = true;
    let character: char = '🦀';
    let text: &str = "Hello";
    let owned_string: String = String::from("World");

    println!("{} {} {} {} {} {} {}", integer, unsigned, float, boolean, character, text, owned_string);
}
```

---

### 1.5 Control Flow

```rust
fn main() {
    let number = 7;

    // if/else
    if number < 5 {
        println!("Less than 5");
    } else if number == 7 {
        println!("It's seven!");
    } else {
        println!("Something else");
    }

    // if as an expression
    let description = if number % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", number, description);

    // loop
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 5 {
            break count * 2;
        }
    };
    println!("Loop result: {}", result);

    // while
    let mut n = 3;
    while n > 0 {
        println!("{}!", n);
        n -= 1;
    }

    // for with ranges
    for i in 1..=5 {
        println!("i = {}", i);
    }

    // for over a collection
    let fruits = ["apple", "banana", "cherry"];
    for fruit in fruits.iter() {
        println!("Fruit: {}", fruit);
    }
}
```

---

### 1.6 Functions

```rust
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
```

---

### 🛠️ Mini Project 1: Temperature Converter

Build a command-line temperature converter that converts between Celsius, Fahrenheit, and Kelvin.

**Requirements:**
- Accept a temperature value and unit from the user via `stdin`
- Convert to the other two units
- Display results formatted to 2 decimal places
- Handle invalid input gracefully

```rust
use std::io;

fn celsius_to_fahrenheit(c: f64) -> f64 { c * 9.0 / 5.0 + 32.0 }
fn celsius_to_kelvin(c: f64) -> f64 { c + 273.15 }
fn fahrenheit_to_celsius(f: f64) -> f64 { (f - 32.0) * 5.0 / 9.0 }

fn main() {
    println!("=== Temperature Converter ===");
    println!("Enter temperature (e.g., '100 C' or '212 F' or '373 K'):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        println!("Invalid input. Please enter a number followed by C, F, or K.");
        return;
    }

    let value: f64 = match parts[0].parse() {
        Ok(v) => v,
        Err(_) => { println!("Invalid number."); return; }
    };

    let celsius = match parts[1].to_uppercase().as_str() {
        "C" => value,
        "F" => fahrenheit_to_celsius(value),
        "K" => value - 273.15,
        _ => { println!("Unknown unit. Use C, F, or K."); return; }
    };

    println!("\nResults:");
    println!("  Celsius:    {:.2}°C", celsius);
    println!("  Fahrenheit: {:.2}°F", celsius_to_fahrenheit(celsius));
    println!("  Kelvin:     {:.2}K", celsius_to_kelvin(celsius));
}
```

**Challenge Extensions:**
- Add a loop so the user can convert multiple temperatures
- Support abbreviated unit names (e.g., "Cel", "Fahr")

---

## Section 2: Ownership, Borrowing & Lifetimes

### 2.1 The Ownership Model

Ownership is Rust's most unique feature. Every value has a single **owner**, and when the owner goes out of scope, the value is dropped (freed).

**Three Rules of Ownership:**
1. Each value in Rust has a variable that's called its *owner*
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is MOVED into s2 — s1 is no longer valid!

    // println!("{}", s1); // ERROR: value borrowed after move

    println!("{}", s2); // OK

    // Clone to make a deep copy
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);
}
```

---

### 2.2 Borrowing & References

Instead of transferring ownership, you can **borrow** a value using references (`&`).

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("hello world");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);
}
```

**Mutable References:**

```rust
fn append_world(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    let mut s = String::from("hello");
    append_world(&mut s);
    println!("{}", s); // "hello, world"
}
```

---

### 2.3 The Slice Type

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("First word: {}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}", slice);
}
```

---

### 2.4 Lifetimes

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("long string");
    let s2 = String::from("xyz");
    let result = longest(s1.as_str(), s2.as_str());
    println!("Longest: {}", result);
}
```

---

### 🛠️ Mini Project 2: Word Frequency Counter

```rust
use std::collections::HashMap;

fn word_frequency(text: &str) -> HashMap<&str, usize> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    map
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let freq = word_frequency(text);

    let mut pairs: Vec<(&&str, &usize)> = freq.iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(a.1));

    println!("Word Frequencies:");
    for (word, count) in pairs {
        println!("  {:15} : {}", word, count);
    }
}
```

**Challenge Extensions:**
- Read text from a file using `std::fs::read_to_string`
- Ignore punctuation and normalize to lowercase
- Show only the top N most frequent words

---

## Section 3: Structs, Enums & Pattern Matching

### 3.1 Structs

```rust
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    fn area(&self) -> f64 { self.width * self.height }
    fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
    fn is_square(&self) -> bool { self.width == self.height }
}

fn main() {
    let rect = Rectangle::new(10.0, 5.0);
    println!("{:?}", rect);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!("Is square: {}", rect.is_square());
}
```

---

### 3.2 Enums

```rust
#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];

    for shape in &shapes {
        println!("{:?} => area = {:.2}", shape, shape.area());
    }
}
```

---

### 3.3 Option and Pattern Matching

```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

fn main() {
    match divide(10.0, 2.0) {
        Some(result) => println!("10 / 2 = {}", result),
        None => println!("Cannot divide by zero"),
    }

    let result = divide(10.0, 0.0).unwrap_or(f64::INFINITY);
    println!("Result: {}", result);

    if let Some(val) = divide(9.0, 3.0) {
        println!("9 / 3 = {}", val);
    }

    // Exhaustive match with ranges
    let n = 42i32;
    let desc = match n {
        i32::MIN..=-1 => "negative",
        0 => "zero",
        1..=9 => "single digit",
        10..=99 => "double digit",
        _ => "large",
    };
    println!("{} is {}", n, desc);
}
```

---

### 🛠️ Mini Project 3: Shape Calculator

Build a shape calculator that uses enums and pattern matching to compute area and perimeter for multiple shapes.

**Requirements:**
- Support Circle, Rectangle, Triangle, and Hexagon
- Implement `area()` and `perimeter()` for each
- Accept user input to select a shape and enter dimensions
- Display results in a formatted table

**Challenge Extensions:**
- Add a `describe()` method returning a human-readable description
- Sort a list of shapes by area
- Implement the `Display` trait for pretty printing

---

## Section 4: Error Handling

### 4.1 The Result Type

```rust
use std::num::ParseIntError;

fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let n = s.trim().parse::<i32>()?;
    Ok(n * 2)
}

fn main() {
    match parse_and_double("21") {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }

    match parse_and_double("abc") {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}
```

---

### 4.2 The ? Operator

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    let content = fs::read_to_string("username.txt")?;
    Ok(content.trim().to_string())
}

fn main() {
    match read_username_from_file() {
        Ok(name) => println!("Username: {}", name),
        Err(e) => println!("Failed to read username: {}", e),
    }
}
```

---

### 4.3 Custom Error Types

```rust
use std::fmt;

#[derive(Debug)]
enum AppError {
    ParseError(String),
    DivisionByZero,
    NegativeInput(f64),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ParseError(s) => write!(f, "Parse error: {}", s),
            AppError::DivisionByZero => write!(f, "Cannot divide by zero"),
            AppError::NegativeInput(n) => write!(f, "Negative input not allowed: {}", n),
        }
    }
}

fn safe_sqrt(n: f64) -> Result<f64, AppError> {
    if n < 0.0 { Err(AppError::NegativeInput(n)) } else { Ok(n.sqrt()) }
}

fn main() {
    for val in [16.0, -4.0, 0.0] {
        match safe_sqrt(val) {
            Ok(result) => println!("sqrt({}) = {}", val, result),
            Err(e) => println!("Error for {}: {}", val, e),
        }
    }
}
```

---

### 🛠️ Mini Project 4: CSV Parser

Build a simple CSV parser that reads a file, parses rows, and handles errors gracefully.

**Requirements:**
- Read a CSV file with headers
- Parse each row into a struct
- Handle missing fields, type conversion errors, and file I/O errors
- Print a summary of successfully parsed vs failed rows

**Challenge Extensions:**
- Use the `thiserror` crate for ergonomic custom errors
- Write parsed data to a new file
- Support quoted fields with commas inside

---

## Section 5: Collections & Iterators

### 5.1 Vectors

```rust
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![10, 20, 30, 40, 50];

    println!("Third element: {}", v2[2]);

    match v2.get(10) {
        Some(val) => println!("Got: {}", val),
        None => println!("Index out of bounds"),
    }

    for val in &v2 {
        print!("{} ", val);
    }
    println!();
}
```

---

### 5.2 HashMaps

```rust
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Alice"), 95);
    scores.insert(String::from("Bob"), 87);

    if let Some(score) = scores.get("Alice") {
        println!("Alice's score: {}", score);
    }

    scores.entry(String::from("Dave")).or_insert(80);

    let mut pairs: Vec<_> = scores.iter().collect();
    pairs.sort_by_key(|&(k, _)| k);
    for (name, score) in pairs {
        println!("{}: {}", name, score);
    }
}
```

---

### 5.3 Iterators & Adapters

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let even_squares: Vec<i32> = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("Even squares: {:?}", even_squares);

    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);

    let a = vec![1, 2, 3];
    let b = vec!['a', 'b', 'c'];
    let zipped: Vec<_> = a.iter().zip(b.iter()).collect();
    println!("Zipped: {:?}", zipped);

    let words = vec!["hello world", "foo bar"];
    let all_words: Vec<&str> = words.iter()
        .flat_map(|s| s.split_whitespace())
        .collect();
    println!("All words: {:?}", all_words);
}
```

---

### 🛠️ Mini Project 5: Student Grade Analyzer

```rust
struct Student { name: String, scores: Vec<f64> }

impl Student {
    fn average(&self) -> f64 {
        self.scores.iter().sum::<f64>() / self.scores.len() as f64
    }
    fn grade(&self) -> char {
        match self.average() as u32 {
            90..=100 => 'A', 80..=89 => 'B',
            70..=79 => 'C', 60..=69 => 'D', _ => 'F'
        }
    }
}

fn main() {
    let students = vec![
        Student { name: "Alice".into(), scores: vec![92.0, 88.0, 95.0] },
        Student { name: "Bob".into(),   scores: vec![75.0, 82.0, 79.0] },
        Student { name: "Charlie".into(), scores: vec![60.0, 55.0, 70.0] },
    ];

    println!("{:<10} {:>8} {:>6}", "Name", "Average", "Grade");
    println!("{}", "-".repeat(28));
    for s in &students {
        println!("{:<10} {:>8.2} {:>6}", s.name, s.average(), s.grade());
    }

    let class_avg: f64 = students.iter().map(|s| s.average()).sum::<f64>()
        / students.len() as f64;
    println!("\nClass Average: {:.2}", class_avg);
}
```

---

## Section 6: Traits & Generics

### 6.1 Traits

```rust
trait Describable {
    fn describe(&self) -> String;
    fn short_name(&self) -> &str;
    fn label(&self) -> String { format!("[{}]", self.short_name()) }
}

struct Dog { name: String, breed: String }
struct Cat { name: String, indoor: bool }

impl Describable for Dog {
    fn describe(&self) -> String { format!("{} is a {} dog", self.name, self.breed) }
    fn short_name(&self) -> &str { &self.name }
}

impl Describable for Cat {
    fn describe(&self) -> String {
        let loc = if self.indoor { "indoor" } else { "outdoor" };
        format!("{} is an {} cat", self.name, loc)
    }
    fn short_name(&self) -> &str { &self.name }
}

fn print_description(item: &impl Describable) {
    println!("{}: {}", item.label(), item.describe());
}

fn main() {
    let dog = Dog { name: "Rex".into(), breed: "Labrador".into() };
    let cat = Cat { name: "Whiskers".into(), indoor: true };
    print_description(&dog);
    print_description(&cat);
}
```

---

### 6.2 Generics

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));
}
```

---

### 6.3 Trait Objects & Dynamic Dispatch

```rust
trait Animal {
    fn sound(&self) -> &str;
    fn name(&self) -> &str;
}

struct Dog; struct Cat; struct Cow;
impl Animal for Dog { fn sound(&self) -> &str { "Woof" } fn name(&self) -> &str { "Dog" } }
impl Animal for Cat { fn sound(&self) -> &str { "Meow" } fn name(&self) -> &str { "Cat" } }
impl Animal for Cow { fn sound(&self) -> &str { "Moo"  } fn name(&self) -> &str { "Cow" } }

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog), Box::new(Cat), Box::new(Cow),
    ];
    for animal in &animals {
        println!("{} says: {}", animal.name(), animal.sound());
    }
}
```

---

### 🛠️ Mini Project 6: Generic Data Store

Build a generic in-memory key-value store with trait-based serialization.

**Requirements:**
- Generic over key and value types
- Support `insert`, `get`, `remove`, `contains_key`
- Implement a `Printable` trait for values that can be displayed
- Add a method to dump all entries sorted by key

---

## Section 7: Closures & Functional Programming

### 7.1 Closures

```rust
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(f(x)) }

fn main() {
    println!("double(5) = {}", apply(|x| x * 2, 5));
    println!("add_ten(5) = {}", apply(|x| x + 10, 5));
    println!("double twice: {}", apply_twice(|x| x * 2, 3));

    let threshold = 5;
    let numbers = vec![1, 8, 3, 9, 2, 7];
    let big: Vec<i32> = numbers.into_iter().filter(|&x| x > threshold).collect();
    println!("Numbers > {}: {:?}", threshold, big);

    let name = String::from("Alice");
    let greet = move || println!("Hello, {}!", name);
    greet();
}
```

---

### 7.2 Fn, FnMut, FnOnce

```rust
fn call_once<F: FnOnce()>(f: F) { f(); }
fn call_mut<F: FnMut()>(mut f: F) { f(); f(); }
fn call_fn<F: Fn()>(f: F) { f(); f(); }

fn main() {
    let s = String::from("hello");
    call_once(move || println!("FnOnce: {}", s));

    let mut count = 0;
    call_mut(|| { count += 1; println!("FnMut count: {}", count); });

    let msg = "world";
    call_fn(|| println!("Fn: {}", msg));
}
```

---

### 7.3 Functional Patterns

```rust
fn main() {
    let data = vec![
        ("Alice", 85), ("Bob", 92), ("Charlie", 78),
        ("Diana", 95), ("Eve", 88),
    ];

    let mut top: Vec<_> = data.iter()
        .filter(|(_, s)| *s >= 80)
        .cloned()
        .collect();
    top.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Top students:");
    for (name, score) in top.iter().take(3) {
        println!("  {} - {}", name, score);
    }

    let (passing, failing): (Vec<_>, Vec<_>) = data.iter()
        .partition(|(_, s)| *s >= 80);
    println!("Passing: {}, Failing: {}", passing.len(), failing.len());
}
```

---

### 🛠️ Mini Project 7: Functional Pipeline Processor

Build a data processing pipeline using closures and iterators.

**Requirements:**
- Define a `Pipeline<T>` struct that chains transformations
- Support `map`, `filter`, `take`, and `collect` operations
- Process a dataset of employee records (name, department, salary)
- Find the top earner per department using functional style

---

## Section 8: Modules, Crates & Cargo

### 8.1 Modules

```rust
mod math {
    pub mod basic {
        pub fn add(a: i32, b: i32) -> i32 { a + b }
        pub fn subtract(a: i32, b: i32) -> i32 { a - b }
    }
    pub mod advanced {
        pub fn power(base: f64, exp: u32) -> f64 {
            (0..exp).fold(1.0, |acc, _| acc * base)
        }
        pub fn factorial(n: u64) -> u64 { (1..=n).product() }
    }
}

use math::basic::{add, subtract};
use math::advanced::*;

fn main() {
    println!("3 + 4 = {}", add(3, 4));
    println!("10 - 3 = {}", subtract(10, 3));
    println!("2^10 = {}", power(2.0, 10));
    println!("5! = {}", factorial(5));
}
```

---

### 8.2 Cargo & Dependencies

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
clap = { version = "4", features = ["derive"] }
thiserror = "1.0"
```

```bash
cargo new my_project       # Create new project
cargo build                # Build debug
cargo build --release      # Build optimized release
cargo run                  # Build and run
cargo test                 # Run tests
cargo doc --open           # Generate and open docs
cargo add serde            # Add a dependency
cargo clippy               # Lint your code
cargo fmt                  # Format your code
```

---

### 8.3 Writing Tests

```rust
fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_panic() {
        let v = vec![1, 2, 3];
        let _ = v[10];
    }
}
```

---

### 🛠️ Mini Project 8: Library Crate

Create a reusable library crate for mathematical utilities.

**Requirements:**
- Implement a `stats` module: mean, median, mode, variance, std deviation
- Implement a `geometry` module: 2D and 3D shape calculations
- Write comprehensive unit tests for all functions
- Document all public APIs with doc comments (`///`)

---

## Section 9: Working with Files

### 9.1 Basic File I/O

The `std::fs` module provides synchronous file operations. These are the building blocks for all file work in Rust.

```rust
use std::fs;
use std::io::{self, Write, BufRead, BufReader, BufWriter};

fn main() -> io::Result<()> {
    // --- Writing a file ---
    fs::write("hello.txt", "Hello, file world!\n")?;

    // --- Reading entire file as a String ---
    let content = fs::read_to_string("hello.txt")?;
    println!("Read: {}", content.trim());

    // --- Reading as raw bytes ---
    let bytes = fs::read("hello.txt")?;
    println!("Bytes: {:?}", &bytes[..5]);

    // --- Appending to a file ---
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("hello.txt")?;
    writeln!(file, "Appended line.")?;

    // --- Buffered writing (efficient for many small writes) ---
    let out = fs::File::create("buffered.txt")?;
    let mut writer = BufWriter::new(out);
    for i in 0..1000 {
        writeln!(writer, "Line {}", i)?;
    }
    writer.flush()?; // Ensure all buffered data is written

    // --- Buffered reading line by line ---
    let input = fs::File::open("buffered.txt")?;
    let reader = BufReader::new(input);
    let mut line_count = 0;
    for line in reader.lines() {
        let _ = line?;
        line_count += 1;
    }
    println!("Lines written and read back: {}", line_count);

    // --- Cleanup ---
    fs::remove_file("hello.txt")?;
    fs::remove_file("buffered.txt")?;

    Ok(())
}
```

---

### 9.2 Working with Paths and Directories

`std::path::Path` and `PathBuf` are Rust's cross-platform path types.

```rust
use std::fs;
use std::path::{Path, PathBuf};
use std::io;

fn main() -> io::Result<()> {
    // --- Building paths safely (cross-platform) ---
    let base = PathBuf::from("my_project");
    let config_path = base.join("config").join("settings.toml");
    println!("Config path: {}", config_path.display());

    // --- Path inspection ---
    let p = Path::new("/home/user/documents/report.pdf");
    println!("File name:  {:?}", p.file_name());
    println!("Extension:  {:?}", p.extension());
    println!("Parent dir: {:?}", p.parent());
    println!("Stem:       {:?}", p.file_stem());
    println!("Is absolute: {}", p.is_absolute());

    // --- Creating directories ---
    fs::create_dir_all("sandbox/a/b/c")?;

    // --- Listing directory contents ---
    for entry in fs::read_dir("sandbox")? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;
        println!(
            "{} | dir={} | size={}",
            path.display(),
            metadata.is_dir(),
            metadata.len()
        );
    }

    // --- Recursive directory walk (manual) ---
    fn walk_dir(dir: &Path) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    walk_dir(&path)?;
                } else {
                    println!("File: {}", path.display());
                }
            }
        }
        Ok(())
    }

    walk_dir(Path::new("sandbox"))?;

    // --- Cleanup ---
    fs::remove_dir_all("sandbox")?;

    Ok(())
}
```

---

### 9.3 Temporary Files

Temporary files are essential for safe intermediate processing. Use the `tempfile` crate for robust temp file handling.

Add to `Cargo.toml`:
```toml
[dependencies]
tempfile = "3"
```

```rust
use std::io::{Write, Read, Seek, SeekFrom};
use tempfile::{tempfile, NamedTempFile, TempDir};

fn main() -> std::io::Result<()> {
    // --- Anonymous temp file (deleted when handle is dropped) ---
    let mut tmp = tempfile()?;
    writeln!(tmp, "Temporary data")?;
    tmp.seek(SeekFrom::Start(0))?;
    let mut content = String::new();
    tmp.read_to_string(&mut content)?;
    println!("Anon temp: {}", content.trim());
    // File is automatically deleted here when `tmp` goes out of scope

    // --- Named temp file (you can get its path) ---
    let mut named = NamedTempFile::new()?;
    writeln!(named, "Named temp content")?;
    println!("Temp file path: {}", named.path().display());

    // Persist the temp file (moves it to a permanent location)
    let permanent_path = std::env::temp_dir().join("my_saved_file.txt");
    named.persist(&permanent_path)?;
    println!("Persisted to: {}", permanent_path.display());
    std::fs::remove_file(&permanent_path)?;

    // --- Temp directory ---
    let tmp_dir = TempDir::new()?;
    let file_path = tmp_dir.path().join("data.txt");
    std::fs::write(&file_path, "data in temp dir")?;
    println!("Temp dir: {}", tmp_dir.path().display());
    // Entire directory is deleted when `tmp_dir` is dropped

    Ok(())
}
```

---

### 9.4 Atomic File Writing

Atomic writes prevent data corruption — if your program crashes mid-write, the original file is untouched. The pattern is: **write to a temp file → rename over the target**.

```rust
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write, BufWriter};
use std::path::Path;

/// Atomically write `data` to `target_path`.
/// Uses a sibling temp file + rename to ensure the write is all-or-nothing.
fn atomic_write(target_path: &Path, data: &[u8]) -> io::Result<()> {
    // Temp file must be on the same filesystem as the target
    // so that rename() is atomic (a single syscall).
    let parent = target_path.parent().unwrap_or(Path::new("."));
    let tmp_path = parent.join(format!(
        ".tmp_{}_{}",
        target_path.file_name().unwrap().to_string_lossy(),
        std::process::id()
    ));

    // Write to the temp file
    {
        let mut tmp_file = BufWriter::new(File::create(&tmp_path)?);
        tmp_file.write_all(data)?;
        tmp_file.flush()?;
        // fsync ensures data is on disk before rename
        tmp_file.into_inner()?.sync_all()?;
    }

    // Atomically replace the target
    fs::rename(&tmp_path, target_path)?;

    Ok(())
}

fn main() -> io::Result<()> {
    let path = Path::new("config.json");

    let data = r#"{ "version": 1, "debug": false }"#;
    atomic_write(path, data.as_bytes())?;
    println!("Wrote config atomically.");

    // Simulate an update
    let updated = r#"{ "version": 2, "debug": true }"#;
    atomic_write(path, updated.as_bytes())?;
    println!("Updated config atomically.");

    println!("Final: {}", fs::read_to_string(path)?);
    fs::remove_file(path)?;

    Ok(())
}
```

> **Why rename is atomic:** On POSIX systems (Linux/macOS), `rename()` is guaranteed to be atomic by the kernel. On Windows, Rust's `fs::rename` uses `MoveFileExW` with `MOVEFILE_REPLACE_EXISTING`. For cross-platform guaranteed atomicity, consider the `atomicwrites` or `tempfile` crates.

---

### 9.5 File Metadata & Permissions

```rust
use std::fs;
use std::time::UNIX_EPOCH;

fn main() -> std::io::Result<()> {
    fs::write("meta_test.txt", "some content")?;

    let meta = fs::metadata("meta_test.txt")?;

    println!("Size:       {} bytes", meta.len());
    println!("Is file:    {}", meta.is_file());
    println!("Is dir:     {}", meta.is_dir());
    println!("Read-only:  {}", meta.permissions().readonly());

    if let Ok(modified) = meta.modified() {
        let secs = modified.duration_since(UNIX_EPOCH).unwrap().as_secs();
        println!("Modified:   {} (unix timestamp)", secs);
    }

    // Make file read-only
    let mut perms = meta.permissions();
    perms.set_readonly(true);
    fs::set_permissions("meta_test.txt", perms)?;
    println!("Set to read-only.");

    // Restore write permission before deleting
    let mut perms = fs::metadata("meta_test.txt")?.permissions();
    perms.set_readonly(false);
    fs::set_permissions("meta_test.txt", perms)?;
    fs::remove_file("meta_test.txt")?;

    Ok(())
}
```

---

### 9.6 Async File I/O with Tokio

For applications that need to handle many concurrent file operations without blocking threads, use `tokio::fs`.

Add to `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncSeekExt, BufReader, BufWriter};
use std::io::SeekFrom;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    // --- Async write ---
    fs::write("async_hello.txt", "Hello from async Rust!\n").await?;

    // --- Async read ---
    let content = fs::read_to_string("async_hello.txt").await?;
    println!("Read: {}", content.trim());

    // --- Async buffered write ---
    let file = File::create("async_buffered.txt").await?;
    let mut writer = BufWriter::new(file);
    for i in 0..100 {
        writer.write_all(format!("Line {}\n", i).as_bytes()).await?;
    }
    writer.flush().await?;

    // --- Async buffered read ---
    let file = File::open("async_buffered.txt").await?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    use tokio::io::AsyncBufReadExt;
    let mut count = 0;
    loop {
        line.clear();
        let n = reader.read_line(&mut line).await?;
        if n == 0 { break; }
        count += 1;
    }
    println!("Lines read: {}", count);

    // --- Concurrent file reads ---
    let paths = vec!["async_hello.txt", "async_buffered.txt"];
    let handles: Vec<_> = paths.iter()
        .map(|p| tokio::spawn(fs::read_to_string(p.to_string())))
        .collect();

    for handle in handles {
        match handle.await? {
            Ok(text) => println!("Concurrent read: {} chars", text.len()),
            Err(e) => println!("Error: {}", e),
        }
    }

    // --- Cleanup ---
    fs::remove_file("async_hello.txt").await?;
    fs::remove_file("async_buffered.txt").await?;

    Ok(())
}
```

---

### 9.7 Async Atomic Writing

Combining async I/O with atomic write safety:

```rust
use tokio::fs::{self, File};
use tokio::io::{AsyncWriteExt, BufWriter};
use std::path::Path;

async fn async_atomic_write(target: &Path, data: &[u8]) -> tokio::io::Result<()> {
    let parent = target.parent().unwrap_or(Path::new("."));
    let tmp = parent.join(format!(
        ".tmp_{}_{}",
        target.file_name().unwrap().to_string_lossy(),
        std::process::id()
    ));

    {
        let file = File::create(&tmp).await?;
        let mut writer = BufWriter::new(file);
        writer.write_all(data).await?;
        writer.flush().await?;
        writer.into_inner().sync_all().await?;
    }

    fs::rename(&tmp, target).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let path = Path::new("settings.json");
    let payload = br#"{"theme":"dark","font_size":14}"#;

    async_atomic_write(path, payload).await?;
    println!("Async atomic write complete.");
    println!("Content: {}", fs::read_to_string(path).await?);

    fs::remove_file(path).await?;
    Ok(())
}
```

---

### 9.8 File Watching

React to file system changes in real time using the `notify` crate.

Add to `Cargo.toml`:
```toml
[dependencies]
notify = "6"
```

```rust
use notify::{Watcher, RecursiveMode, recommended_watcher, Event};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::Path;

fn main() -> notify::Result<()> {
    let (tx, rx) = channel::<notify::Result<Event>>();

    let mut watcher = recommended_watcher(tx)?;
    watcher.watch(Path::new("."), RecursiveMode::NonRecursive)?;

    println!("Watching current directory for changes...");
    println!("(Modify or create a file to see events)");

    // Listen for up to 5 events then exit
    let mut event_count = 0;
    for res in rx {
        match res {
            Ok(event) => {
                println!("Event: {:?}", event.kind);
                for path in &event.paths {
                    println!("  Path: {}", path.display());
                }
                event_count += 1;
                if event_count >= 5 { break; }
            }
            Err(e) => println!("Watch error: {:?}", e),
        }
    }

    Ok(())
}
```

---

### 🛠️ Mini Project 9: Atomic Config Manager

Build a configuration manager that safely reads and writes JSON config files.

**Requirements:**
- Load config from `~/.myapp/config.json` on startup (create defaults if missing)
- Support `get <key>`, `set <key> <value>`, and `list` commands
- All writes must be atomic (temp file + rename)
- Preserve comments/formatting using pretty-printed JSON
- Handle concurrent access gracefully with file locking

```rust
// Cargo.toml dependencies needed:
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
// dirs = "5"

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Default)]
struct Config {
    values: HashMap<String, serde_json::Value>,
}

struct ConfigManager {
    path: PathBuf,
    config: Config,
}

impl ConfigManager {
    fn load_or_create(path: PathBuf) -> std::io::Result<Self> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let config = if path.exists() {
            let text = fs::read_to_string(&path)?;
            serde_json::from_str(&text).unwrap_or_default()
        } else {
            Config::default()
        };
        Ok(ConfigManager { path, config })
    }

    fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.config.values.get(key)
    }

    fn set(&mut self, key: String, value: serde_json::Value) {
        self.config.values.insert(key, value);
    }

    fn save(&self) -> std::io::Result<()> {
        let parent = self.path.parent().unwrap_or(Path::new("."));
        let tmp = parent.join(format!(".tmp_config_{}", std::process::id()));
        {
            let mut writer = BufWriter::new(File::create(&tmp)?);
            let json = serde_json::to_string_pretty(&self.config).unwrap();
            writer.write_all(json.as_bytes())?;
            writer.flush()?;
            writer.into_inner()?.sync_all()?;
        }
        fs::rename(&tmp, &self.path)?;
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let config_path = PathBuf::from("myapp_config.json");
    let mut mgr = ConfigManager::load_or_create(config_path)?;

    mgr.set("theme".into(), serde_json::json!("dark"));
    mgr.set("font_size".into(), serde_json::json!(14));
    mgr.set("auto_save".into(), serde_json::json!(true));
    mgr.save()?;

    println!("Config saved.");
    if let Some(theme) = mgr.get("theme") {
        println!("theme = {}", theme);
    }

    fs::remove_file("myapp_config.json")?;
    Ok(())
}
```

**Challenge Extensions:**
- Add async support using `tokio::fs`
- Implement file locking with the `fs2` crate
- Support TOML format in addition to JSON
- Add a `--watch` flag that reloads config when the file changes

---

## Section 10: Concurrency & Async Programming

### 10.1 Threads

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Spawned thread: {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 1..=3 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(80));
    }

    handle.join().unwrap();
}
```

---

### 10.2 Message Passing with Channels

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(format!("Message from thread {}", i)).unwrap();
        });
    }

    drop(tx);

    for received in rx {
        println!("Received: {}", received);
    }
}
```

---

### 10.3 Shared State with Arc and Mutex

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles { handle.join().unwrap(); }
    println!("Final counter: {}", *counter.lock().unwrap());
}
```

---

### 10.4 Async/Await with Tokio

```rust
use tokio::time::{sleep, Duration};

async fn fetch_data(id: u32) -> String {
    sleep(Duration::from_millis(100)).await;
    format!("Data for id={}", id)
}

#[tokio::main]
async fn main() {
    let (r1, r2, r3) = tokio::join!(
        fetch_data(10),
        fetch_data(11),
        fetch_data(12),
    );
    println!("{}, {}, {}", r1, r2, r3);
}
```

---

### 🛠️ Mini Project 10: Concurrent Web Scraper

Build a concurrent web scraper that fetches multiple URLs in parallel.

**Requirements:**
- Accept a list of URLs
- Fetch all URLs concurrently using `tokio` and `reqwest`
- Extract the page title from each response
- Report success/failure for each URL with timing information
- Limit concurrency to N simultaneous requests using a semaphore

**Challenge Extensions:**
- Save results to a JSON file using `serde_json`
- Retry failed requests with exponential backoff
- Add a progress bar using the `indicatif` crate

---

## Section 11: Advanced Rust — Macros, Unsafe & FFI

### 11.1 Declarative Macros

```rust
macro_rules! vec_of_strings {
    ($($x:expr),*) => {
        vec![$($x.to_string()),*]
    };
}

macro_rules! assert_approx_eq {
    ($a:expr, $b:expr, $eps:expr) => {
        assert!(
            ($a - $b).abs() < $eps,
            "assertion failed: |{} - {}| = {} >= {}",
            $a, $b, ($a - $b).abs(), $eps
        );
    };
}

fn main() {
    let names = vec_of_strings!["Alice", "Bob", "Charlie"];
    println!("{:?}", names);
    assert_approx_eq!(3.14159, std::f64::consts::PI, 0.001);
    println!("Pi approximation is close enough!");
}
```

---

### 11.2 Procedural Macros (Derive)

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    host: String,
    port: u16,
    max_connections: u32,
    debug_mode: bool,
}

fn main() {
    let config = Config {
        host: "localhost".to_string(),
        port: 8080,
        max_connections: 100,
        debug_mode: true,
    };

    let json = serde_json::to_string_pretty(&config).unwrap();
    println!("{}", json);

    let config2: Config = serde_json::from_str(&json).unwrap();
    println!("Host: {}, Port: {}", config2.host, config2.port);
}
```

---

### 11.3 Unsafe Rust

```rust
fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 = {}", *r1);
        *r2 = 10;
        println!("r2 = {}", *r2);
    }

    unsafe fn dangerous() { println!("Unsafe function called"); }
    unsafe { dangerous(); }
}
```

---

### 11.4 Smart Pointers

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // Box<T> — heap allocation
    let boxed = Box::new(5);
    println!("Boxed: {}", boxed);

    // Rc<T> — reference counting
    let a = Rc::new(String::from("shared"));
    let b = Rc::clone(&a);
    println!("Ref count: {}", Rc::strong_count(&a));
    println!("{} {}", a, b);

    // Rc<RefCell<T>> — shared mutable state
    let shared = Rc::new(RefCell::new(0));
    let c1 = Rc::clone(&shared);
    let c2 = Rc::clone(&shared);
    *c1.borrow_mut() += 10;
    *c2.borrow_mut() += 20;
    println!("Shared value: {}", shared.borrow());
}
```

---

### 🛠️ Mini Project 11: Custom Derive Macro

Build a custom procedural macro that auto-implements a `Builder` pattern.

**Requirements:**
- Create a `#[derive(Builder)]` macro
- Generate a `XxxBuilder` struct for any annotated struct
- Support optional fields with `Option<T>`
- Generate a `build()` method that validates required fields
- Write integration tests

---

## Capstone Project: Real-World CLI Task Manager

### Project Overview

Build a fully-featured **command-line task management application** — a real-world tool you can actually use. This project integrates everything you've learned.

**Application Name:** `rustask` — A blazing-fast CLI task manager

---

### Features

- ✅ Add, list, complete, and delete tasks
- 🏷️ Tag tasks with categories and priorities
- 📅 Set due dates and get overdue alerts
- 🔍 Filter and search tasks
- 💾 Persist data atomically to a JSON file
- 📊 Show statistics and productivity reports
- 🎨 Colorful terminal output
- ⚡ Async file I/O

---

### Project Structure

```
rustask/
├── Cargo.toml
├── src/
│   ├── main.rs          # Entry point, CLI parsing
│   ├── task.rs          # Task struct, enums, logic
│   ├── storage.rs       # Atomic file persistence (JSON)
│   ├── commands.rs      # Command handlers
│   ├── display.rs       # Terminal formatting
│   └── error.rs         # Custom error types
└── tests/
    ├── task_tests.rs
    └── storage_tests.rs
```

---

### Cargo.toml

```toml
[package]
name = "rustask"
version = "0.1.0"
edition = "2021"
description = "A blazing-fast CLI task manager written in Rust"

[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
colored = "2.0"
tokio = { version = "1", features = ["full"] }
thiserror = "1.0"
uuid = { version = "1.0", features = ["v4"] }
dirs = "5"
tempfile = "3"
```

---

### Core Data Model (src/task.rs)

```rust
use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority { Low, Medium, High, Critical }

impl Priority {
    pub fn emoji(&self) -> &str {
        match self {
            Priority::Low => "🟢", Priority::Medium => "🟡",
            Priority::High => "🟠", Priority::Critical => "🔴",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Status { Todo, InProgress, Done, Cancelled }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub status: Status,
    pub tags: Vec<String>,
    pub due_date: Option<NaiveDate>,
    pub created_at: DateTime<Local>,
    pub completed_at: Option<DateTime<Local>>,
}

impl Task {
    pub fn new(title: String, priority: Priority) -> Self {
        Task {
            id: Uuid::new_v4().to_string()[..8].to_string(),
            title, description: None, priority,
            status: Status::Todo, tags: vec![],
            due_date: None, created_at: Local::now(), completed_at: None,
        }
    }

    pub fn is_overdue(&self) -> bool {
        if matches!(self.status, Status::Done | Status::Cancelled) { return false; }
        self.due_date.map_or(false, |d| d < Local::now().date_naive())
    }

    pub fn complete(&mut self) {
        self.status = Status::Done;
        self.completed_at = Some(Local::now());
    }
}
```

---

### Atomic Storage Layer (src/storage.rs)

```rust
use crate::error::AppError;
use crate::task::Task;
use std::path::PathBuf;
use tokio::fs::{self, File};
use tokio::io::{AsyncWriteExt, BufWriter};

pub struct Storage { path: PathBuf }

impl Storage {
    pub fn new() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        Storage { path: home.join(".rustask").join("tasks.json") }
    }

    pub async fn load(&self) -> Result<Vec<Task>, AppError> {
        if !self.path.exists() { return Ok(vec![]); }
        let content = fs::read_to_string(&self.path).await?;
        Ok(serde_json::from_str(&content)?)
    }

    /// Atomically save tasks — uses temp file + rename to prevent corruption
    pub async fn save(&self, tasks: &[Task]) -> Result<(), AppError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let tmp = self.path.with_extension(
            format!("tmp_{}", std::process::id())
        );

        {
            let file = File::create(&tmp).await?;
            let mut writer = BufWriter::new(file);
            let json = serde_json::to_string_pretty(tasks)?;
            writer.write_all(json.as_bytes()).await?;
            writer.flush().await?;
            writer.into_inner().sync_all().await?;
        }

        fs::rename(&tmp, &self.path).await?;
        Ok(())
    }
}
```

---

### Custom Errors (src/error.rs)

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] tokio::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Task not found: {0}")]
    TaskNotFound(String),
    #[error("Invalid date format: {0}")]
    InvalidDate(String),
    #[error("Invalid priority: {0}")]
    InvalidPriority(String),
}
```

---

### CLI Interface (src/main.rs)

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustask", about = "A blazing-fast CLI task manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        title: String,
        #[arg(short, long, default_value = "medium")]
        priority: String,
        #[arg(short, long)]
        due: Option<String>,
        #[arg(short, long)]
        tags: Option<String>,
    },
    /// List all tasks
    List {
        #[arg(short, long)] status: Option<String>,
        #[arg(short, long)] tag: Option<String>,
        #[arg(long)] overdue: bool,
    },
    /// Mark a task as complete
    Done { id: String },
    /// Delete a task
    Delete { id: String },
    /// Show statistics
    Stats,
    /// Search tasks by keyword
    Search { query: String },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    // Route to command handlers in commands.rs
}
```

---

### Display & Formatting (src/display.rs)

```rust
use crate::task::{Priority, Status, Task};
use colored::*;

pub fn print_task(task: &Task) {
    let status_icon = match task.status {
        Status::Todo => "○".white(),
        Status::InProgress => "◐".yellow(),
        Status::Done => "●".green(),
        Status::Cancelled => "✗".red(),
    };

    let overdue = if task.is_overdue() {
        " ⚠ OVERDUE".red().bold().to_string()
    } else { String::new() };

    println!("{} {} [{}] {}{}", status_icon, task.priority.emoji(),
        task.id.cyan(), task.title.bold(), overdue);

    if let Some(due) = task.due_date {
        println!("   📅 Due: {}", due);
    }
    if !task.tags.is_empty() {
        let tags: Vec<String> = task.tags.iter()
            .map(|t| format!("#{}", t).blue().to_string()).collect();
        println!("   🏷  {}", tags.join(" "));
    }
}

pub fn print_stats(tasks: &[Task]) {
    let total = tasks.len();
    let done = tasks.iter().filter(|t| t.status == Status::Done).count();
    let todo = tasks.iter().filter(|t| t.status == Status::Todo).count();
    let overdue = tasks.iter().filter(|t| t.is_overdue()).count();

    println!("{}", "=== Task Statistics ===".bold());
    println!("Total:   {}", total.to_string().cyan());
    println!("Done:    {}", done.to_string().green());
    println!("Todo:    {}", todo.to_string().yellow());
    println!("Overdue: {}", overdue.to_string().red());

    if total > 0 {
        let pct = (done as f64 / total as f64) * 100.0;
        let bar_len = (pct / 5.0) as usize;
        let bar = "█".repeat(bar_len) + &"░".repeat(20 - bar_len);
        println!("Progress: {:.1}% [{}]", pct, bar.green());
    }
}
```

---

### Implementation Milestones

**Milestone 1 — Core Data Model**
- [ ] Define `Task`, `Priority`, `Status` types
- [ ] Implement `Task::new()`, `Task::complete()`, `Task::is_overdue()`
- [ ] Write unit tests for all methods

**Milestone 2 — Atomic Storage Layer**
- [ ] Implement JSON serialization/deserialization
- [ ] Implement async `load()` and atomic `save()`
- [ ] Handle first-run (no file exists) gracefully
- [ ] Test that a crash mid-write does not corrupt existing data

**Milestone 3 — CLI Interface**
- [ ] Set up `clap` with all subcommands
- [ ] Implement `add` and `list` commands

**Milestone 4 — Task Operations**
- [ ] Implement `done`, `delete`, `search` commands
- [ ] Add tag and status filtering
- [ ] Add overdue detection

**Milestone 5 — Display & Polish**
- [ ] Add colored terminal output
- [ ] Implement `stats` command with progress bar
- [ ] Add input validation and helpful error messages

**Milestone 6 — Testing & Documentation**
- [ ] Write integration tests
- [ ] Add doc comments to all public APIs
- [ ] Write a README with usage examples

---

### Sample Usage

```bash
rustask add "Write project proposal" --priority high --due 2024-12-31 --tags work,writing
rustask add "Buy groceries" --priority low --tags personal
rustask list
rustask list --status todo
rustask list --tag work
rustask list --overdue
rustask done abc12345
rustask search "bug"
rustask stats
```

---

### Stretch Goals

1. **Recurring Tasks** — tasks that reset on completion (daily, weekly)
2. **Sub-tasks** — hierarchical task breakdown
3. **Export** — export to Markdown, CSV, or HTML report
4. **Sync** — sync tasks to a remote server via REST API
5. **TUI Mode** — interactive terminal UI using the `ratatui` crate
6. **Notifications** — desktop notifications for due tasks using `notify-rust`
7. **Natural Language Dates** — parse "tomorrow", "next Friday" as due dates
8. **File Watching** — auto-reload tasks if the file is edited externally

---

## 🎓 What's Next?

Congratulations on completing the Rust Developer Roadmap! Here are resources to continue your journey:

### Official Resources
- [The Rust Book](https://doc.rust-lang.org/book/) — The definitive guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — Learn by doing
- [Rustlings](https://github.com/rust-lang/rustlings) — Small exercises
- [The Async Book](https://rust-lang.github.io/async-book/) — Deep dive into async

### Advanced Topics to Explore
- **WebAssembly** — Compile Rust to WASM with `wasm-pack`
- **Embedded Systems** — Bare-metal Rust with `no_std`
- **Game Development** — `Bevy` game engine
- **Web Development** — `Axum` or `Actix-web` frameworks
- **Systems Programming** — OS kernels, device drivers
- **Cryptography** — `ring` and `rustls` crates

### Community
- [r/rust](https://www.reddit.com/r/rust/) — Reddit community
- [Rust Users Forum](https://users.rust-lang.org/) — Official forum
- [This Week in Rust](https://this-week-in-rust.org/) — Weekly newsletter
- [Rust Discord](https://discord.gg/rust-lang) — Real-time chat

---

*Happy coding, Rustacean! 🦀*
