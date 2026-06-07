# The Rust Developer Roadmap: Novice to Pro

> **A comprehensive, prose-first guide to mastering Rust — written for developers who want to truly understand the language, not just copy-paste code.**

---

## Table of Contents

1. [Getting Started with Rust](#section-1-getting-started-with-rust)
2. [Ownership, Borrowing & Lifetimes](#section-2-ownership-borrowing--lifetimes)
3. [Structs, Enums & Pattern Matching](#section-3-structs-enums--pattern-matching)
4. [Smart Pointers & Interior Mutability](#section-4-smart-pointers--interior-mutability)
5. [Error Handling](#section-5-error-handling)
6. [Collections & Iterators](#section-6-collections--iterators)
7. [Traits & Generics](#section-7-traits--generics)
8. [Closures & Functional Patterns](#section-8-closures--functional-patterns)
9. [Working with Files](#section-9-working-with-files)
10. [Modules, Crates & Workspaces](#section-10-modules-crates--workspaces)
11. [Testing, Linting & Formatting](#section-11-testing-linting--formatting)
12. [Concurrency & Async Rust](#section-12-concurrency--async-rust)
13. [Macros](#section-13-macros)
14. [Unsafe Rust & FFI](#section-14-unsafe-rust--ffi)
15. [Design Patterns in Rust](#section-15-design-patterns-in-rust)
16. [Performance & Profiling](#section-16-performance--profiling)
17. [Networking & Web](#section-17-networking--web)
18. [Serde Deep Dive](#section-18-serde-deep-dive)
19. [Capstone Project: logforge](#capstone-project-logforge)

---

## Section 1: Getting Started with Rust

### Why Rust Exists

Before writing a single line of Rust, it is worth understanding *why* Rust was created. Most systems programming languages force you to choose between two things: safety and control. Languages like C and C++ give you full control over memory, but they also give you full responsibility for it — and decades of CVEs, buffer overflows, and use-after-free bugs are the result. Languages like Java, Python, and Go give you safety through garbage collection, but at the cost of runtime overhead, unpredictable pauses, and reduced control over memory layout.

Rust's central thesis is that this tradeoff is a false dichotomy. Through a system called *ownership*, Rust enforces memory safety at compile time — with no garbage collector, no runtime, and no performance penalty. The compiler itself becomes your safety net, catching entire classes of bugs before your program ever runs.

This is not just a theoretical achievement. Rust has been adopted by Mozilla, Microsoft, Google, Amazon, Meta, and the Linux kernel — not because it is trendy, but because it solves real problems that have plagued systems programming for decades.

### The Mental Model: The Compiler as a Strict Colleague

When you first start writing Rust, the compiler will reject a lot of your code. This can feel frustrating, especially if you are coming from a language where the compiler is more permissive. The key mental shift is to stop thinking of the compiler as an obstacle and start thinking of it as a very strict, very knowledgeable colleague who is reviewing your code in real time.

Every error the Rust compiler produces is telling you something meaningful about your program. Unlike C, where undefined behavior silently corrupts memory, Rust surfaces these issues at compile time with detailed, actionable error messages. Learning to read and appreciate these messages is one of the most important skills you will develop as a Rust programmer.

### Installing Rust

Rust is installed and managed through `rustup`, the official Rust toolchain installer. `rustup` handles installing the compiler (`rustc`), the package manager (`cargo`), and the standard library, and it makes it easy to switch between stable, beta, and nightly versions of the compiler.

```bash
# Install rustup (on macOS/Linux)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version

# Update Rust
rustup update

# Install nightly (needed for some advanced features)
rustup install nightly
rustup default stable  # keep stable as default
```

After installation, `rustup` will have added `~/.cargo/bin` to your PATH. This directory contains all the Rust toolchain binaries.

### Understanding Cargo

Cargo is Rust's build system and package manager, and it is one of the best tools in the Rust ecosystem. Unlike C/C++ where you might use Make, CMake, or Bazel, Cargo is the single, official, universally-used build tool for Rust. This means that virtually every Rust project you encounter will use Cargo, and learning it well pays dividends immediately.

Cargo handles:
- **Building** your project (`cargo build`)
- **Running** your project (`cargo run`)
- **Testing** your project (`cargo test`)
- **Managing dependencies** (called *crates*) via `Cargo.toml`
- **Publishing** your library to crates.io
- **Generating documentation** (`cargo doc`)
- **Checking** your code without producing a binary (`cargo check`)

The distinction between `cargo build` and `cargo check` is important. `cargo check` runs the compiler's analysis phase but skips code generation, making it much faster. During development, you will often run `cargo check` in a tight loop to get fast feedback on type errors and borrow checker violations.

```bash
# Create a new binary project
cargo new my_project
cd my_project

# Create a new library project
cargo new my_lib --lib

# Build in debug mode (fast compile, slow runtime, includes debug info)
cargo build

# Build in release mode (slow compile, fast runtime, optimized)
cargo build --release

# Run the project
cargo run

# Run with arguments
cargo run -- arg1 arg2

# Check for errors without building
cargo check

# Run tests
cargo test

# Generate and open documentation
cargo doc --open
```

### The Cargo.toml File

Every Rust project has a `Cargo.toml` file at its root. This is the manifest file that describes your project — its name, version, dependencies, and build configuration. Understanding this file is essential.

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "A brief description of what this does"
license = "MIT"

[dependencies]
# External crates from crates.io
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"

[dev-dependencies]
# Dependencies only used in tests and benchmarks
criterion = "0.5"
mockall = "0.12"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

The `edition` field is important. Rust releases new *editions* every few years (2015, 2018, 2021) that introduce breaking changes to the language syntax in a backward-compatible way. Each crate declares which edition it uses, and the compiler handles the differences. Always use edition 2021 for new projects.

### Your First Rust Program

Let us walk through a slightly more interesting first program than "Hello, World!" — one that demonstrates several Rust concepts at once.

```rust
// src/main.rs

// The `use` keyword brings items into scope, similar to `import` in Python
// or `#include` in C++. Here we bring in the standard I/O module.
use std::io::{self, BufRead};

fn main() {
    // `println!` is a macro (note the `!`), not a function.
    // Macros in Rust are expanded at compile time.
    println!("Enter lines of text (Ctrl+D to stop):");

    // `io::stdin()` returns a handle to standard input.
    // `.lock()` acquires a lock for thread-safe access.
    let stdin = io::stdin();

    // We collect lines into a Vec<String>.
    // The type annotation `Vec<String>` tells the compiler what we expect.
    let mut lines: Vec<String> = Vec::new();

    // `.lines()` returns an iterator over lines.
    // Each item is a `Result<String, io::Error>` — we use `.flatten()`
    // to skip any error lines (a simplification for now).
    for line in stdin.lock().lines().flatten() {
        lines.push(line);
    }

    // Print a summary
    println!("\nYou entered {} lines.", lines.len());
    println!("Sorted:");

    // `.sort()` sorts in place. Note: `lines` must be `mut` for this.
    lines.sort();

    for (i, line) in lines.iter().enumerate() {
        // `{i}` and `{line}` are format specifiers — Rust's equivalent
        // of Python's f-strings or C's printf format strings.
        println!("  {i}: {line}");
    }
}
```

Even in this small program, you can see several Rust idioms: the `use` statement, macros with `!`, explicit mutability with `mut`, iterators, and `Result` types. We will cover all of these in depth throughout this guide.

### Primitive Types and Variables

Rust is a statically typed language, meaning every variable has a type known at compile time. However, Rust has powerful *type inference* — the compiler can often figure out the type from context, so you do not always need to write it explicitly.

```rust
fn main() {
    // Type inference: Rust infers `x` is `i32` (the default integer type)
    let x = 5;

    // Explicit type annotation
    let y: i64 = 10_000_000_000; // underscores in numbers for readability

    // Immutability by default: this would be a compile error:
    // x = 6; // ERROR: cannot assign twice to immutable variable

    // To make a variable mutable, use `mut`
    let mut count = 0;
    count += 1;

    // Shadowing: you can re-declare a variable with `let`
    // This is different from mutation — it creates a new binding
    let spaces = "   ";
    let spaces = spaces.len(); // now `spaces` is a usize, not a &str

    // Integer types: i8, i16, i32, i64, i128, isize (signed)
    //                u8, u16, u32, u64, u128, usize (unsigned)
    let a: i32 = -42;
    let b: u8 = 255;
    let c: usize = 42; // used for indexing and sizes

    // Floating point: f32, f64 (default is f64)
    let pi: f64 = 3.14159265358979;

    // Boolean
    let is_ready: bool = true;

    // Character: a Unicode scalar value (4 bytes)
    let letter: char = 'A';
    let emoji: char = '🦀'; // Rust's mascot, Ferris the crab

    // Tuples: fixed-size, heterogeneous collections
    let point: (f64, f64) = (3.0, 4.0);
    let (px, py) = point; // destructuring
    println!("Point: ({}, {})", point.0, point.1);

    // Arrays: fixed-size, homogeneous collections (stack-allocated)
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let zeros = [0; 100]; // 100 zeros
    println!("First element: {}", arr[0]);
    println!("Length: {}", arr.len());
}
```

One of the most important things to understand about Rust variables is that **immutability is the default**. This is a deliberate design choice. Immutable data is easier to reason about, easier to share across threads, and less prone to bugs. When you need mutation, you opt in explicitly with `mut`. This forces you to think about which data needs to change and which does not.

### Control Flow

Rust's control flow constructs will feel familiar if you have used other languages, but there are some important differences.

```rust
fn main() {
    // if/else: conditions do not need parentheses (unlike C/Java)
    let number = 7;
    if number < 5 {
        println!("less than five");
    } else if number == 5 {
        println!("five");
    } else {
        println!("greater than five");
    }

    // if is an expression in Rust — it returns a value!
    // This is a key difference from C/Java where if is a statement.
    let description = if number % 2 == 0 { "even" } else { "odd" };
    println!("{number} is {description}");

    // loop: infinite loop (use `break` to exit)
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // `break` can return a value from a loop
        }
    };
    println!("Loop result: {result}");

    // while loop
    let mut n = 1;
    while n < 100 {
        n *= 2;
    }
    println!("First power of 2 >= 100: {n}");

    // for loop with ranges
    for i in 0..5 {
        print!("{i} "); // 0 1 2 3 4
    }
    println!();

    for i in 0..=5 {
        print!("{i} "); // 0 1 2 3 4 5 (inclusive)
    }
    println!();

    // for loop over a collection
    let fruits = ["apple", "banana", "cherry"];
    for fruit in &fruits {
        println!("I like {fruit}");
    }

    // Iterating with index using enumerate()
    for (i, fruit) in fruits.iter().enumerate() {
        println!("{i}: {fruit}");
    }
}
```

Notice that `if` and `loop` are *expressions* in Rust — they produce values. This is part of Rust's expression-oriented design philosophy, borrowed from functional languages. Almost everything in Rust is an expression, which enables concise, readable code without sacrificing clarity.

### Functions

Functions in Rust are declared with the `fn` keyword. The return type is specified after `->`. Rust functions return the value of their last expression implicitly — no `return` keyword needed (though you can use it for early returns).

```rust
// A simple function with parameters and a return type
fn add(a: i32, b: i32) -> i32 {
    a + b // no semicolon = this is the return value
}

// Using `return` for early exit
fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        return f64::NAN; // early return
    }
    a / b // implicit return
}

// Functions can return tuples for multiple values
fn min_max(numbers: &[i32]) -> (i32, i32) {
    let mut min = numbers[0];
    let mut max = numbers[0];
    for &n in numbers {
        if n < min { min = n; }
        if n > max { max = n; }
    }
    (min, max)
}

fn main() {
    println!("{}", add(3, 4));

    let (min, max) = min_max(&[3, 1, 4, 1, 5, 9, 2, 6]);
    println!("min={min}, max={max}");
}
```

The absence of a semicolon on the last line of a function is significant. In Rust, a line ending with a semicolon is a *statement* (it discards its value), while a line without a semicolon is an *expression* (its value is used). This is why `a + b` without a semicolon returns the sum, but `a + b;` with a semicolon would return `()` (the unit type, Rust's equivalent of void).

### Mini Project 1: Temperature Converter CLI

Build a command-line tool that converts between Celsius, Fahrenheit, and Kelvin.

```rust
use std::env;

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_kelvin(c: f64) -> f64 {
    c + 273.15
}

fn kelvin_to_celsius(k: f64) -> f64 {
    k - 273.15
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <value> <from_unit>", args[0]);
        eprintln!("Units: C, F, K");
        std::process::exit(1);
    }

    let value: f64 = args[1].parse().expect("Invalid number");
    let unit = args[2].to_uppercase();

    match unit.as_str() {
        "C" => {
            println!("{:.2}°C = {:.2}°F", value, celsius_to_fahrenheit(value));
            println!("{:.2}°C = {:.2}K", value, celsius_to_kelvin(value));
        }
        "F" => {
            let c = fahrenheit_to_celsius(value);
            println!("{:.2}°F = {:.2}°C", value, c);
            println!("{:.2}°F = {:.2}K", value, celsius_to_kelvin(c));
        }
        "K" => {
            let c = kelvin_to_celsius(value);
            println!("{:.2}K = {:.2}°C", value, c);
            println!("{:.2}K = {:.2}°F", value, celsius_to_fahrenheit(c));
        }
        _ => {
            eprintln!("Unknown unit: {}. Use C, F, or K.", unit);
            std::process::exit(1);
        }
    }
}
```

---

## Section 2: Ownership, Borrowing & Lifetimes

### The Core Problem Rust Solves

To understand ownership, you first need to understand the problem it solves. Memory management is one of the hardest problems in systems programming. There are two broad approaches:

**Manual memory management** (C, C++): The programmer explicitly allocates memory (`malloc`, `new`) and frees it (`free`, `delete`). This gives maximum control and performance, but it is error-prone. Forget to free memory and you have a memory leak. Free it twice and you have undefined behavior. Use memory after freeing it (use-after-free) and you have a security vulnerability. These bugs are notoriously hard to find and have been responsible for countless security exploits.

**Garbage collection** (Java, Python, Go, C#): A runtime system automatically tracks which memory is still in use and frees the rest. This eliminates the manual memory management bugs, but at a cost: the garbage collector runs periodically, causing unpredictable pauses. It also requires a runtime, which makes it unsuitable for embedded systems, OS kernels, and other low-level contexts.

Rust's ownership system is a third approach: **compile-time memory management**. The compiler tracks ownership of every piece of memory and inserts the appropriate free operations automatically — but it does this at compile time, with zero runtime overhead. The rules are enforced statically, so if your program compiles, it is guaranteed to be free of memory safety bugs.

### The Three Rules of Ownership

Rust's ownership system is built on three simple rules:

1. **Each value in Rust has exactly one owner.**
2. **There can only be one owner at a time.**
3. **When the owner goes out of scope, the value is dropped (freed).**

These rules sound simple, but their implications are profound. Let us explore them.

```rust
fn main() {
    // Rule 1: `s1` owns the String "hello"
    let s1 = String::from("hello");

    // Rule 2: When we assign s1 to s2, ownership MOVES to s2.
    // s1 is no longer valid. This is called a "move".
    let s2 = s1;

    // This would be a compile error:
    // println!("{}", s1); // ERROR: value borrowed here after move

    println!("{}", s2); // OK: s2 is the owner

    // Rule 3: When s2 goes out of scope (end of main), the String is dropped.
    // Rust automatically calls the `drop` function, which frees the heap memory.
}
```

The concept of *moving* is crucial. When you assign a value to another variable, Rust moves ownership rather than copying. This is different from most languages where assignment copies a reference or the value itself. The move semantics ensure that there is always exactly one owner, which makes it safe to free the memory when the owner goes out of scope.

### The Stack vs. The Heap

To understand why moves exist, you need to understand the difference between stack and heap memory.

The **stack** is fast, automatically managed memory. When you call a function, a stack frame is pushed; when the function returns, the frame is popped. Stack memory is limited in size and must have a known, fixed size at compile time. Integers, floats, booleans, and fixed-size arrays live on the stack.

The **heap** is slower, manually managed memory. It can hold data of arbitrary size and lives as long as you need it. `String`, `Vec`, and `Box` store their data on the heap. The heap allocation stores a pointer, a length, and a capacity on the stack, while the actual data lives on the heap.

Types that live entirely on the stack implement the `Copy` trait. For these types, assignment copies the value rather than moving it, because copying is cheap and there is no heap memory to worry about.

```rust
fn main() {
    // i32 implements Copy, so assignment copies the value
    let x = 5;
    let y = x; // x is COPIED, not moved
    println!("x={x}, y={y}"); // both are valid

    // String does NOT implement Copy (it has heap data)
    let s1 = String::from("hello");
    let s2 = s1; // s1 is MOVED to s2
    // println!("{s1}"); // ERROR: s1 was moved

    // To explicitly copy a String, use `.clone()`
    let s3 = String::from("world");
    let s4 = s3.clone(); // deep copy: both s3 and s4 are valid
    println!("s3={s3}, s4={s4}");

    // Types that implement Copy:
    // - All integer types (i32, u64, etc.)
    // - Floating point types (f32, f64)
    // - bool
    // - char
    // - Tuples of Copy types: (i32, f64) is Copy
    // - Arrays of Copy types: [i32; 5] is Copy
}
```

### Borrowing: References Without Ownership

Moving ownership is useful, but often you want to let a function use a value without taking ownership of it. This is called *borrowing*, and it is done through *references*.

A reference is like a pointer, but with a crucial guarantee: it is always valid. The Rust compiler ensures that references never outlive the data they point to, eliminating dangling pointer bugs entirely.

```rust
fn calculate_length(s: &String) -> usize {
    // `s` is a reference to a String. It does not own the String.
    // When `s` goes out of scope, the String is NOT dropped.
    s.len()
}

fn main() {
    let s1 = String::from("hello");

    // We pass a reference to s1 using `&`
    // s1 is "borrowed" by calculate_length, but s1 still owns the String
    let len = calculate_length(&s1);

    // s1 is still valid here because we only borrowed it
    println!("The length of '{s1}' is {len}.");
}
```

References come in two flavors: **shared references** (`&T`) and **mutable references** (`&mut T`).

The rules for references are:
- You can have **any number of shared references** (`&T`) at the same time.
- You can have **exactly one mutable reference** (`&mut T`) at a time.
- You **cannot have both** shared and mutable references at the same time.

These rules enforce what is sometimes called the "aliasing XOR mutability" principle: you can either have multiple readers OR one writer, but never both simultaneously. This eliminates data races at compile time.

```rust
fn main() {
    let mut s = String::from("hello");

    // Multiple shared references are fine
    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}"); // OK: both are read-only

    // After r1 and r2 are last used, we can create a mutable reference
    // (Rust uses "Non-Lexical Lifetimes" — references end at their last use,
    // not at the end of their scope)
    let r3 = &mut s;
    r3.push_str(", world");
    println!("{r3}");

    // This would be an error:
    // let r4 = &s;    // shared reference
    // let r5 = &mut s; // mutable reference — ERROR: cannot borrow as mutable
    //                  // because it is also borrowed as immutable
    // println!("{r4} {r5}");
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

### The Slice Type

Slices are a special kind of reference that refer to a contiguous sequence of elements in a collection, rather than the whole collection. They are a fundamental part of Rust's borrowing system.

```rust
fn first_word(s: &str) -> &str {
    // `s.as_bytes()` gives us the bytes of the string
    // We iterate with `.iter().enumerate()` to get (index, byte) pairs
    for (i, &byte) in s.as_bytes().iter().enumerate() {
        if byte == b' ' {
            return &s[0..i]; // return a slice up to the space
        }
    }
    &s[..] // return the whole string if no space found
}

fn main() {
    let sentence = String::from("hello world");

    // String slices: &str
    let word = first_word(&sentence);
    println!("First word: {word}");

    // Array slices: &[T]
    let a = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3]; // [2, 3]
    println!("Slice: {:?}", slice);

    // Slices are "fat pointers": they contain a pointer AND a length
    // This is why they are safe — the length is always known
}
```

### Lifetimes: Ensuring References Are Always Valid

Lifetimes are Rust's way of ensuring that references are always valid — that they never outlive the data they point to. In most cases, the compiler can infer lifetimes automatically (called *lifetime elision*). But in some cases, you need to annotate them explicitly.

A lifetime annotation does not change how long a reference lives — it just describes the relationship between the lifetimes of multiple references, so the compiler can verify that references are used safely.

```rust
// This function takes two string slices and returns the longer one.
// The lifetime annotation `'a` says: "the returned reference will be valid
// for as long as BOTH input references are valid."
// Without this annotation, the compiler cannot know which input the
// return value refers to, and therefore cannot verify safety.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is: {result}");
        // `result` is used here, while both string1 and string2 are valid
    }
    // If we tried to use `result` here, the compiler would reject it,
    // because string2 has been dropped and result might point to it.
}

// Lifetime annotations in structs
// This struct holds a reference, so it needs a lifetime annotation.
// It says: "an instance of ImportantExcerpt cannot outlive the reference
// it holds in the `part` field."
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention: {announcement}");
        self.part
    }
}
```

Lifetimes can feel abstract at first. The key insight is that they are not a runtime concept — they exist only in the compiler's analysis. They are a way of expressing constraints: "this reference must be valid for at least this long." The compiler uses these constraints to verify that your program is memory-safe.

### Mini Project 2: String Statistics

Build a function library that analyzes strings using references and slices.

```rust
/// Count the number of words in a string slice
fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

/// Find the longest word in a string, returning a slice of the original
fn longest_word(s: &str) -> &str {
    s.split_whitespace()
        .max_by_key(|w| w.len())
        .unwrap_or("")
}

/// Check if a string is a palindrome (ignoring case and spaces)
fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    let reversed: String = cleaned.chars().rev().collect();
    cleaned == reversed
}

/// Return the most common character in a string
fn most_common_char(s: &str) -> Option<char> {
    let mut counts = std::collections::HashMap::new();
    for c in s.chars().filter(|c| !c.is_whitespace()) {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts.into_iter().max_by_key(|(_, count)| *count).map(|(c, _)| c)
}

fn main() {
    let text = "A man a plan a canal Panama";

    println!("Text: \"{}\"", text);
    println!("Word count: {}", word_count(text));
    println!("Longest word: {}", longest_word(text));
    println!("Is palindrome: {}", is_palindrome(text));

    if let Some(c) = most_common_char(text) {
        println!("Most common char: '{c}'");
    }
}
```

---

## Section 3: Structs, Enums & Pattern Matching

### Structs: Building Custom Data Types

Structs are Rust's primary mechanism for creating custom data types. They are similar to structs in C, classes in Python/Java (without methods by default), or records in functional languages. A struct groups related data together under a single name.

The philosophy behind structs in Rust is that data and behavior are separate by default. You define the data in the struct, and then you add behavior through `impl` blocks. This separation makes it easy to understand what data a type holds without having to wade through methods.

```rust
// A basic struct definition
// By convention, struct names use PascalCase
struct Rectangle {
    width: f64,
    height: f64,
}

// An `impl` block adds methods to the struct
impl Rectangle {
    // Associated function (like a static method) — called with Rectangle::new()
    // By convention, `new` is used to create instances, but it is not special
    fn new(width: f64, height: f64) -> Self {
        // `Self` refers to the type being implemented (Rectangle)
        Self { width, height }
    }

    // Method: takes `&self` (shared reference to the instance)
    // `self` is the Rust equivalent of `this` in Java/Python
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }

    // Mutable method: takes `&mut self` to modify the instance
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    // Consuming method: takes `self` (ownership), consuming the instance
    // After calling this, the original Rectangle is gone
    fn into_square(self) -> Rectangle {
        let side = (self.width * self.height).sqrt();
        Rectangle { width: side, height: side }
    }
}

// Implementing the Display trait allows us to use `{}` in format strings
use std::fmt;
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle({}x{})", self.width, self.height)
    }
}

// Deriving common traits automatically
// Debug: enables {:?} formatting
// Clone: enables .clone()
// PartialEq: enables == and !=
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

fn main() {
    let mut rect = Rectangle::new(10.0, 5.0);
    println!("{rect}");
    println!("Area: {}", rect.area());
    println!("Is square: {}", rect.is_square());

    rect.scale(2.0);
    println!("After scaling: {rect}");

    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    println!("Distance: {}", p1.distance_to(&p2));
    println!("Debug: {:?}", p1);
}
```

### Tuple Structs and Unit Structs

Rust has two other kinds of structs beyond the named-field struct shown above.

**Tuple structs** are structs with unnamed fields. They are useful when you want to give a type a name for type safety, but the field names would be redundant.

**Unit structs** have no fields at all. They are useful as markers or for implementing traits on types that carry no data.

```rust
// Tuple struct: fields accessed by index (.0, .1, etc.)
struct Meters(f64);
struct Kilograms(f64);

// This prevents accidentally mixing up units — the compiler will reject
// passing Kilograms where Meters is expected, even though both wrap f64
fn calculate_bmi(weight: Kilograms, height: Meters) -> f64 {
    weight.0 / (height.0 * height.0)
}

// Unit struct: no fields
struct AlwaysEqual;

fn main() {
    let height = Meters(1.75);
    let weight = Kilograms(70.0);
    println!("BMI: {:.1}", calculate_bmi(weight, height));

    // This would be a compile error — type safety!
    // calculate_bmi(height, weight); // ERROR: wrong types
}
```

### Enums: Modeling Alternatives

Enums in Rust are far more powerful than enums in most other languages. In C or Java, an enum is essentially a named integer. In Rust, each variant of an enum can hold different data. This makes Rust enums equivalent to *algebraic data types* or *tagged unions* in other languages.

The power of Rust enums comes from their ability to represent "one of several possible things, each of which may have different associated data." This is perfect for modeling states, results, options, and any situation where a value can be one of several distinct cases.

```rust
// A simple enum (like C enums)
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

// An enum with data in each variant
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle { .. } => "circle",
            Shape::Rectangle { .. } => "rectangle",
            Shape::Triangle { .. } => "triangle",
        }
    }
}

// Enums can also hold tuple-style data
#[derive(Debug)]
enum Message {
    Quit,                       // no data
    Move { x: i32, y: i32 },   // named fields
    Write(String),              // single value
    ChangeColor(u8, u8, u8),    // tuple
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to ({x}, {y})"),
        Message::Write(text) => println!("Writing: {text}"),
        Message::ChangeColor(r, g, b) => println!("Color: rgb({r},{g},{b})"),
    }
}
```

### Option<T>: Rust's Answer to Null

One of the most famous design decisions in Rust is the absence of `null`. Tony Hoare, who invented null references in 1965, called it his "billion-dollar mistake" — null references have caused countless bugs, crashes, and security vulnerabilities.

Rust replaces null with the `Option<T>` enum, which is defined in the standard library as:

```rust
enum Option<T> {
    Some(T),  // there is a value, and it is T
    None,     // there is no value
}
```

The key difference from null is that `Option<T>` is a type. The compiler forces you to handle both cases — you cannot accidentally use a `None` value as if it were `Some`. This eliminates null pointer dereferences entirely.

```rust
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None // no even number found
}

fn main() {
    let numbers = vec![1, 3, 5, 4, 7];

    // Pattern matching on Option
    match find_first_even(&numbers) {
        Some(n) => println!("Found even number: {n}"),
        None => println!("No even numbers found"),
    }

    // `if let` — a concise way to match one pattern
    if let Some(n) = find_first_even(&numbers) {
        println!("First even: {n}");
    }

    // Option methods
    let maybe_value: Option<i32> = Some(42);

    // unwrap_or: provide a default if None
    let value = maybe_value.unwrap_or(0);

    // map: transform the inner value if Some
    let doubled = maybe_value.map(|n| n * 2);

    // and_then: chain operations that might fail (flatMap)
    let result = maybe_value
        .filter(|&n| n > 10)
        .map(|n| n.to_string());

    println!("value={value}, doubled={doubled:?}, result={result:?}");

    // unwrap() panics if None — use sparingly, only when you are certain
    let definitely_some: Option<i32> = Some(5);
    let inner = definitely_some.unwrap(); // OK here, but risky in general

    // expect() is like unwrap() but with a custom panic message
    let inner2 = definitely_some.expect("This should always be Some");
}
```

### Pattern Matching: The Heart of Rust Control Flow

Pattern matching with `match` is one of Rust's most powerful features. It is like a `switch` statement on steroids — it can match on values, types, struct fields, enum variants, ranges, and more, and the compiler ensures that all cases are handled (exhaustiveness checking).

```rust
fn describe_number(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1 | 2 | 3 => "small positive",
        4..=9 => "medium positive",
        10..=99 => "large positive",
        100.. => "very large",
        i32::MIN..=-1 => "negative",
        // The compiler knows all i32 values are covered
    }
}

#[derive(Debug)]
struct Point { x: i32, y: i32 }

fn classify_point(p: &Point) -> &str {
    match p {
        Point { x: 0, y: 0 } => "origin",
        Point { x: 0, .. } => "on y-axis",
        Point { y: 0, .. } => "on x-axis",
        Point { x, y } if x == y => "on diagonal",
        Point { x, y } if x > &0 && y > &0 => "first quadrant",
        _ => "other",
    }
}

fn main() {
    // Matching with guards (if conditions)
    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("equal"),
        (x, y) if x + y == 0 => println!("opposites"),
        (x, _) if x % 2 == 0 => println!("first is even"),
        _ => println!("other"),
    }

    // Destructuring in let statements
    let (a, b, c) = (1, 2, 3);
    let Point { x, y } = Point { x: 10, y: 20 };

    // @ bindings: bind a value while also matching a pattern
    let n = 15;
    match n {
        x @ 1..=12 => println!("Month {x}"),
        x @ 13..=19 => println!("Teen {x}"),
        x => println!("Other: {x}"),
    }

    // while let: loop while a pattern matches
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("Popped: {top}");
    }
}
```

### Mini Project 3: Shape Calculator

```rust
use std::f64::consts::PI;

#[derive(Debug, Clone)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { a: f64, b: f64, c: f64 },
    RegularPolygon { sides: u32, side_length: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { a, b, c } => {
                // Heron's formula
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
            Shape::RegularPolygon { sides, side_length } => {
                let n = *sides as f64;
                (n * side_length * side_length) / (4.0 * (PI / n).tan())
            }
        }
    }

    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle { radius } => 2.0 * PI * radius,
            Shape::Rectangle { width, height } => 2.0 * (width + height),
            Shape::Triangle { a, b, c } => a + b + c,
            Shape::RegularPolygon { sides, side_length } => {
                *sides as f64 * side_length
            }
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle { .. } => "Circle",
            Shape::Rectangle { .. } => "Rectangle",
            Shape::Triangle { .. } => "Triangle",
            Shape::RegularPolygon { .. } => "Regular Polygon",
        }
    }
}

fn main() {
    let shapes: Vec<Shape> = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { width: 4.0, height: 6.0 },
        Shape::Triangle { a: 3.0, b: 4.0, c: 5.0 },
        Shape::RegularPolygon { sides: 6, side_length: 3.0 },
    ];

    let mut total_area = 0.0;
    for shape in &shapes {
        let area = shape.area();
        total_area += area;
        println!("{}: area={:.2}, perimeter={:.2}", shape.name(), area, shape.perimeter());
    }
    println!("Total area: {:.2}", total_area);

    // Find the largest shape
    if let Some(largest) = shapes.iter().max_by(|a, b| {
        a.area().partial_cmp(&b.area()).unwrap()
    }) {
        println!("Largest: {} with area {:.2}", largest.name(), largest.area());
    }
}
```

---

## Section 4: Smart Pointers & Interior Mutability

### What Are Smart Pointers?

A pointer is a variable that holds a memory address. In C, raw pointers are just integers — they carry no information about ownership, lifetime, or validity. Rust's references (`&T` and `&mut T`) are safe pointers with compile-time guarantees, but they are not the only kind of pointer in Rust.

*Smart pointers* are data structures that act like pointers but also have additional metadata and capabilities. They typically own the data they point to, and they implement the `Drop` trait to clean up when they go out of scope. The most important smart pointers in Rust are `Box<T>`, `Rc<T>`, `Arc<T>`, `RefCell<T>`, and `Mutex<T>`.

Understanding when to use each one is a key skill for intermediate and advanced Rust programming.

### Box<T>: Heap Allocation

`Box<T>` is the simplest smart pointer. It allocates a value on the heap and gives you ownership of it. When the `Box` goes out of scope, both the `Box` and the heap-allocated value are dropped.

You use `Box<T>` when:
- You have a type whose size is not known at compile time (like a recursive type)
- You want to transfer ownership of a large amount of data without copying it
- You want to own a value that implements a trait, without caring about the concrete type

```rust
// Recursive types need Box because the compiler needs to know the size
// of a type at compile time. Without Box, this would be infinite size.
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), // Box breaks the infinite size cycle
    Nil,
}

impl List {
    fn new() -> Self {
        List::Nil
    }

    fn prepend(self, value: i32) -> Self {
        List::Cons(value, Box::new(self))
    }

    fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, tail) => 1 + tail.len(),
        }
    }
}

fn main() {
    // Box for heap allocation
    let b = Box::new(5);
    println!("b = {b}"); // Box<i32> derefs to i32 automatically

    // Box for recursive types
    let list = List::new()
        .prepend(3)
        .prepend(2)
        .prepend(1);
    println!("List length: {}", list.len());

    // Box for trait objects (dynamic dispatch)
    // We can store different types that implement the same trait
    trait Animal {
        fn speak(&self) -> &str;
    }

    struct Dog;
    struct Cat;

    impl Animal for Dog {
        fn speak(&self) -> &str { "Woof!" }
    }

    impl Animal for Cat {
        fn speak(&self) -> &str { "Meow!" }
    }

    // Vec<Box<dyn Animal>> can hold Dogs and Cats together
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
        Box::new(Dog),
    ];

    for animal in &animals {
        println!("{}", animal.speak());
    }
}
```

### Rc<T>: Reference Counting for Single-Threaded Shared Ownership

Sometimes you need multiple owners of the same data. The ownership rules say there can only be one owner, but `Rc<T>` (Reference Counted) works around this by keeping a count of how many references exist. The data is only dropped when the count reaches zero.

`Rc<T>` is for **single-threaded** scenarios only. It is not safe to send across threads. For multi-threaded shared ownership, use `Arc<T>`.

```rust
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<Node>>,
}

fn main() {
    // Create a shared node
    let shared = Rc::new(Node { value: 42, children: vec![] });

    println!("Reference count: {}", Rc::strong_count(&shared)); // 1

    // Clone an Rc increments the reference count, not the data
    let clone1 = Rc::clone(&shared);
    let clone2 = Rc::clone(&shared);

    println!("Reference count: {}", Rc::strong_count(&shared)); // 3

    // All three point to the same Node
    println!("shared.value = {}", shared.value);
    println!("clone1.value = {}", clone1.value);
    println!("clone2.value = {}", clone2.value);

    drop(clone1);
    println!("After dropping clone1: {}", Rc::strong_count(&shared)); // 2

    // When all Rc clones are dropped, the Node is freed
}
```

### Arc<T>: Atomic Reference Counting for Multi-Threaded Shared Ownership

`Arc<T>` (Atomically Reference Counted) is the thread-safe version of `Rc<T>`. It uses atomic operations to update the reference count, which is safe across threads but slightly slower than `Rc<T>`.

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);

    let mut handles = vec![];

    for i in 0..3 {
        // Clone the Arc to share ownership with the new thread
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            // Each thread can read the data safely
            println!("Thread {i}: sum = {}", data_clone.iter().sum::<i32>());
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Main thread: {:?}", data);
}
```

### RefCell<T>: Interior Mutability

Rust's borrowing rules are enforced at compile time, which is great for safety but sometimes too restrictive. `RefCell<T>` provides *interior mutability* — it allows you to mutate data even when you only have a shared reference, by moving the borrow checking to runtime.

This sounds dangerous, but `RefCell<T>` still enforces the borrowing rules — it just does so at runtime instead of compile time. If you violate the rules (e.g., try to get two mutable borrows), it panics instead of causing undefined behavior.

Use `RefCell<T>` when you know your code is correct but the compiler cannot verify it statically.

```rust
use std::cell::RefCell;
use std::rc::Rc;

// A common pattern: Rc<RefCell<T>> for shared mutable data
// in single-threaded code
#[derive(Debug)]
struct SharedCounter {
    count: Rc<RefCell<i32>>,
}

impl SharedCounter {
    fn new() -> Self {
        Self { count: Rc::new(RefCell::new(0)) }
    }

    fn clone_handle(&self) -> Self {
        Self { count: Rc::clone(&self.count) }
    }

    fn increment(&self) {
        // borrow_mut() gives a mutable reference at runtime
        *self.count.borrow_mut() += 1;
    }

    fn value(&self) -> i32 {
        // borrow() gives a shared reference at runtime
        *self.count.borrow()
    }
}

fn main() {
    let counter = SharedCounter::new();
    let counter2 = counter.clone_handle();

    counter.increment();
    counter.increment();
    counter2.increment();

    println!("Counter value: {}", counter.value()); // 3
    println!("Counter2 value: {}", counter2.value()); // 3 (same data!)
}
```

### Weak<T>: Breaking Reference Cycles

A potential problem with `Rc<T>` is reference cycles — if two `Rc` values point to each other, their reference counts will never reach zero, causing a memory leak. `Weak<T>` solves this by creating a non-owning reference that does not increment the strong reference count.

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,  // Weak reference to parent (non-owning)
    children: RefCell<Vec<Rc<Node>>>, // Strong references to children (owning)
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // Set leaf's parent to branch (using a Weak reference)
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // Upgrade a Weak reference to get an Rc (returns Option<Rc<T>>)
    if let Some(parent) = leaf.parent.borrow().upgrade() {
        println!("Leaf's parent value: {}", parent.value);
    }

    println!("Branch strong count: {}", Rc::strong_count(&branch)); // 1
    println!("Leaf strong count: {}", Rc::strong_count(&leaf)); // 2 (branch + main)
}
```

### Mini Project 4: Observer Pattern with Rc<RefCell<T>>

```rust
use std::rc::Rc;
use std::cell::RefCell;

trait Observer {
    fn update(&self, event: &str, value: f64);
}

struct EventBus {
    observers: Vec<Rc<dyn Observer>>,
}

impl EventBus {
    fn new() -> Self {
        Self { observers: vec![] }
    }

    fn subscribe(&mut self, observer: Rc<dyn Observer>) {
        self.observers.push(observer);
    }

    fn publish(&self, event: &str, value: f64) {
        for observer in &self.observers {
            observer.update(event, value);
        }
    }
}

struct Logger {
    name: String,
    log: RefCell<Vec<String>>,
}

impl Logger {
    fn new(name: &str) -> Rc<Self> {
        Rc::new(Self {
            name: name.to_string(),
            log: RefCell::new(vec![]),
        })
    }

    fn print_log(&self) {
        println!("Log for {}:", self.name);
        for entry in self.log.borrow().iter() {
            println!("  {entry}");
        }
    }
}

impl Observer for Logger {
    fn update(&self, event: &str, value: f64) {
        let entry = format!("[{}] {}: {:.2}", self.name, event, value);
        self.log.borrow_mut().push(entry);
    }
}

fn main() {
    let mut bus = EventBus::new();

    let logger1 = Logger::new("FileLogger");
    let logger2 = Logger::new("ConsoleLogger");

    bus.subscribe(Rc::clone(&logger1) as Rc<dyn Observer>);
    bus.subscribe(Rc::clone(&logger2) as Rc<dyn Observer>);

    bus.publish("temperature", 23.5);
    bus.publish("humidity", 65.0);
    bus.publish("pressure", 1013.25);

    logger1.print_log();
    logger2.print_log();
}
```

---

## Section 5: Error Handling

### Rust's Philosophy on Errors

Error handling is one of the areas where Rust's design philosophy is most clearly expressed. Rust distinguishes between two kinds of errors:

**Recoverable errors** are situations where the program can reasonably continue after the error — a file not found, a network timeout, invalid user input. These are represented by the `Result<T, E>` type.

**Unrecoverable errors** are bugs — situations that should never happen in a correct program, like accessing an array out of bounds or integer overflow in debug mode. These cause a *panic*, which unwinds the stack and terminates the program (or the thread).

This distinction is important. In languages like Java or Python, exceptions are used for both kinds of errors, which can make it hard to know which functions might fail and how. In Rust, if a function can fail in a recoverable way, its return type tells you so — you cannot ignore the error without explicitly deciding to.

### Result<T, E>: Explicit Error Handling

`Result<T, E>` is an enum defined as:

```rust
enum Result<T, E> {
    Ok(T),   // success, contains the value
    Err(E),  // failure, contains the error
}
```

Every function that can fail returns a `Result`. The caller must handle both cases. This makes error handling explicit and visible in the code.

```rust
use std::fs;
use std::num::ParseIntError;

// A function that can fail returns Result
fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let n = s.trim().parse::<i32>()?; // `?` propagates errors
    Ok(n * 2)
}

fn read_number_from_file(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?; // ? propagates io::Error
    let number = content.trim().parse::<i32>()?; // ? propagates ParseIntError
    Ok(number)
}

fn main() {
    // Pattern matching on Result
    match parse_and_double("21") {
        Ok(n) => println!("Result: {n}"),
        Err(e) => println!("Error: {e}"),
    }

    // if let for one case
    if let Ok(n) = parse_and_double("21") {
        println!("Got: {n}");
    }

    // Result methods
    let result: Result<i32, &str> = Ok(42);

    // unwrap_or: default value on error
    let value = result.unwrap_or(0);

    // map: transform the Ok value
    let doubled = result.map(|n| n * 2);

    // map_err: transform the Err value
    let stringified = result.map_err(|e| format!("Error: {e}"));

    // and_then: chain fallible operations
    let chained = result.and_then(|n| {
        if n > 0 { Ok(n) } else { Err("must be positive") }
    });

    println!("{value}, {doubled:?}, {stringified:?}, {chained:?}");
}
```

### The ? Operator: Ergonomic Error Propagation

The `?` operator is syntactic sugar for a common pattern: if the result is `Ok`, unwrap it; if it is `Err`, return the error from the current function. It makes error propagation concise and readable.

```rust
use std::fs::File;
use std::io::{self, Read};

// Without ?: verbose and repetitive
fn read_username_verbose(path: &str) -> Result<String, io::Error> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// With ?: clean and readable
fn read_username(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// Even more concise using method chaining
fn read_username_concise(path: &str) -> Result<String, io::Error> {
    std::fs::read_to_string(path)
}
```

### Custom Error Types

For real applications, you will want to define your own error types. This gives you control over the error messages and allows you to add context to errors.

```rust
use std::fmt;
use std::num::ParseIntError;

// A custom error enum for our application
#[derive(Debug)]
enum AppError {
    ParseError(ParseIntError),
    ValidationError(String),
    IoError(std::io::Error),
}

// Implement Display for human-readable error messages
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ParseError(e) => write!(f, "Parse error: {e}"),
            AppError::ValidationError(msg) => write!(f, "Validation error: {msg}"),
            AppError::IoError(e) => write!(f, "I/O error: {e}"),
        }
    }
}

// Implement the Error trait (required for compatibility with other error handling)
impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::ParseError(e) => Some(e),
            AppError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

// From implementations allow ? to convert between error types automatically
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::ParseError(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::IoError(e)
    }
}

fn parse_positive(s: &str) -> Result<u32, AppError> {
    let n: i32 = s.trim().parse()?; // ParseIntError auto-converts to AppError
    if n < 0 {
        return Err(AppError::ValidationError(
            format!("{n} is not positive")
        ));
    }
    Ok(n as u32)
}
```

### The anyhow and thiserror Crates

In practice, most Rust developers use two popular crates for error handling:

- **`thiserror`**: Makes it easy to define custom error types with less boilerplate
- **`anyhow`**: Provides a flexible error type for applications where you just want to propagate errors without defining custom types

```rust
// With thiserror (in a library)
use thiserror::Error;

#[derive(Error, Debug)]
enum DatabaseError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Query error: {query} — {source}")]
    QueryError {
        query: String,
        #[source]
        source: Box<dyn std::error::Error>,
    },

    #[error("Record not found: id={id}")]
    NotFound { id: u64 },
}

// With anyhow (in an application)
use anyhow::{Context, Result, bail, ensure};

fn process_file(path: &str) -> Result<()> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {path}"))?;

    ensure!(!content.is_empty(), "File is empty: {path}");

    let lines: Vec<&str> = content.lines().collect();
    if lines.len() < 2 {
        bail!("File must have at least 2 lines, found {}", lines.len());
    }

    println!("Processed {} lines", lines.len());
    Ok(())
}
```

### Mini Project 5: Robust CSV Parser

```rust
use std::fmt;
use std::num::ParseFloatError;

#[derive(Debug)]
enum CsvError {
    InvalidHeader(String),
    InvalidRow { line: usize, reason: String },
    ParseError { line: usize, field: String, source: ParseFloatError },
}

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CsvError::InvalidHeader(h) => write!(f, "Invalid header: {h}"),
            CsvError::InvalidRow { line, reason } => {
                write!(f, "Invalid row at line {line}: {reason}")
            }
            CsvError::ParseError { line, field, source } => {
                write!(f, "Parse error at line {line}, field '{field}': {source}")
            }
        }
    }
}

impl std::error::Error for CsvError {}

#[derive(Debug)]
struct Record {
    name: String,
    value: f64,
    category: String,
}

fn parse_csv(input: &str) -> Result<Vec<Record>, CsvError> {
    let mut lines = input.lines().enumerate();

    // Parse header
    let (_, header) = lines.next()
        .ok_or_else(|| CsvError::InvalidHeader("Empty input".to_string()))?;

    let expected = "name,value,category";
    if header.trim() != expected {
        return Err(CsvError::InvalidHeader(
            format!("Expected '{expected}', got '{header}'")
        ));
    }

    let mut records = Vec::new();

    for (line_num, line) in lines {
        let line_num = line_num + 1; // 1-indexed for user display
        let fields: Vec<&str> = line.split(',').collect();

        if fields.len() != 3 {
            return Err(CsvError::InvalidRow {
                line: line_num,
                reason: format!("Expected 3 fields, got {}", fields.len()),
            });
        }

        let value = fields[1].trim().parse::<f64>().map_err(|e| {
            CsvError::ParseError {
                line: line_num,
                field: "value".to_string(),
                source: e,
            }
        })?;

        records.push(Record {
            name: fields[0].trim().to_string(),
            value,
            category: fields[2].trim().to_string(),
        });
    }

    Ok(records)
}

fn main() {
    let csv = "name,value,category
Alice,95.5,A
Bob,82.0,B
Charlie,91.3,A";

    match parse_csv(csv) {
        Ok(records) => {
            for r in &records {
                println!("{}: {:.1} ({})", r.name, r.value, r.category);
            }
            let avg = records.iter().map(|r| r.value).sum::<f64>() / records.len() as f64;
            println!("Average: {:.1}", avg);
        }
        Err(e) => eprintln!("Error: {e}"),
    }
}
```

---

## Section 6: Collections & Iterators

### The Standard Collections

Rust's standard library provides a rich set of collection types. Choosing the right collection for your use case is important for both correctness and performance.

**`Vec<T>`** is the most commonly used collection — a growable array stored on the heap. It is the go-to choice when you need a list of items and you do not know the size at compile time.

**`HashMap<K, V>`** is a hash map — a collection of key-value pairs with O(1) average-case lookup. It is the go-to choice when you need to associate values with keys.

**`HashSet<T>`** is a set — a collection of unique values. It is backed by a `HashMap` where the values are `()`.

**`BTreeMap<K, V>`** and **`BTreeSet<T>`** are sorted versions of `HashMap` and `HashSet`, backed by a B-tree. They are slower for random access but maintain sorted order and support range queries.

**`VecDeque<T>`** is a double-ended queue — efficient for pushing and popping from both ends.

**`LinkedList<T>`** is a doubly-linked list. It is rarely the right choice in Rust because cache locality makes `Vec` faster in most cases.

```rust
use std::collections::{HashMap, HashSet, BTreeMap, VecDeque};

fn main() {
    // Vec<T>: growable array
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.extend([4, 5, 6]);

    println!("Vec: {:?}", v);
    println!("Length: {}", v.len());
    println!("First: {:?}", v.first());
    println!("Last: {:?}", v.last());

    // Slicing
    let slice = &v[1..4]; // [2, 3, 4]

    // Retain only even numbers
    v.retain(|&x| x % 2 == 0);
    println!("Even only: {:?}", v);

    // HashMap<K, V>
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("Alice".to_string(), 95);
    scores.insert("Bob".to_string(), 82);

    // entry API: insert if not present, or modify existing
    scores.entry("Charlie".to_string()).or_insert(0);
    *scores.entry("Alice".to_string()).or_insert(0) += 5;

    for (name, score) in &scores {
        println!("{name}: {score}");
    }

    // HashSet<T>
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(2); // duplicate, ignored

    let other: HashSet<i32> = [2, 3, 4].into_iter().collect();

    // Set operations
    let union: HashSet<_> = set.union(&other).collect();
    let intersection: HashSet<_> = set.intersection(&other).collect();
    let difference: HashSet<_> = set.difference(&other).collect();

    println!("Union: {:?}", union);
    println!("Intersection: {:?}", intersection);

    // VecDeque: efficient push/pop from both ends
    let mut deque: VecDeque<i32> = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    println!("Deque: {:?}", deque); // [0, 1, 2]
    deque.pop_front(); // removes 0
}
```

### Iterators: The Functional Heart of Rust

Iterators are one of Rust's most powerful and idiomatic features. An iterator is any type that implements the `Iterator` trait, which requires a single method: `next()`, which returns `Option<Item>`.

The power of iterators comes from their *adapter* methods — methods that transform one iterator into another. These adapters are lazy: they do not do any work until you consume the iterator. This means you can chain many adapters together without creating intermediate collections, and the compiler can often optimize the entire chain into a single tight loop.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map: transform each element
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();

    // filter: keep elements matching a predicate
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();

    // filter_map: filter and transform in one step
    let even_squares: Vec<i32> = numbers.iter()
        .filter_map(|&x| if x % 2 == 0 { Some(x * x) } else { None })
        .collect();

    // fold: reduce to a single value (like reduce in other languages)
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    let product = numbers.iter().fold(1, |acc, &x| acc * x);

    // sum and product are also available directly
    let sum2: i32 = numbers.iter().sum();

    // any and all: short-circuit predicates
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    let all_positive = numbers.iter().all(|&x| x > 0);

    // find: return the first matching element
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);

    // position: return the index of the first match
    let pos = numbers.iter().position(|&x| x == 5);

    // take and skip
    let first_three: Vec<&i32> = numbers.iter().take(3).collect();
    let skip_three: Vec<&i32> = numbers.iter().skip(3).collect();

    // zip: combine two iterators element-by-element
    let letters = vec!['a', 'b', 'c'];
    let zipped: Vec<(i32, char)> = numbers.iter()
        .copied()
        .zip(letters.iter().copied())
        .collect();

    // chain: concatenate two iterators
    let more = vec![11, 12, 13];
    let chained: Vec<i32> = numbers.iter()
        .chain(more.iter())
        .copied()
        .collect();

    // flat_map: map and flatten
    let words = vec!["hello world", "foo bar"];
    let all_words: Vec<&str> = words.iter()
        .flat_map(|s| s.split_whitespace())
        .collect();

    // enumerate: add indices
    for (i, &n) in numbers.iter().enumerate() {
        if i < 3 { println!("{i}: {n}"); }
    }

    // Chaining multiple adapters — all lazy, compiled to one loop
    let result: Vec<String> = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .take(3)
        .map(|x| format!("{}^2={}", (x as f64).sqrt() as i32, x))
        .collect();

    println!("{:?}", result);
}
```

### Implementing Your Own Iterator

Understanding how to implement the `Iterator` trait gives you deep insight into how iterators work and lets you create custom lazy sequences.

```rust
// A Fibonacci iterator that generates numbers on demand
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Self { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(self.a) // infinite iterator — always returns Some
    }
}

fn main() {
    // Take the first 10 Fibonacci numbers
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("{:?}", fibs);

    // Find the first Fibonacci number greater than 1000
    let big_fib = Fibonacci::new().find(|&n| n > 1000);
    println!("First Fibonacci > 1000: {:?}", big_fib);

    // Sum of Fibonacci numbers less than 100
    let sum: u64 = Fibonacci::new().take_while(|&n| n < 100).sum();
    println!("Sum of Fibonacci < 100: {sum}");
}
```

### Mini Project 6: Word Frequency Analyzer

```rust
use std::collections::HashMap;

fn analyze_text(text: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();

    text.split_whitespace()
        .map(|word| {
            word.chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| c.to_lowercase().next().unwrap())
                .collect::<String>()
        })
        .filter(|word| !word.is_empty())
        .for_each(|word| {
            *freq.entry(word).or_insert(0) += 1;
        });

    freq
}

fn top_n_words(freq: &HashMap<String, usize>, n: usize) -> Vec<(&String, &usize)> {
    let mut pairs: Vec<_> = freq.iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    pairs.into_iter().take(n).collect()
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog
    the dog barked at the fox and the fox ran away
    the quick fox was too quick for the lazy dog";

    let freq = analyze_text(text);

    println!("Word frequencies:");
    for (word, count) in top_n_words(&freq, 5) {
        println!("  {:15} {}", word, "*".repeat(*count));
    }

    println!("\nTotal unique words: {}", freq.len());
    println!("Total words: {}", freq.values().sum::<usize>());

    // Words that appear more than once
    let repeated: Vec<_> = freq.iter()
        .filter(|(_, &count)| count > 1)
        .map(|(word, _)| word.as_str())
        .collect();
    println!("Repeated words: {:?}", repeated);
}
```

---

## Section 7: Traits & Generics

### Traits: Defining Shared Behavior

Traits are Rust's mechanism for defining shared behavior across types. They are similar to interfaces in Java or Go, or abstract base classes in Python, but with some important differences. A trait defines a set of methods that a type must implement. Any type that implements a trait can be used wherever that trait is expected.

The key insight about traits is that they separate *what a type can do* from *what a type is*. This is the foundation of Rust's polymorphism system.

```rust
// Define a trait
trait Summary {
    // Required method: implementors must provide this
    fn summarize_author(&self) -> String;

    // Default method: implementors can override this, but don't have to
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct NewsArticle {
    title: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    // Override the default implementation
    fn summarize(&self) -> String {
        format!("{}, by {} — {}", self.title, self.author, &self.content[..50])
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // Uses the default summarize() implementation
}

// Trait bounds: require a type to implement a trait
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Equivalent using where clause (more readable for complex bounds)
fn notify_verbose<T>(item: &T) where T: Summary {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
fn notify_display<T: Summary + std::fmt::Display>(item: &T) {
    println!("{item}");
    println!("{}", item.summarize());
}
```

### Generics: Writing Code That Works for Any Type

Generics allow you to write code that works for multiple types without duplicating it. They are similar to templates in C++ or generics in Java, but Rust's generics are *monomorphized* at compile time — the compiler generates a separate copy of the code for each concrete type used, resulting in zero-overhead abstraction.

```rust
// A generic function: works for any type T that implements PartialOrd
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// A generic struct
#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
}

// Conditional implementation: only implement cmp_display if T is Display + PartialOrd
impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.first >= self.second {
            println!("First is larger: {}", self.first);
        } else {
            println!("Second is larger: {}", self.second);
        }
    }
}

// Generic struct with multiple type parameters
#[derive(Debug)]
struct KeyValue<K, V> {
    key: K,
    value: V,
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));

    let pair = Pair::new(5, 10);
    pair.cmp_display();
}
```

### Trait Objects: Dynamic Dispatch

Sometimes you need to work with values of different types that implement the same trait, but you do not know the types at compile time. This is where *trait objects* come in. A trait object (`dyn Trait`) is a fat pointer that contains a pointer to the data and a pointer to a vtable (virtual dispatch table).

Trait objects enable *dynamic dispatch* — the method to call is determined at runtime based on the actual type. This is slower than static dispatch (generics) but more flexible.

```rust
trait Draw {
    fn draw(&self);
    fn bounding_box(&self) -> (f64, f64, f64, f64); // (x, y, width, height)
}

struct Circle {
    x: f64, y: f64, radius: f64,
}

struct Rectangle {
    x: f64, y: f64, width: f64, height: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing circle at ({}, {}) with radius {}", self.x, self.y, self.radius);
    }
    fn bounding_box(&self) -> (f64, f64, f64, f64) {
        (self.x - self.radius, self.y - self.radius, self.radius * 2.0, self.radius * 2.0)
    }
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("Drawing rectangle at ({}, {}) size {}x{}", self.x, self.y, self.width, self.height);
    }
    fn bounding_box(&self) -> (f64, f64, f64, f64) {
        (self.x, self.y, self.width, self.height)
    }
}

// A canvas that can hold any drawable object
struct Canvas {
    components: Vec<Box<dyn Draw>>, // trait object: Box<dyn Draw>
}

impl Canvas {
    fn new() -> Self {
        Self { components: vec![] }
    }

    fn add(&mut self, component: Box<dyn Draw>) {
        self.components.push(component);
    }

    fn render(&self) {
        for component in &self.components {
            component.draw(); // dynamic dispatch: called via vtable
        }
    }
}

fn main() {
    let mut canvas = Canvas::new();
    canvas.add(Box::new(Circle { x: 0.0, y: 0.0, radius: 5.0 }));
    canvas.add(Box::new(Rectangle { x: 1.0, y: 1.0, width: 10.0, height: 5.0 }));
    canvas.add(Box::new(Circle { x: 3.0, y: 3.0, radius: 2.0 }));
    canvas.render();
}
```

### Important Standard Library Traits

The Rust standard library defines many important traits that you should know:

```rust
use std::fmt;
use std::ops::{Add, Mul, Neg};

// Display: for user-facing string representation ({})
// Debug: for developer-facing string representation ({:?})
// These are the most commonly implemented traits

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

// Operator overloading via traits
impl Add for Vector2D {
    type Output = Vector2D;
    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Mul<f64> for Vector2D {
    type Output = Vector2D;
    fn mul(self, scalar: f64) -> Vector2D {
        Vector2D { x: self.x * scalar, y: self.y * scalar }
    }
}

impl Neg for Vector2D {
    type Output = Vector2D;
    fn neg(self) -> Vector2D {
        Vector2D { x: -self.x, y: -self.y }
    }
}

impl Vector2D {
    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn dot(&self, other: &Vector2D) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

// From/Into: type conversions
impl From<(f64, f64)> for Vector2D {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

fn main() {
    let v1 = Vector2D { x: 1.0, y: 2.0 };
    let v2 = Vector2D { x: 3.0, y: 4.0 };

    println!("v1 = {v1}");
    println!("v2 = {v2}");
    println!("v1 + v2 = {}", v1 + v2);
    println!("v1 * 2 = {}", v1 * 2.0);
    println!("|v2| = {:.2}", v2.magnitude());
    println!("v1 · v2 = {:.2}", v1.dot(&v2));

    // From/Into conversion
    let v3: Vector2D = (5.0, 6.0).into();
    let v4 = Vector2D::from((7.0, 8.0));
    println!("v3 = {v3}, v4 = {v4}");
}
```

### Mini Project 7: Generic Data Pipeline

```rust
use std::fmt::Display;

trait Transform<T> {
    fn transform(&self, input: T) -> T;
}

trait Filter<T> {
    fn keep(&self, input: &T) -> bool;
}

struct Pipeline<T> {
    data: Vec<T>,
}

impl<T: Clone + Display> Pipeline<T> {
    fn new(data: Vec<T>) -> Self {
        Self { data }
    }

    fn apply_transform<F: Transform<T>>(self, transformer: F) -> Self {
        Self {
            data: self.data.into_iter().map(|x| transformer.transform(x)).collect(),
        }
    }

    fn apply_filter<F: Filter<T>>(self, filter: F) -> Self {
        Self {
            data: self.data.into_iter().filter(|x| filter.keep(x)).collect(),
        }
    }

    fn collect(self) -> Vec<T> {
        self.data
    }

    fn print(&self) {
        for item in &self.data {
            print!("{item} ");
        }
        println!();
    }
}

struct Doubler;
impl Transform<i32> for Doubler {
    fn transform(&self, input: i32) -> i32 { input * 2 }
}

struct EvenFilter;
impl Filter<i32> for EvenFilter {
    fn keep(&self, input: &i32) -> bool { input % 2 == 0 }
}

struct AddN(i32);
impl Transform<i32> for AddN {
    fn transform(&self, input: i32) -> i32 { input + self.0 }
}

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result = Pipeline::new(data)
        .apply_filter(EvenFilter)
        .apply_transform(Doubler)
        .apply_transform(AddN(1))
        .collect();

    println!("{:?}", result); // [5, 9, 13, 17, 21]
}
```

---

## Section 8: Closures & Functional Patterns

### What Are Closures?

A closure is an anonymous function that can *capture* variables from its surrounding environment. Closures are a fundamental building block of functional programming, and Rust's closures are particularly powerful because they interact with the ownership system in interesting ways.

In Rust, closures are not just syntactic sugar for functions — they are distinct types that implement one or more of three traits: `Fn`, `FnMut`, and `FnOnce`. Understanding these traits is key to using closures effectively.

```rust
fn main() {
    let x = 5;

    // A closure that captures x by reference
    let add_x = |n| n + x;
    println!("{}", add_x(3)); // 8

    // Closures can infer their types from context
    let multiply = |a, b| a * b;
    println!("{}", multiply(3, 4)); // 12

    // Explicit type annotations (rarely needed)
    let square = |n: i32| -> i32 { n * n };

    // Closures that capture by move (using `move` keyword)
    let name = String::from("Alice");
    let greet = move || println!("Hello, {name}!"); // name is moved into the closure
    greet();
    // println!("{name}"); // ERROR: name was moved into the closure

    // Closures as function arguments
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum();
    println!("Sum of evens: {sum}");
}
```

### The Fn, FnMut, and FnOnce Traits

Every closure in Rust implements at least one of these three traits, depending on how it captures its environment:

- **`FnOnce`**: The closure can be called once. It takes ownership of captured variables. All closures implement this.
- **`FnMut`**: The closure can be called multiple times and may mutate captured variables. Closures that capture by mutable reference implement this.
- **`Fn`**: The closure can be called multiple times without mutating captured variables. Closures that capture by shared reference implement this.

```rust
fn apply_once<F: FnOnce() -> String>(f: F) -> String {
    f() // can only call f once
}

fn apply_mut<F: FnMut() -> i32>(mut f: F, times: usize) -> Vec<i32> {
    (0..times).map(|_| f()).collect()
}

fn apply<F: Fn(i32) -> i32>(f: F, values: &[i32]) -> Vec<i32> {
    values.iter().map(|&x| f(x)).collect()
}

fn main() {
    // FnOnce: consumes a captured value
    let name = String::from("World");
    let greeting = apply_once(move || format!("Hello, {name}!"));
    println!("{greeting}");

    // FnMut: mutates a captured value
    let mut count = 0;
    let counter = || {
        count += 1;
        count
    };
    let counts = apply_mut(counter, 5);
    println!("{:?}", counts); // [1, 2, 3, 4, 5]

    // Fn: reads a captured value
    let multiplier = 3;
    let triple = |x| x * multiplier;
    let tripled = apply(triple, &[1, 2, 3, 4, 5]);
    println!("{:?}", tripled); // [3, 6, 9, 12, 15]
}
```

### Returning Closures from Functions

Returning closures from functions requires some care because closures have anonymous types. You can use `impl Fn` for static dispatch or `Box<dyn Fn>` for dynamic dispatch.

```rust
// Return a closure using `impl Fn` (static dispatch, preferred)
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |n| n + x
}

// Return a closure using Box<dyn Fn> (dynamic dispatch, needed for trait objects)
fn make_operation(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
    match op {
        "add" => Box::new(|a, b| a + b),
        "sub" => Box::new(|a, b| a - b),
        "mul" => Box::new(|a, b| a * b),
        _ => Box::new(|a, _| a),
    }
}

// Higher-order functions: functions that take or return functions
fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn main() {
    let add5 = make_adder(5);
    let add10 = make_adder(10);
    println!("{}", add5(3));  // 8
    println!("{}", add10(3)); // 13

    let op = make_operation("mul");
    println!("{}", op(3, 4)); // 12

    // Function composition
    let double = |x: i32| x * 2;
    let add_one = |x: i32| x + 1;
    let double_then_add = compose(double, add_one);
    println!("{}", double_then_add(5)); // 11

    // Memoization using closures
    let mut cache = std::collections::HashMap::new();
    let mut memoized_square = |n: i32| -> i32 {
        *cache.entry(n).or_insert_with(|| {
            println!("Computing {}^2", n);
            n * n
        })
    };

    println!("{}", memoized_square(5)); // computes
    println!("{}", memoized_square(5)); // cached
    println!("{}", memoized_square(6)); // computes
}
```

### Mini Project 8: Functional Data Processing Pipeline

```rust
type Transform = Box<dyn Fn(f64) -> f64>;

struct DataPipeline {
    transforms: Vec<Transform>,
}

impl DataPipeline {
    fn new() -> Self {
        Self { transforms: vec![] }
    }

    fn add_transform(mut self, f: impl Fn(f64) -> f64 + 'static) -> Self {
        self.transforms.push(Box::new(f));
        self
    }

    fn scale(self, factor: f64) -> Self {
        self.add_transform(move |x| x * factor)
    }

    fn offset(self, amount: f64) -> Self {
        self.add_transform(move |x| x + amount)
    }

    fn clamp(self, min: f64, max: f64) -> Self {
        self.add_transform(move |x| x.max(min).min(max))
    }

    fn apply(&self, data: &[f64]) -> Vec<f64> {
        data.iter().map(|&x| {
            self.transforms.iter().fold(x, |acc, f| f(acc))
        }).collect()
    }

    fn statistics(data: &[f64]) -> (f64, f64, f64) {
        let n = data.len() as f64;
        let mean = data.iter().sum::<f64>() / n;
        let variance = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / n;
        let std_dev = variance.sqrt();
        let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        (mean, std_dev, max - min)
    }
}

fn main() {
    let raw_data = vec![-5.0, 0.0, 3.5, 7.2, 10.0, 15.3, -2.1, 8.8];

    let pipeline = DataPipeline::new()
        .offset(5.0)      // shift all values up by 5
        .scale(2.0)       // double all values
        .clamp(0.0, 30.0); // clamp to [0, 30]

    let processed = pipeline.apply(&raw_data);

    println!("Raw:       {:?}", raw_data);
    println!("Processed: {:?}", processed);

    let (mean, std, range) = DataPipeline::statistics(&processed);
    println!("Mean: {mean:.2}, Std: {std:.2}, Range: {range:.2}");
}
```

---

## Section 9: Working with Files

### File I/O in Rust: The Philosophy

File I/O in Rust follows the same principles as the rest of the language: explicit error handling, clear ownership semantics, and zero-cost abstractions. Every file operation that can fail returns a `Result`, forcing you to handle errors explicitly. This might feel verbose at first, but it prevents the silent failures that plague file handling in other languages.

Rust's file I/O is built on the `std::fs` module for synchronous operations and `tokio::fs` (or `async-std::fs`) for asynchronous operations. Understanding when to use each is important: synchronous I/O is simpler and appropriate for command-line tools and scripts; asynchronous I/O is essential for servers and applications that need to handle many concurrent I/O operations.

```rust
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Write, Read};
use std::path::Path;

fn basic_file_operations() -> io::Result<()> {
    // Write a file (creates or truncates)
    fs::write("hello.txt", "Hello, World!\n")?;

    // Read entire file into a String
    let content = fs::read_to_string("hello.txt")?;
    println!("Content: {content}");

    // Read into bytes
    let bytes = fs::read("hello.txt")?;
    println!("Bytes: {:?}", &bytes[..5]);

    // Append to a file
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("hello.txt")?;
    writeln!(file, "Another line")?;

    // Check if file exists
    if Path::new("hello.txt").exists() {
        println!("File exists");
    }

    // Get file metadata
    let metadata = fs::metadata("hello.txt")?;
    println!("File size: {} bytes", metadata.len());
    println!("Is file: {}", metadata.is_file());

    // Copy, rename, delete
    fs::copy("hello.txt", "hello_copy.txt")?;
    fs::rename("hello_copy.txt", "hello_renamed.txt")?;
    fs::remove_file("hello_renamed.txt")?;
    fs::remove_file("hello.txt")?;

    Ok(())
}
```

### Buffered I/O: Why It Matters

Raw file I/O makes a system call for every read or write operation. System calls are expensive — they require switching from user space to kernel space. For small, frequent reads and writes, this overhead dominates.

Buffered I/O solves this by accumulating data in memory and making fewer, larger system calls. `BufReader` and `BufWriter` wrap any `Read` or `Write` implementor and add buffering.

```rust
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn process_large_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let input = File::open(input_path)?;
    let output = File::create(output_path)?;

    // BufReader reads in chunks (default 8KB buffer)
    // This is MUCH faster than reading line by line without buffering
    let reader = BufReader::new(input);

    // BufWriter accumulates writes and flushes in chunks
    let mut writer = BufWriter::new(output);

    let mut line_count = 0;
    let mut word_count = 0;

    for line in reader.lines() {
        let line = line?; // each line() call returns Result<String>
        word_count += line.split_whitespace().count();
        line_count += 1;

        // Write processed line to output
        writeln!(writer, "{:4}: {}", line_count, line.to_uppercase())?;
    }

    // BufWriter flushes automatically when dropped, but explicit flush
    // ensures all data is written before we return
    writer.flush()?;

    println!("Processed {line_count} lines, {word_count} words");
    Ok(())
}
```

### Working with Paths

Rust has two path types: `Path` (borrowed, like `&str`) and `PathBuf` (owned, like `String`). They provide a cross-platform API for working with file system paths.

```rust
use std::path::{Path, PathBuf};
use std::fs;

fn explore_directory(dir: &Path) -> io::Result<()> {
    println!("Exploring: {}", dir.display());

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            println!("  DIR:  {}", path.display());
            // Recursively explore (careful with deep trees!)
            // explore_directory(&path)?;
        } else {
            let size = metadata.len();
            let ext = path.extension()
                .and_then(|e| e.to_str())
                .unwrap_or("none");
            println!("  FILE: {} ({} bytes, .{})", 
                path.file_name().unwrap().to_string_lossy(), size, ext);
        }
    }
    Ok(())
}

fn path_operations() {
    let mut path = PathBuf::from("/home/user");
    path.push("documents");
    path.push("report.txt");

    println!("Full path: {}", path.display());
    println!("Parent: {:?}", path.parent());
    println!("File name: {:?}", path.file_name());
    println!("Extension: {:?}", path.extension());
    println!("Stem: {:?}", path.file_stem());

    // Path joining
    let base = Path::new("/home/user");
    let full = base.join("documents").join("file.txt");
    println!("Joined: {}", full.display());

    // Check components
    for component in path.components() {
        println!("Component: {:?}", component);
    }
}
```

### Atomic File Writing

One of the most important patterns in file I/O is *atomic writing*. If your program crashes while writing a file, you can end up with a partially written, corrupted file. The solution is to write to a temporary file first, then rename it to the target path. On most operating systems, rename is an atomic operation — it either succeeds completely or fails completely.

```rust
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

fn atomic_write(path: &Path, content: &[u8]) -> io::Result<()> {
    // Create a temp file in the same directory as the target
    // (important: rename across filesystems is not atomic)
    let dir = path.parent().unwrap_or(Path::new("."));
    let temp_path = dir.join(format!(".tmp_{}", 
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));

    // Write to temp file
    {
        let file = File::create(&temp_path)?;
        let mut writer = BufWriter::new(&file);
        writer.write_all(content)?;
        writer.flush()?;

        // fsync: ensure data is written to disk, not just OS buffer
        // This is critical for durability guarantees
        file.sync_all()?;
    }

    // Atomically rename temp file to target
    fs::rename(&temp_path, path)?;

    Ok(())
}

fn main() -> io::Result<()> {
    let config = r#"
    {
        "host": "localhost",
        "port": 8080,
        "debug": true
    }
    "#;

    atomic_write(Path::new("config.json"), config.as_bytes())?;
    println!("Config written atomically");

    // Clean up
    fs::remove_file("config.json")?;
    Ok(())
}
```

### Async File I/O with Tokio

For applications that handle many concurrent operations, synchronous file I/O blocks the thread while waiting for the OS. Async file I/O allows the runtime to do other work while waiting for I/O to complete.

```rust
use tokio::fs::{self, File};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Async write
    fs::write("async_test.txt", "Hello from async!\n").await?;

    // Async read
    let content = fs::read_to_string("async_test.txt").await?;
    println!("Read: {content}");

    // Async buffered reading
    let file = File::open("async_test.txt").await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        println!("Line: {line}");
    }

    // Async atomic write
    let temp = "async_test.tmp";
    let target = "async_output.txt";

    let mut file = File::create(temp).await?;
    file.write_all(b"Atomically written content").await?;
    file.flush().await?;
    file.sync_all().await?;
    drop(file);

    fs::rename(temp, target).await?;

    // Cleanup
    fs::remove_file(target).await?;
    fs::remove_file("async_test.txt").await?;

    Ok(())
}
```

### Mini Project 9: Atomic Config Manager

```rust
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write, BufWriter};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct Config {
    values: HashMap<String, String>,
    path: PathBuf,
}

impl Config {
    fn load(path: &Path) -> io::Result<Self> {
        let mut values = HashMap::new();

        if path.exists() {
            let content = fs::read_to_string(path)?;
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some((key, value)) = line.split_once('=') {
                    values.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
        }

        Ok(Self { values, path: path.to_path_buf() })
    }

    fn set(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), value.to_string());
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }

    fn save(&self) -> io::Result<()> {
        let dir = self.path.parent().unwrap_or(Path::new("."));
        let temp_path = dir.join(format!(".config_tmp_{}", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos()
        ));

        {
            let file = fs::File::create(&temp_path)?;
            let mut writer = BufWriter::new(&file);

            writeln!(writer, "# Config file — auto-generated")?;
            writeln!(writer, "# Last saved: {:?}", std::time::SystemTime::now())?;
            writeln!(writer)?;

            let mut keys: Vec<_> = self.values.keys().collect();
            keys.sort();

            for key in keys {
                writeln!(writer, "{} = {}", key, self.values[key])?;
            }

            writer.flush()?;
            file.sync_all()?;
        }

        fs::rename(&temp_path, &self.path)?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let path = Path::new("app.conf");
    let mut config = Config::load(path)?;

    config.set("host", "localhost");
    config.set("port", "8080");
    config.set("debug", "true");
    config.set("log_level", "info");

    config.save()?;
    println!("Config saved to {}", path.display());

    // Reload and verify
    let loaded = Config::load(path)?;
    println!("host = {:?}", loaded.get("host"));
    println!("port = {:?}", loaded.get("port"));

    fs::remove_file(path)?;
    Ok(())
}
```

---


## Section 10: Command Line Arguments (CLI)

### The Philosophy of CLI Design
A professional command-line tool is more than just a script; it is a user interface. In the world of systems programming, the CLI is the primary way automation, cloud infrastructure, and other developers interact with your software. 

A well-designed CLI follows the "Rule of Least Surprise." This means using standard conventions for flags (`--verbose`), options (`--output <file>`), and subcommands (`git push`). In Rust, we move from the low-level `std::env::args` to the high-level `clap` (Command Line Argument Parser) to handle these conventions automatically.

### Positional vs. Keyword Arguments
Understanding the difference between these two is fundamental to CLI design:

1.  **Positional Arguments**: These are arguments identified by their position in the command. For example, in `cp file1.txt file2.txt`, both are positional. They are usually mandatory and represent the "target" of the action.
2.  **Keyword Arguments (Options/Flags)**: These are identified by a name (e.g., `--limit 50` or `-v`). They can usually appear in any order and are often optional, providing configuration for "how" the program should run.

### Mixed Arguments: Positionals with Keywords
Often, a command might have a positional argument that *takes* its own keyword arguments. In modern CLI design, this is usually handled via **Subcommands**. For example, in `docker run --name my-container ubuntu:latest`, `run` is a subcommand, `--name` is a keyword argument belonging to that subcommand, and `ubuntu:latest` is a positional argument.

### The Modern Way: Clap (Command Line Argument Parser)
In Rust, `clap` is the undisputed king of CLI parsing. It uses Rust's powerful derive macro system to turn a simple `struct` into a robust parser.

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "DataTool", version = "1.0", about = "Processes data files")]
struct Cli {
    /// The positional input file (Mandatory)
    input: String,

    /// An optional keyword argument for the output path
    #[arg(short, long, value_name = "FILE")]
    output: Option<String>,

    /// A flag argument that turns on verbose mode
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Perform a specific check on the data
    Check {
        /// A keyword argument specific to this subcommand
        #[arg(long)]
        strict: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Verbosity enabled. Processing {}", cli.input);
    }

    // Logic to handle keywords and positionals...
}
```

---

## Section 11: Terminal User Interfaces (TUI)

### Moving Beyond Scrolling Text
While a standard CLI prints text line-by-line, a **Terminal User Interface (TUI)** uses the terminal as a visual grid. Think of tools like `htop`, `vim`, or `lazygit`. These applications allow for layouts, colors, responsive design, and keyboard interactions.

In Rust, the primary ecosystem for this is **Ratatui** (built on top of the **Crossterm** backend). Ratatui follows an "Immediate Mode" rendering pattern: every time the screen needs to update, you describe the *entire* UI from scratch based on your current application state.

### The Virtual Grid and Layouts
To build a TUI, you divide your terminal window into rectangular areas. Ratatui uses a `Layout` system where you define constraints (e.g., "Give the top section 3 lines, and let the bottom section take the rest").

- **Constraints**: You can use `Percentage(x)`, `Length(y)`, `Min(z)`, and `Max(w)`.
- **Rects**: These represent the physical boundaries of your widgets.

### Colors and Styles
Colors in the terminal have evolved from the basic 8 colors to a full 24-bit "True Color" gamut. Modern crates allow you to apply styles like `Bold`, `Italic`, `Underline`, and custom HEX colors to any piece of text.

### The Event Loop
A TUI application doesn't exit after printing. It runs in a loop, waiting for user input:
1.  **Poll for events**: Keyboard press? Mouse click? Window resize?
2.  **Update State**: If 'Up Arrow' was pressed, move the selected index in a list.
3.  **Draw**: Render the widgets based on the new state.

### Mini-Project 11: "SysWatch" Performance Dashboard
**Goal**: Build a simple TUI dashboard that shows a simulated system status.

**Key Requirements**:
- Use a `Block` with a `Border` and a `Title`.
- Implement a `Gauge` widget to show a "CPU Load" percentage.
- Change the gauge color based on the value (Green < 50%, Yellow < 80%, Red > 80%).
- Ensure the program exits gracefully when the user presses 'q'.

---
## Section 12: Modules, Crates & Workspaces

### The Module System: Organizing Code

As your Rust programs grow, you need a way to organize code into logical units. Rust's module system provides this through *modules*, *crates*, and *workspaces*.

A **module** is a namespace that groups related code together. Modules control the *visibility* of items — by default, everything in Rust is private. You use the `pub` keyword to make items public.

A **crate** is the unit of compilation in Rust. It is either a binary (an executable) or a library. Every Rust project is a crate.

A **workspace** is a collection of related crates that share a `Cargo.lock` file and output directory.

```rust
// src/lib.rs — the root of a library crate

// Declare a module (can be inline or in a separate file)
pub mod geometry {
    // Items are private by default
    struct InternalHelper;

    // pub makes items visible outside the module
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    impl Point {
        pub fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }

        pub fn distance_to(&self, other: &Point) -> f64 {
            ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
        }
    }

    // Nested module
    pub mod shapes {
        use super::Point; // `super` refers to the parent module

        pub struct Circle {
            pub center: Point,
            pub radius: f64,
        }

        impl Circle {
            pub fn new(x: f64, y: f64, radius: f64) -> Self {
                Self { center: Point::new(x, y), radius }
            }

            pub fn area(&self) -> f64 {
                std::f64::consts::PI * self.radius * self.radius
            }
        }
    }
}

// Using the module
use geometry::Point;
use geometry::shapes::Circle;

pub fn example() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    println!("Distance: {}", p1.distance_to(&p2));

    let c = Circle::new(0.0, 0.0, 5.0);
    println!("Area: {:.2}", c.area());
}
```

### Splitting Modules Across Files

For larger projects, you will split modules across multiple files. Rust's module system maps directly to the file system.

```
src/
├── main.rs          (or lib.rs for libraries)
├── geometry/
│   ├── mod.rs       (declares the geometry module)
│   ├── point.rs     (the Point type)
│   └── shapes.rs    (shape types)
└── utils.rs         (utility functions)
```

```rust
// src/geometry/mod.rs
pub mod point;
pub mod shapes;

pub use point::Point; // re-export for convenience

// src/geometry/point.rs
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

// src/main.rs
mod geometry;
mod utils;

use geometry::Point;

fn main() {
    let p = Point::new(1.0, 2.0);
    println!("{:?}", p);
}
```

### Cargo Workspaces

A workspace is a collection of related crates that are developed together. This is the standard way to organize large Rust projects with multiple components.

```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "core",
    "cli",
    "server",
    "common",
]

# All crates share the same Cargo.lock and target directory
# This means they all use the same versions of dependencies
```

```
my_project/
├── Cargo.toml          (workspace manifest)
├── Cargo.lock          (shared lock file)
├── target/             (shared build output)
├── common/             (shared library)
│   ├── Cargo.toml
│   └── src/lib.rs
├── core/               (core business logic)
│   ├── Cargo.toml
│   └── src/lib.rs
├── cli/                (command-line interface)
│   ├── Cargo.toml
│   └── src/main.rs
└── server/             (HTTP server)
    ├── Cargo.toml
    └── src/main.rs
```

```toml
# core/Cargo.toml
[package]
name = "core"
version = "0.1.0"
edition = "2021"

[dependencies]
common = { path = "../common" }
serde = { version = "1.0", features = ["derive"] }
```

### Visibility and pub(crate)

Rust has fine-grained visibility control:

```rust
pub struct MyStruct {
    pub public_field: i32,          // visible everywhere
    pub(crate) crate_field: i32,    // visible within the crate
    pub(super) super_field: i32,    // visible in parent module
    private_field: i32,             // visible only in this module
}

// pub(crate) is very useful for internal APIs that should not be
// exposed to users of your library but need to be shared across modules
pub(crate) fn internal_helper() {
    // ...
}
```

### Mini Project 10: Multi-Module Library

Build a small math library with proper module organization.

```
math_lib/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── statistics.rs
    ├── linear_algebra.rs
    └── number_theory.rs
```

```rust
// src/lib.rs
pub mod statistics;
pub mod linear_algebra;
pub mod number_theory;

// Re-export commonly used items
pub use statistics::{mean, median, std_dev};
pub use linear_algebra::Matrix;

// src/statistics.rs
pub fn mean(data: &[f64]) -> Option<f64> {
    if data.is_empty() { return None; }
    Some(data.iter().sum::<f64>() / data.len() as f64)
}

pub fn median(data: &mut Vec<f64>) -> Option<f64> {
    if data.is_empty() { return None; }
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = data.len() / 2;
    if data.len() % 2 == 0 {
        Some((data[mid - 1] + data[mid]) / 2.0)
    } else {
        Some(data[mid])
    }
}

pub fn std_dev(data: &[f64]) -> Option<f64> {
    let m = mean(data)?;
    let variance = data.iter().map(|&x| (x - m).powi(2)).sum::<f64>() / data.len() as f64;
    Some(variance.sqrt())
}
```

---

## Section 13: Testing, Linting & Formatting

### The Philosophy of Testing in Rust

Testing is a first-class citizen in Rust. The language and toolchain are designed with testing in mind from the ground up. Unlike many languages where testing requires external frameworks, Rust's testing infrastructure is built into the compiler and Cargo.

Rust's testing philosophy aligns with its overall philosophy: make correctness verifiable. The type system and borrow checker catch many bugs at compile time, but tests catch the logical errors that types cannot express. Together, they form a powerful safety net.

There are four main kinds of tests in Rust:
1. **Unit tests**: Test individual functions and modules in isolation
2. **Integration tests**: Test the public API of your crate as a whole
3. **Documentation tests**: Test the code examples in your documentation
4. **Benchmark tests**: Measure the performance of your code

### Unit Tests

Unit tests in Rust live in the same file as the code they test, in a special `#[cfg(test)]` module. The `#[cfg(test)]` attribute tells the compiler to only compile this module when running tests, so it does not affect the production binary.

```rust
// src/lib.rs

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let sqrt = (n as f64).sqrt() as u64;
    (3..=sqrt).step_by(2).all(|i| n % i != 0)
}

// The test module is only compiled when running `cargo test`
#[cfg(test)]
mod tests {
    // `use super::*` brings all items from the parent module into scope
    // This is the standard pattern for unit tests
    use super::*;

    // Each test function is annotated with #[test]
    #[test]
    fn test_add_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add(-2, -3), -5);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 5), 5);
        assert_eq!(add(5, 0), 5);
    }

    #[test]
    fn test_divide_normal() {
        assert_eq!(divide(10.0, 2.0), Some(5.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10.0, 0.0), None);
    }

    // Test that a function panics
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_out_of_bounds() {
        let v = vec![1, 2, 3];
        let _ = v[10]; // should panic
    }

    // Skip a test (useful for known failures or slow tests)
    #[test]
    #[ignore = "too slow for regular CI"]
    fn test_large_prime() {
        assert!(is_prime(999_999_999_989));
    }

    // Test with multiple assertions
    #[test]
    fn test_is_prime() {
        // assert_eq! shows both values on failure
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(17), true);
        assert_eq!(is_prime(100), false);

        // assert! for boolean conditions
        assert!(is_prime(97), "97 should be prime");
        assert!(!is_prime(99), "99 should not be prime");

        // Custom failure message
        for &p in &[2, 3, 5, 7, 11, 13, 17, 19, 23] {
            assert!(is_prime(p), "{p} should be prime");
        }
    }
}
```

### Integration Tests

Integration tests live in a `tests/` directory at the root of your crate. They test your crate's public API from the outside, just like a user of your library would.

```rust
// tests/integration_test.rs
// Note: no `mod tests` needed — each file in tests/ is its own crate

use my_lib::{add, divide, is_prime};

#[test]
fn test_add_works_from_outside() {
    assert_eq!(add(1, 1), 2);
}

#[test]
fn test_prime_sequence() {
    let primes: Vec<u64> = (2..30).filter(|&n| is_prime(n)).collect();
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
}

// tests/common/mod.rs — shared test utilities
// (create this file to share helpers across integration test files)
pub fn setup() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}
```

### Documentation Tests

One of Rust's most unique features is *documentation tests* — code examples in your documentation comments are automatically compiled and run as tests. This ensures your documentation is always accurate and up to date.

```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// use my_lib::add;
///
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// Negative numbers work too:
///
/// ```
/// use my_lib::add;
///
/// assert_eq!(add(-1, -1), -2);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Divides two numbers, returning None if the divisor is zero.
///
/// # Examples
///
/// ```
/// use my_lib::divide;
///
/// assert_eq!(divide(10.0, 2.0), Some(5.0));
/// assert_eq!(divide(10.0, 0.0), None);
/// ```
///
/// # Panics
///
/// This function does not panic.
pub fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}
```

### Mocking with mockall

The `mockall` crate provides powerful mocking capabilities for Rust. Mocking allows you to test code in isolation by replacing real dependencies with controlled fakes.

```rust
use mockall::{automock, predicate::*};

#[automock]
trait Database {
    fn get_user(&self, id: u64) -> Option<String>;
    fn save_user(&mut self, id: u64, name: &str) -> bool;
}

struct UserService {
    db: Box<dyn Database>,
}

impl UserService {
    fn new(db: Box<dyn Database>) -> Self {
        Self { db }
    }

    fn get_username(&self, id: u64) -> String {
        self.db.get_user(id).unwrap_or_else(|| "Unknown".to_string())
    }

    fn create_user(&mut self, id: u64, name: &str) -> Result<(), String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if self.db.save_user(id, name) {
            Ok(())
        } else {
            Err("Failed to save user".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_existing_user() {
        let mut mock_db = MockDatabase::new();

        // Set up expectation: get_user(1) will be called once and return "Alice"
        mock_db.expect_get_user()
            .with(eq(1u64))
            .times(1)
            .returning(|_| Some("Alice".to_string()));

        let service = UserService::new(Box::new(mock_db));
        assert_eq!(service.get_username(1), "Alice");
    }

    #[test]
    fn test_get_nonexistent_user() {
        let mut mock_db = MockDatabase::new();

        mock_db.expect_get_user()
            .returning(|_| None);

        let service = UserService::new(Box::new(mock_db));
        assert_eq!(service.get_username(999), "Unknown");
    }

    #[test]
    fn test_create_user_empty_name() {
        let mock_db = MockDatabase::new();
        let mut service = UserService::new(Box::new(mock_db));

        let result = service.create_user(1, "");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Name cannot be empty");
    }
}
```

### Property-Based Testing with proptest

Property-based testing generates random inputs and checks that certain properties hold for all of them. This is much more thorough than hand-written test cases.

```rust
use proptest::prelude::*;

fn reverse<T: Clone>(v: &[T]) -> Vec<T> {
    v.iter().rev().cloned().collect()
}

fn is_sorted(v: &[i32]) -> bool {
    v.windows(2).all(|w| w[0] <= w[1])
}

proptest! {
    // This test runs with hundreds of random inputs automatically
    #[test]
    fn test_reverse_twice_is_identity(v in prop::collection::vec(any::<i32>(), 0..100)) {
        let reversed_twice = reverse(&reverse(&v));
        prop_assert_eq!(v, reversed_twice);
    }

    #[test]
    fn test_reverse_length_preserved(v in prop::collection::vec(any::<i32>(), 0..100)) {
        prop_assert_eq!(v.len(), reverse(&v).len());
    }

    #[test]
    fn test_sort_is_idempotent(mut v in prop::collection::vec(any::<i32>(), 0..100)) {
        v.sort();
        let sorted_once = v.clone();
        v.sort();
        prop_assert_eq!(sorted_once, v);
    }

    #[test]
    fn test_add_is_commutative(a in any::<i32>(), b in any::<i32>()) {
        // Use wrapping_add to avoid overflow panics
        prop_assert_eq!(a.wrapping_add(b), b.wrapping_add(a));
    }
}
```

### Benchmarking with criterion

`criterion` is the standard benchmarking library for Rust. It provides statistically rigorous benchmarks with warm-up, multiple samples, and outlier detection.

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn fibonacci_iterative(n: u64) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

fn bench_fibonacci(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");

    for n in [10u64, 15, 20].iter() {
        group.bench_with_input(
            BenchmarkId::new("recursive", n),
            n,
            |b, &n| b.iter(|| fibonacci_recursive(black_box(n)))
        );

        group.bench_with_input(
            BenchmarkId::new("iterative", n),
            n,
            |b, &n| b.iter(|| fibonacci_iterative(black_box(n)))
        );
    }

    group.finish();
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);
```

### Clippy: The Rust Linter

Clippy is Rust's official linter. It provides over 700 lints that catch common mistakes, suggest more idiomatic code, and identify potential performance issues. Running Clippy should be part of every Rust developer's workflow.

```bash
# Run clippy
cargo clippy

# Run clippy with all lints enabled (very strict)
cargo clippy -- -W clippy::all -W clippy::pedantic

# Run clippy and treat warnings as errors (for CI)
cargo clippy -- -D warnings

# Fix automatically fixable issues
cargo clippy --fix
```

Clippy catches things like:

```rust
// Clippy warns: use `is_empty()` instead of `len() == 0`
if v.len() == 0 { }  // clippy::len_zero
if v.is_empty() { }  // idiomatic

// Clippy warns: use `..=` for inclusive ranges
if x >= 0 && x <= 100 { }  // clippy::manual_range_contains
if (0..=100).contains(&x) { }  // idiomatic

// Clippy warns: use `unwrap_or_default()` instead of `unwrap_or(Default::default())`
let x = opt.unwrap_or(String::new());  // verbose
let x = opt.unwrap_or_default();       // idiomatic

// Clippy warns: needless collect
let v: Vec<_> = iter.collect();
v.iter().for_each(|x| ...);
// Better: just use the iterator directly
iter.for_each(|x| ...);
```

You can configure Clippy in `Cargo.toml` or with a `.clippy.toml` file:

```toml
# Cargo.toml
[lints.clippy]
pedantic = "warn"
nursery = "warn"
# Disable specific lints
module_name_repetitions = "allow"
```

### rustfmt: Automatic Code Formatting

`rustfmt` is Rust's official code formatter. It enforces a consistent style across all Rust code, eliminating debates about formatting and making code reviews focus on logic rather than style.

```bash
# Format all files in the project
cargo fmt

# Check formatting without modifying files (for CI)
cargo fmt -- --check

# Format a specific file
rustfmt src/main.rs
```

You can configure `rustfmt` with a `rustfmt.toml` file:

```toml
# rustfmt.toml
edition = "2021"
max_width = 100
tab_spaces = 4
use_small_heuristics = "Default"
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
```

You can suppress formatting for specific blocks:

```rust
#[rustfmt::skip]
let matrix = [
    1, 0, 0,
    0, 1, 0,
    0, 0, 1,
];
```

### CI/CD Pipeline for Rust

A complete CI/CD pipeline for a Rust project should run tests, linting, and formatting checks on every pull request.

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  test:
    name: Test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --all-features

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo clippy --all-targets --all-features -- -D warnings

  fmt:
    name: Format
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo fmt --all -- --check

  coverage:
    name: Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo install cargo-tarpaulin
      - run: cargo tarpaulin --out Xml
      - uses: codecov/codecov-action@v3
```

### Mini Project 11: Fully Tested Math Library

```rust
// src/lib.rs

/// Computes the greatest common divisor using the Euclidean algorithm.
///
/// # Examples
///
/// ```
/// use math_lib::gcd;
/// assert_eq!(gcd(48, 18), 6);
/// assert_eq!(gcd(0, 5), 5);
/// ```
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// Computes the least common multiple.
///
/// # Examples
///
/// ```
/// use math_lib::lcm;
/// assert_eq!(lcm(4, 6), 12);
/// ```
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 { return 0; }
    a / gcd(a, b) * b
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_gcd_known_values() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(100, 75), 25);
        assert_eq!(gcd(7, 13), 1); // coprime
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
    }

    proptest! {
        #[test]
        fn test_gcd_divides_both(a in 1u64..1000, b in 1u64..1000) {
            let g = gcd(a, b);
            prop_assert_eq!(a % g, 0);
            prop_assert_eq!(b % g, 0);
        }

        #[test]
        fn test_lcm_is_multiple_of_both(a in 1u64..100, b in 1u64..100) {
            let l = lcm(a, b);
            prop_assert_eq!(l % a, 0);
            prop_assert_eq!(l % b, 0);
        }

        #[test]
        fn test_gcd_lcm_relationship(a in 1u64..100, b in 1u64..100) {
            // gcd(a,b) * lcm(a,b) == a * b
            prop_assert_eq!(gcd(a, b) * lcm(a, b), a * b);
        }
    }
}
```

---

## Section 14: Concurrency & Async Rust

### Why Concurrency Is Hard (And How Rust Helps)

Concurrency is one of the hardest problems in programming. When multiple threads access shared data simultaneously, you can get:

- **Data races**: Two threads read and write the same memory simultaneously, producing unpredictable results
- **Deadlocks**: Two threads each hold a lock the other needs, causing both to wait forever
- **Race conditions**: The behavior of the program depends on the timing of thread execution

Most languages deal with data races through runtime checks (Go's race detector), conventions (Python's GIL), or by simply not preventing them (C/C++). Rust prevents data races at compile time through the ownership and type systems.

The key insight is that Rust's ownership rules naturally prevent data races: you cannot have two mutable references to the same data at the same time. The `Send` and `Sync` traits extend this to threads.

### Threads and Message Passing

Rust's standard library provides OS threads through `std::thread`. The recommended way to communicate between threads is *message passing* — threads send data to each other through channels, rather than sharing memory directly.

```rust
use std::thread;
use std::sync::mpsc; // multiple producer, single consumer
use std::time::Duration;

fn main() {
    // Create a channel: tx is the sender, rx is the receiver
    let (tx, rx) = mpsc::channel();

    // Spawn a thread that sends messages
    let tx_clone = tx.clone(); // clone the sender for multiple producers

    let handle = thread::spawn(move || {
        let messages = vec!["hello", "from", "the", "thread"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Another producer
    thread::spawn(move || {
        tx_clone.send("extra message").unwrap();
    });

    // Receive messages (blocks until all senders are dropped)
    for received in rx {
        println!("Got: {received}");
    }

    handle.join().unwrap();
}
```

### Shared State with Mutex and Arc

When you need multiple threads to share and modify the same data, use `Arc<Mutex<T>>`. `Arc` provides shared ownership across threads, and `Mutex` ensures only one thread can access the data at a time.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc<Mutex<T>>: the standard pattern for shared mutable state
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // lock() acquires the mutex, returning a MutexGuard
            // The guard automatically releases the lock when dropped
            let mut num = counter.lock().unwrap();
            *num += 1;
            // Lock is released here when `num` goes out of scope
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {}", *counter.lock().unwrap()); // 10

    // RwLock: allows multiple readers OR one writer
    use std::sync::RwLock;
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    // Multiple readers can hold the lock simultaneously
    let data_clone = Arc::clone(&data);
    let reader = thread::spawn(move || {
        let read_guard = data_clone.read().unwrap();
        println!("Reading: {:?}", *read_guard);
    });

    reader.join().unwrap();

    // Only one writer at a time
    data.write().unwrap().push(4);
    println!("After write: {:?}", *data.read().unwrap());
}
```

### Async Rust: The Fundamentals

Async Rust is a way to write concurrent code that looks sequential. Instead of blocking a thread while waiting for I/O, async code *yields* control back to the runtime, which can run other tasks in the meantime.

The key concepts are:
- **`async fn`**: A function that returns a `Future` instead of a value directly
- **`await`**: Suspends the current task until the future completes
- **Runtime**: An executor that drives futures to completion (Tokio is the most popular)
- **Task**: A lightweight unit of concurrent work (much cheaper than a thread)

```rust
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc;

// An async function returns a Future
async fn fetch_data(id: u32) -> String {
    // Simulate network request
    sleep(Duration::from_millis(100)).await;
    format!("Data for id={id}")
}

async fn process_all() {
    // Sequential: each awaits the previous
    let d1 = fetch_data(1).await;
    let d2 = fetch_data(2).await;
    println!("Sequential: {d1}, {d2}");

    // Concurrent: both run at the same time
    let (d3, d4) = tokio::join!(fetch_data(3), fetch_data(4));
    println!("Concurrent: {d3}, {d4}");

    // Spawn tasks: fire and forget
    let handle = tokio::spawn(async {
        fetch_data(5).await
    });
    let d5 = handle.await.unwrap();
    println!("Spawned: {d5}");
}

#[tokio::main]
async fn main() {
    process_all().await;

    // Async channels
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
        }
    });

    while let Some(value) = rx.recv().await {
        println!("Received: {value}");
    }
}
```

### Understanding the Async Runtime

It is important to understand what happens under the hood when you use async/await. An `async fn` is syntactic sugar for a function that returns a type implementing the `Future` trait. When you `.await` a future, you are telling the runtime "I am waiting for this; please run other tasks in the meantime."

The runtime (Tokio, async-std, etc.) is a scheduler that manages a pool of threads and distributes tasks across them. When a task awaits, the runtime parks it and runs another task. When the awaited operation completes, the runtime wakes the task and resumes it.

```rust
use tokio::time::{sleep, Duration, Instant};

async fn demonstrate_concurrency() {
    let start = Instant::now();

    // This takes 300ms total (sequential)
    async fn sequential() {
        sleep(Duration::from_millis(100)).await;
        sleep(Duration::from_millis(100)).await;
        sleep(Duration::from_millis(100)).await;
    }

    // This takes ~100ms total (concurrent)
    async fn concurrent() {
        tokio::join!(
            sleep(Duration::from_millis(100)),
            sleep(Duration::from_millis(100)),
            sleep(Duration::from_millis(100)),
        );
    }

    sequential().await;
    println!("Sequential: {:?}", start.elapsed()); // ~300ms

    let start = Instant::now();
    concurrent().await;
    println!("Concurrent: {:?}", start.elapsed()); // ~100ms
}

#[tokio::main]
async fn main() {
    demonstrate_concurrency().await;
}
```

### Mini Project 12: Async Web Scraper

```rust
use tokio;
use std::collections::HashMap;

// Simulated async HTTP client (in real code, use reqwest)
async fn fetch_url(url: &str) -> Result<String, String> {
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    Ok(format!("Content of {url}"))
}

async fn scrape_urls(urls: Vec<String>) -> HashMap<String, Result<String, String>> {
    let futures: Vec<_> = urls.iter()
        .map(|url| {
            let url = url.clone();
            tokio::spawn(async move {
                let result = fetch_url(&url).await;
                (url, result)
            })
        })
        .collect();

    let mut results = HashMap::new();
    for future in futures {
        let (url, result) = future.await.unwrap();
        results.insert(url, result);
    }
    results
}

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://example.com/page1".to_string(),
        "https://example.com/page2".to_string(),
        "https://example.com/page3".to_string(),
    ];

    let start = std::time::Instant::now();
    let results = scrape_urls(urls).await;
    println!("Scraped {} URLs in {:?}", results.len(), start.elapsed());

    for (url, result) in &results {
        match result {
            Ok(content) => println!("OK: {url} — {}", &content[..20]),
            Err(e) => println!("ERR: {url} — {e}"),
        }
    }
}
```

---

## Section 15: Macros

### What Are Macros and Why Do They Exist?

Macros are a form of *metaprogramming* — code that writes code. In Rust, macros are expanded at compile time, before the type checker runs. This gives them capabilities that functions do not have:

- They can take a variable number of arguments (like `println!("{} {} {}", a, b, c)`)
- They can generate code based on the structure of their input
- They can implement traits automatically (like `#[derive(Debug)]`)
- They can create domain-specific languages (DSLs)

Rust has two kinds of macros: **declarative macros** (`macro_rules!`) and **procedural macros** (derive, attribute, and function-like).

### Declarative Macros with macro_rules!

Declarative macros use pattern matching on the macro's input to generate code. They are the simpler kind of macro and are sufficient for most use cases.

```rust
// A simple macro that creates a HashMap
macro_rules! hashmap {
    // Match: key => value, key => value, ...
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

// A macro for creating vectors with repeated elements
macro_rules! vec_of_strings {
    ($($x:expr),* $(,)?) => {
        vec![$($x.to_string()),*]
    };
}

// A macro that implements a trait for multiple types
macro_rules! impl_display_for {
    ($($t:ty),*) => {
        $(
            impl std::fmt::Display for $t {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
        )*
    };
}

fn main() {
    let scores = hashmap! {
        "Alice" => 95,
        "Bob" => 82,
        "Charlie" => 91,
    };
    println!("{:?}", scores);

    let names = vec_of_strings!["Alice", "Bob", "Charlie"];
    println!("{:?}", names);
}
```

### Procedural Macros: Custom Derive

Procedural macros are more powerful than declarative macros. They operate on the Rust abstract syntax tree (AST) and can generate arbitrary code. The most common kind is a *derive macro*, which automatically implements a trait for a type.

```rust
// In a separate crate (proc-macro crate)
// Cargo.toml:
// [lib]
// proc-macro = true
//
// [dependencies]
// syn = "2"
// quote = "1"
// proc-macro2 = "1"

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let builder_name = format!("{}Builder", name);
    let builder_ident = syn::Ident::new(&builder_name, name.span());

    // Generate builder struct and impl
    let expanded = quote! {
        pub struct #builder_ident {
            // ... generated fields
        }

        impl #name {
            pub fn builder() -> #builder_ident {
                #builder_ident::default()
            }
        }
    };

    TokenStream::from(expanded)
}

// Usage (in another crate):
#[derive(Builder, Debug)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
}
```

### Mini Project 13: Custom Logging Macro

```rust
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info  => write!(f, "INFO "),
            LogLevel::Warn  => write!(f, "WARN "),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

static MIN_LEVEL: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(1);

macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        {
            let level_val = $level as u8;
            let min_val = MIN_LEVEL.load(std::sync::atomic::Ordering::Relaxed);
            if level_val >= min_val {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                eprintln!("[{}] [{}] {}:{} — {}",
                    now, $level, file!(), line!(), format!($($arg)*));
            }
        }
    };
}

macro_rules! debug { ($($arg:tt)*) => { log!(LogLevel::Debug, $($arg)*) }; }
macro_rules! info  { ($($arg:tt)*) => { log!(LogLevel::Info,  $($arg)*) }; }
macro_rules! warn  { ($($arg:tt)*) => { log!(LogLevel::Warn,  $($arg)*) }; }
macro_rules! error { ($($arg:tt)*) => { log!(LogLevel::Error, $($arg)*) }; }

fn main() {
    info!("Application starting");
    debug!("Debug mode: {}", true);
    warn!("Low memory: {} MB remaining", 128);
    error!("Connection failed: {}", "timeout");
}
```

---

## Section 16: Unsafe Rust & FFI

### When and Why to Use Unsafe

Rust's safety guarantees are powerful, but they come with constraints. Sometimes you need to do things that the compiler cannot verify as safe — interfacing with C libraries, implementing low-level data structures, or performing hardware-specific operations.

`unsafe` Rust is not "unsafe" in the sense of being dangerous by default — it is a way of telling the compiler "I have verified that this is safe, even though you cannot." The programmer takes on the responsibility of upholding the safety invariants.

The key principle is to keep `unsafe` blocks as small as possible and to document exactly what invariants you are relying on.

```rust
fn main() {
    // Raw pointers: can be null, can dangle, no borrow checking
    let x = 5;
    let r1 = &x as *const i32; // raw pointer to x
    let r2 = &x as *const i32;

    // Dereferencing raw pointers requires unsafe
    unsafe {
        println!("r1 = {}", *r1);
        println!("r2 = {}", *r2);
    }

    // Mutable raw pointer
    let mut y = 10;
    let r3 = &mut y as *mut i32;
    unsafe {
        *r3 += 1;
        println!("y = {}", *r3);
    }

    // Calling unsafe functions
    unsafe fn dangerous() {
        println!("This is an unsafe function");
    }

    unsafe {
        dangerous();
    }

    // Safe abstraction over unsafe code
    // The function is safe to call, but uses unsafe internally
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        assert!(mid <= len);

        let ptr = slice.as_mut_ptr();

        // SAFETY: We know mid <= len, so both slices are within bounds.
        // The two slices do not overlap because they cover disjoint ranges.
        unsafe {
            (
                std::slice::from_raw_parts_mut(ptr, mid),
                std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let (left, right) = split_at_mut(&mut v, 3);
    println!("left: {:?}, right: {:?}", left, right);
}
```

### Foreign Function Interface (FFI)

FFI allows Rust to call functions written in other languages (primarily C) and to expose Rust functions to other languages.

```rust
// Calling C functions from Rust
extern "C" {
    fn abs(x: i32) -> i32;
    fn sqrt(x: f64) -> f64;
}

fn main() {
    unsafe {
        println!("abs(-5) = {}", abs(-5));
        println!("sqrt(2.0) = {}", sqrt(2.0));
    }
}

// Exposing Rust functions to C
#[no_mangle] // prevent name mangling so C can find the function
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

// Working with C strings
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub fn greet_c(name: &str) -> String {
    let c_name = CString::new(name).expect("CString::new failed");

    // In a real scenario, you would pass c_name.as_ptr() to a C function
    // and receive a *const c_char back

    format!("Hello, {name}!")
}
```

### Mini Project 14: Safe Wrapper Around C Math

```rust
use std::os::raw::c_double;

extern "C" {
    fn sin(x: c_double) -> c_double;
    fn cos(x: c_double) -> c_double;
    fn pow(base: c_double, exp: c_double) -> c_double;
}

// Safe wrappers
pub fn safe_sin(x: f64) -> f64 {
    unsafe { sin(x) }
}

pub fn safe_cos(x: f64) -> f64 {
    unsafe { cos(x) }
}

pub fn safe_pow(base: f64, exp: f64) -> f64 {
    unsafe { pow(base, exp) }
}

fn main() {
    use std::f64::consts::PI;

    println!("sin(π/2) = {:.6}", safe_sin(PI / 2.0));
    println!("cos(0) = {:.6}", safe_cos(0.0));
    println!("2^10 = {:.0}", safe_pow(2.0, 10.0));
}
```

---

## Section 17: Design Patterns in Rust

### Why Design Patterns Look Different in Rust

Many classic design patterns from object-oriented languages look different in Rust, or are unnecessary, because Rust's type system and ownership model provide better alternatives. Understanding how to translate OOP patterns into idiomatic Rust is a key skill.

### The Builder Pattern

The Builder pattern is used to construct complex objects step by step. In Rust, it is especially useful for types with many optional fields.

```rust
#[derive(Debug)]
struct HttpRequest {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
    timeout_ms: u64,
    follow_redirects: bool,
}

#[derive(Default)]
struct HttpRequestBuilder {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
    timeout_ms: u64,
    follow_redirects: bool,
}

impl HttpRequestBuilder {
    fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            method: "GET".to_string(),
            timeout_ms: 30_000,
            follow_redirects: true,
            ..Default::default()
        }
    }

    fn method(mut self, method: &str) -> Self {
        self.method = method.to_string();
        self
    }

    fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    fn timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }

    fn build(self) -> Result<HttpRequest, String> {
        if self.url.is_empty() {
            return Err("URL cannot be empty".to_string());
        }
        Ok(HttpRequest {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: self.body,
            timeout_ms: self.timeout_ms,
            follow_redirects: self.follow_redirects,
        })
    }
}

fn main() {
    let request = HttpRequestBuilder::new("https://api.example.com/users")
        .method("POST")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer token123")
        .body(r#"{"name": "Alice"}"#)
        .timeout(5000)
        .build()
        .unwrap();

    println!("{:#?}", request);
}
```

### The Typestate Pattern

The Typestate pattern uses Rust's type system to encode state machine transitions. Invalid state transitions become compile-time errors rather than runtime panics.

```rust
use std::marker::PhantomData;

// State types (zero-sized, used only for type-level information)
struct Locked;
struct Unlocked;

struct Safe<State> {
    contents: String,
    _state: PhantomData<State>,
}

impl Safe<Locked> {
    fn new(contents: &str) -> Self {
        Self {
            contents: contents.to_string(),
            _state: PhantomData,
        }
    }

    fn unlock(self, password: &str) -> Result<Safe<Unlocked>, Safe<Locked>> {
        if password == "secret" {
            Ok(Safe {
                contents: self.contents,
                _state: PhantomData,
            })
        } else {
            Err(self)
        }
    }
}

impl Safe<Unlocked> {
    fn get_contents(&self) -> &str {
        &self.contents
    }

    fn lock(self) -> Safe<Locked> {
        Safe {
            contents: self.contents,
            _state: PhantomData,
        }
    }
}

fn main() {
    let safe = Safe::<Locked>::new("secret documents");

    // Cannot access contents while locked — compile error!
    // safe.get_contents(); // ERROR: method not found in `Safe<Locked>`

    match safe.unlock("wrong") {
        Ok(_) => println!("Opened!"),
        Err(locked_safe) => {
            println!("Wrong password");
            if let Ok(unlocked) = locked_safe.unlock("secret") {
                println!("Contents: {}", unlocked.get_contents());
                let _locked_again = unlocked.lock();
            }
        }
    }
}
```

### The Newtype Pattern

The Newtype pattern wraps a type in a struct to give it a distinct type identity. This prevents mixing up values of the same underlying type that have different semantic meanings.

```rust
// Without newtype: easy to mix up
fn calculate_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

// With newtype: type-safe
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Meters(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Kilograms(f64);

#[derive(Debug, Clone, Copy)]
struct Seconds(f64);

impl Meters {
    fn value(self) -> f64 { self.0 }
}

impl std::ops::Add for Meters {
    type Output = Meters;
    fn add(self, other: Meters) -> Meters {
        Meters(self.0 + other.0)
    }
}

fn speed(distance: Meters, time: Seconds) -> f64 {
    distance.0 / time.0 // meters per second
}

fn main() {
    let d = Meters(100.0);
    let t = Seconds(9.58);
    println!("Speed: {:.2} m/s", speed(d, t));

    // This would be a compile error — cannot pass Kilograms where Meters expected
    // speed(Kilograms(70.0), t); // ERROR
}
```

### Mini Project 15: State Machine for a Traffic Light

```rust
use std::marker::PhantomData;
use std::time::Duration;

struct Red;
struct Yellow;
struct Green;

struct TrafficLight<State> {
    _state: PhantomData<State>,
}

impl TrafficLight<Red> {
    fn new() -> Self { Self { _state: PhantomData } }
    fn duration(&self) -> Duration { Duration::from_secs(30) }
    fn next(self) -> TrafficLight<Green> { TrafficLight { _state: PhantomData } }
}

impl TrafficLight<Green> {
    fn duration(&self) -> Duration { Duration::from_secs(25) }
    fn next(self) -> TrafficLight<Yellow> { TrafficLight { _state: PhantomData } }
}

impl TrafficLight<Yellow> {
    fn duration(&self) -> Duration { Duration::from_secs(5) }
    fn next(self) -> TrafficLight<Red> { TrafficLight { _state: PhantomData } }
}

fn main() {
    let light = TrafficLight::<Red>::new();
    println!("Red for {:?}", light.duration());

    let light = light.next();
    println!("Green for {:?}", light.duration());

    let light = light.next();
    println!("Yellow for {:?}", light.duration());

    let light = light.next();
    println!("Red again for {:?}", light.duration());

    // Cannot go from Red directly to Yellow — compile error!
    // light.next().next(); // would need to go Red -> Green -> Yellow
}
```

---

## Section 18: Performance & Profiling

### Rust's Performance Philosophy

Rust is designed to be as fast as C and C++. The language provides zero-cost abstractions — high-level constructs that compile down to the same machine code as hand-written low-level code. Iterators, closures, and generics all have zero runtime overhead compared to their manual equivalents.

However, writing fast Rust code requires understanding where performance comes from and how to measure it.

### Profiling with Flamegraphs

Before optimizing, always profile. Optimization without measurement is guesswork.

```bash
# Install cargo-flamegraph
cargo install flamegraph

# Generate a flamegraph (Linux)
cargo flamegraph --bin my_app

# On macOS
cargo flamegraph --bin my_app -- --root

# Install perf-based profiling
cargo install cargo-profdata
```

### Cache-Friendly Data Structures

Modern CPUs are much faster at accessing sequential memory than random memory. Designing data structures that are cache-friendly can have a dramatic impact on performance.

```rust
// Cache-unfriendly: Array of Structs (AoS)
// Each struct has multiple fields; iterating over one field
// loads all fields into cache unnecessarily
struct ParticleAoS {
    x: f32, y: f32, z: f32,
    vx: f32, vy: f32, vz: f32,
    mass: f32,
}

// Cache-friendly: Struct of Arrays (SoA)
// Each field is in its own array; iterating over x loads
// only x values into cache
struct ParticlesSoA {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<f32>,
    vx: Vec<f32>,
    vy: Vec<f32>,
    vz: Vec<f32>,
    mass: Vec<f32>,
}

impl ParticlesSoA {
    fn new(n: usize) -> Self {
        Self {
            x: vec![0.0; n], y: vec![0.0; n], z: vec![0.0; n],
            vx: vec![0.0; n], vy: vec![0.0; n], vz: vec![0.0; n],
            mass: vec![1.0; n],
        }
    }

    // This loop is cache-friendly: accesses x, vx, y, vy, z, vz sequentially
    fn update(&mut self, dt: f32) {
        for i in 0..self.x.len() {
            self.x[i] += self.vx[i] * dt;
            self.y[i] += self.vy[i] * dt;
            self.z[i] += self.vz[i] * dt;
        }
    }
}
```

### SIMD: Single Instruction, Multiple Data

SIMD instructions allow a single CPU instruction to operate on multiple data values simultaneously. Rust provides access to SIMD through the `std::arch` module and the `packed_simd` crate.

```rust
// The compiler can often auto-vectorize simple loops
// Use #[target_feature] to enable specific CPU features
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

fn sum_f32_scalar(data: &[f32]) -> f32 {
    data.iter().sum()
}

// Manual SIMD (advanced — usually let the compiler do this)
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn sum_f32_avx2(data: &[f32]) -> f32 {
    let mut sum = _mm256_setzero_ps();
    let chunks = data.chunks_exact(8);
    let remainder = chunks.remainder();

    for chunk in chunks {
        let v = _mm256_loadu_ps(chunk.as_ptr());
        sum = _mm256_add_ps(sum, v);
    }

    // Horizontal sum of the 8 floats in the SIMD register
    let mut result = [0f32; 8];
    _mm256_storeu_ps(result.as_mut_ptr(), sum);
    result.iter().sum::<f32>() + remainder.iter().sum::<f32>()
}
```

### Mini Project 16: Performance Comparison

```rust
use std::time::Instant;

fn sum_naive(data: &[f64]) -> f64 {
    let mut sum = 0.0;
    for &x in data {
        sum += x;
    }
    sum
}

fn sum_iterator(data: &[f64]) -> f64 {
    data.iter().sum()
}

fn sum_chunks(data: &[f64]) -> f64 {
    data.chunks(4).map(|chunk| chunk.iter().sum::<f64>()).sum()
}

fn benchmark<F: Fn(&[f64]) -> f64>(name: &str, f: F, data: &[f64], iterations: u32) {
    // Warmup
    for _ in 0..10 {
        let _ = f(data);
    }

    let start = Instant::now();
    let mut result = 0.0;
    for _ in 0..iterations {
        result = f(data);
    }
    let elapsed = start.elapsed();

    println!("{name}: {:.2}ms (result={result:.2})", 
        elapsed.as_secs_f64() * 1000.0 / iterations as f64);
}

fn main() {
    let data: Vec<f64> = (0..1_000_000).map(|i| i as f64).collect();

    benchmark("naive", sum_naive, &data, 100);
    benchmark("iterator", sum_iterator, &data, 100);
    benchmark("chunks", sum_chunks, &data, 100);
}
```

---

## Section 19: Networking & Web

### TCP and UDP with std::net

Rust's standard library provides low-level networking through `std::net`. For production applications, you will typically use higher-level libraries, but understanding the fundamentals is important.

```rust
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::thread;

fn handle_client(stream: TcpStream) {
    let peer = stream.peer_addr().unwrap();
    println!("New connection from {peer}");

    let mut reader = BufReader::new(&stream);
    let mut writer = &stream;

    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => break, // connection closed
            Ok(_) => {
                let response = format!("Echo: {line}");
                if writer.write_all(response.as_bytes()).is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
    }
    println!("Connection from {peer} closed");
}

fn run_echo_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => eprintln!("Error: {e}"),
        }
    }
}
```

### HTTP with reqwest

`reqwest` is the most popular HTTP client library for Rust. It supports both synchronous and asynchronous requests.

```rust
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Post {
    id: u32,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: u32,
}

#[derive(Serialize)]
struct NewPost {
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // GET request
    let post: Post = client
        .get("https://jsonplaceholder.typicode.com/posts/1")
        .send()
        .await?
        .json()
        .await?;

    println!("Post: {:#?}", post);

    // POST request
    let new_post = NewPost {
        title: "My Post".to_string(),
        body: "Hello, World!".to_string(),
        user_id: 1,
    };

    let created: serde_json::Value = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await?
        .json()
        .await?;

    println!("Created: {created}");

    Ok(())
}
```

### Building a REST API with Axum

Axum is a modern, ergonomic web framework built on top of Tokio and Tower. It is the recommended choice for building HTTP APIs in Rust.

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

type Db = Arc<RwLock<HashMap<u64, User>>>;

async fn list_users(State(db): State<Db>) -> Json<Vec<User>> {
    let users = db.read().unwrap();
    Json(users.values().cloned().collect())
}

async fn get_user(
    Path(id): Path<u64>,
    State(db): State<Db>,
) -> Result<Json<User>, StatusCode> {
    let users = db.read().unwrap();
    users.get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

async fn create_user(
    State(db): State<Db>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let mut users = db.write().unwrap();
    let id = users.len() as u64 + 1;
    let user = User { id, name: payload.name, email: payload.email };
    users.insert(id, user.clone());
    (StatusCode::CREATED, Json(user))
}

async fn delete_user(
    Path(id): Path<u64>,
    State(db): State<Db>,
) -> StatusCode {
    let mut users = db.write().unwrap();
    if users.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(RwLock::new(HashMap::new()));

    let app = Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user).delete(delete_user))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
```

### Mini Project 17: REST API with Middleware

```rust
use axum::{
    middleware::{self, Next},
    extract::Request,
    response::Response,
    http::HeaderMap,
};
use std::time::Instant;

// Logging middleware
async fn logging_middleware(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = Instant::now();

    let response = next.run(req).await;

    println!("{} {} — {} ({:?})",
        method, uri, response.status(), start.elapsed());

    response
}

// Auth middleware
async fn auth_middleware(
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth = headers.get("Authorization")
        .and_then(|v| v.to_str().ok());

    match auth {
        Some(token) if token.starts_with("Bearer ") => Ok(next.run(req).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
```

---

## Section 20: Serde Deep Dive

### What Is Serde?

Serde (Serialize/Deserialize) is Rust's de facto standard for data serialization and deserialization. It is a framework, not a specific format — it works with JSON, TOML, YAML, MessagePack, Bincode, and many other formats through a common interface.

The key insight of Serde is that it separates the *data model* (your Rust types) from the *data format* (JSON, TOML, etc.). You implement Serde's traits once for your types, and then you can serialize to and deserialize from any supported format.

```rust
use serde::{Deserialize, Serialize};
use serde_json;

// The derive macros generate Serialize and Deserialize implementations
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
    tags: Vec<String>,
    database: DatabaseConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
    timeout_seconds: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        host: "localhost".to_string(),
        port: 8080,
        debug: true,
        tags: vec!["web".to_string(), "api".to_string()],
        database: DatabaseConfig {
            url: "postgres://localhost/mydb".to_string(),
            max_connections: 10,
            timeout_seconds: 30,
        },
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&config)?;
    println!("{json}");

    // Deserialize from JSON
    let loaded: Config = serde_json::from_str(&json)?;
    println!("Loaded: {:#?}", loaded);

    Ok(())
}
```

### Serde Attributes

Serde provides many attributes to customize serialization behavior.

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // convert field names to camelCase
struct ApiResponse {
    user_id: u64,           // serialized as "userId"
    first_name: String,     // serialized as "firstName"

    #[serde(rename = "email_address")] // override the rename_all
    email: String,

    #[serde(skip_serializing_if = "Option::is_none")] // omit if None
    middle_name: Option<String>,

    #[serde(default)] // use Default::default() if field is missing
    is_active: bool,

    #[serde(skip)] // never serialize or deserialize this field
    internal_id: u64,

    #[serde(flatten)] // inline the fields of this struct
    metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    created_at: String,
    updated_at: String,
}

// Enums with serde
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")] // adjacently tagged
enum Event {
    #[serde(rename = "user_created")]
    UserCreated { id: u64, name: String },

    #[serde(rename = "order_placed")]
    OrderPlaced { order_id: u64, amount: f64 },

    #[serde(rename = "system_error")]
    SystemError(String),
}
```

### Custom Serializers and Deserializers

Sometimes the default serialization is not what you want. You can implement custom serializers and deserializers.

```rust
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

// A type that serializes as a string but stores as a number
#[derive(Debug, Clone, Copy)]
struct Milliseconds(u64);

impl Serialize for Milliseconds {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Serialize as "1234ms"
        serializer.serialize_str(&format!("{}ms", self.0))
    }
}

impl<'de> Deserialize<'de> for Milliseconds {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MsVisitor;

        impl<'de> serde::de::Visitor<'de> for MsVisitor {
            type Value = Milliseconds;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a string like '1234ms'")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Milliseconds, E> {
                v.strip_suffix("ms")
                    .and_then(|n| n.parse().ok())
                    .map(Milliseconds)
                    .ok_or_else(|| E::custom(format!("invalid milliseconds: {v}")))
            }
        }

        deserializer.deserialize_str(MsVisitor)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ms = Milliseconds(1500);
    let json = serde_json::to_string(&ms)?;
    println!("Serialized: {json}"); // "1500ms"

    let parsed: Milliseconds = serde_json::from_str(&json)?;
    println!("Parsed: {:?}", parsed);

    Ok(())
}
```

### Mini Project 18: Multi-Format Config System

```rust
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AppConfig {
    app_name: String,
    version: String,
    server: ServerConfig,
    logging: LoggingConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ServerConfig {
    host: String,
    port: u16,
    workers: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LoggingConfig {
    level: String,
    file: Option<String>,
}

impl AppConfig {
    fn from_json(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    fn to_json(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    fn default_config() -> Self {
        Self {
            app_name: "MyApp".to_string(),
            version: "1.0.0".to_string(),
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                workers: 4,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: None,
            },
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::default_config();

    let path = Path::new("config.json");
    config.to_json(path)?;
    println!("Saved config to {}", path.display());

    let loaded = AppConfig::from_json(path)?;
    println!("Loaded: {:#?}", loaded);

    std::fs::remove_file(path)?;
    Ok(())
}
```

---

## Capstone Project: logforge

### Overview

`logforge` is a distributed log aggregation and analysis engine — a production-grade system that you will build incrementally, applying every concept from this guide. It is inspired by real-world tools like Elasticsearch, Loki, and Splunk.

### What logforge Does

logforge ingests log entries from multiple sources (TCP, UDP, files, stdin), processes them through a configurable transformation pipeline, stores them in a segment-based storage engine with an inverted index, and exposes a query API via REST and WebSocket.

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        logforge                              │
│                                                             │
│  ┌──────────┐    ┌──────────────┐    ┌──────────────────┐  │
│  │ Ingestion│───▶│  Transform   │───▶│  Storage Engine  │  │
│  │  Layer   │    │  Pipeline    │    │  (Segments +     │  │
│  │          │    │              │    │   Inverted Index) │  │
│  │ TCP/UDP  │    │ Parse/Filter │    │                  │  │
│  │ File     │    │ Enrich/Route │    │  Segment files   │  │
│  │ Stdin    │    │              │    │  Index files     │  │
│  └──────────┘    └──────────────┘    └────────┬─────────┘  │
│                                               │             │
│  ┌────────────────────────────────────────────▼──────────┐  │
│  │                    Query Engine                        │  │
│  │  Time range / Level / Source / Tag / Full-text search │  │
│  └────────────────────────────────────────────┬──────────┘  │
│                                               │             │
│  ┌────────────────────────────────────────────▼──────────┐  │
│  │                  REST API + WebSocket                  │  │
│  │  GET /logs  POST /query  WS /stream                   │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Workspace Structure

```
logforge/
├── Cargo.toml              (workspace)
├── crates/
│   ├── logforge-core/      (shared types, traits)
│   ├── logforge-ingest/    (ingestion layer)
│   ├── logforge-pipeline/  (transformation pipeline)
│   ├── logforge-storage/   (storage engine)
│   ├── logforge-query/     (query engine)
│   └── logforge-api/       (REST + WebSocket API)
└── logforge-cli/           (CLI binary)
```

### Milestone 1: Core Types and Traits

```rust
// crates/logforge-core/src/lib.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// A log entry — the fundamental unit of data in logforge
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogEntry {
    /// Unique identifier for this log entry
    pub id: u64,

    /// Unix timestamp in milliseconds
    pub timestamp_ms: u64,

    /// Log level (DEBUG, INFO, WARN, ERROR, FATAL)
    pub level: LogLevel,

    /// The source that produced this log (hostname, service name, etc.)
    pub source: String,

    /// The log message
    pub message: String,

    /// Arbitrary key-value metadata
    pub fields: HashMap<String, String>,

    /// Tags for categorization
    pub tags: Vec<String>,
}

impl LogEntry {
    pub fn new(level: LogLevel, source: &str, message: &str) -> Self {
        Self {
            id: 0, // assigned by storage
            timestamp_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            level,
            source: source.to_string(),
            message: message.to_string(),
            fields: HashMap::new(),
            tags: Vec::new(),
        }
    }

    pub fn with_field(mut self, key: &str, value: &str) -> Self {
        self.fields.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
    Fatal = 4,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info  => write!(f, "INFO"),
            LogLevel::Warn  => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Fatal => write!(f, "FATAL"),
        }
    }
}

impl std::str::FromStr for LogLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DEBUG" => Ok(LogLevel::Debug),
            "INFO"  => Ok(LogLevel::Info),
            "WARN"  => Ok(LogLevel::Warn),
            "ERROR" => Ok(LogLevel::Error),
            "FATAL" => Ok(LogLevel::Fatal),
            _ => Err(format!("Unknown log level: {s}")),
        }
    }
}

/// A query for searching log entries
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LogQuery {
    pub start_ms: Option<u64>,
    pub end_ms: Option<u64>,
    pub levels: Vec<LogLevel>,
    pub sources: Vec<String>,
    pub tags: Vec<String>,
    pub message_contains: Option<String>,
    pub limit: Option<usize>,
}

/// Trait for anything that can store and retrieve log entries
pub trait LogStore: Send + Sync {
    fn append(&self, entry: LogEntry) -> Result<u64, StoreError>;
    fn query(&self, query: &LogQuery) -> Result<Vec<LogEntry>, StoreError>;
    fn count(&self) -> Result<u64, StoreError>;
}

/// Trait for pipeline stages that transform log entries
pub trait PipelineStage: Send + Sync {
    fn name(&self) -> &str;
    fn process(&self, entry: LogEntry) -> Option<LogEntry>;
}

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Index error: {0}")]
    Index(String),
}
```

### Milestone 2: Ingestion Layer

```rust
// crates/logforge-ingest/src/lib.rs

use logforge_core::{LogEntry, LogLevel, PipelineStage};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, UdpSocket};
use tokio::sync::mpsc;

pub struct Ingester {
    pipeline: Vec<Arc<dyn PipelineStage>>,
    sender: mpsc::Sender<LogEntry>,
}

impl Ingester {
    pub fn new(
        pipeline: Vec<Arc<dyn PipelineStage>>,
        sender: mpsc::Sender<LogEntry>,
    ) -> Self {
        Self { pipeline, sender }
    }

    async fn process_and_send(&self, entry: LogEntry) {
        let mut current = entry;
        for stage in &self.pipeline {
            match stage.process(current) {
                Some(e) => current = e,
                None => return, // entry was filtered out
            }
        }
        let _ = self.sender.send(current).await;
    }

    pub async fn listen_tcp(self: Arc<Self>, addr: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(addr).await?;
        println!("TCP ingestion listening on {addr}");

        loop {
            let (socket, peer) = listener.accept().await?;
            let ingester = Arc::clone(&self);

            tokio::spawn(async move {
                let reader = BufReader::new(socket);
                let mut lines = reader.lines();

                while let Ok(Some(line)) = lines.next_line().await {
                    if let Ok(entry) = serde_json::from_str::<LogEntry>(&line) {
                        ingester.process_and_send(entry).await;
                    } else {
                        // Try to parse as plain text
                        let entry = LogEntry::new(LogLevel::Info, &peer.to_string(), &line);
                        ingester.process_and_send(entry).await;
                    }
                }
            });
        }
    }

    pub async fn listen_udp(self: Arc<Self>, addr: &str) -> std::io::Result<()> {
        let socket = UdpSocket::bind(addr).await?;
        println!("UDP ingestion listening on {addr}");

        let mut buf = vec![0u8; 65536];
        loop {
            let (len, peer) = socket.recv_from(&mut buf).await?;
            let data = std::str::from_utf8(&buf[..len]).unwrap_or("");

            let entry = if let Ok(e) = serde_json::from_str::<LogEntry>(data) {
                e
            } else {
                LogEntry::new(LogLevel::Info, &peer.to_string(), data)
            };

            self.process_and_send(entry).await;
        }
    }
}
```

### Milestone 3: Transformation Pipeline

```rust
// crates/logforge-pipeline/src/lib.rs

use logforge_core::{LogEntry, LogLevel, PipelineStage};
use std::collections::HashMap;

/// Filter entries by minimum log level
pub struct LevelFilter {
    min_level: LogLevel,
}

impl LevelFilter {
    pub fn new(min_level: LogLevel) -> Self {
        Self { min_level }
    }
}

impl PipelineStage for LevelFilter {
    fn name(&self) -> &str { "level_filter" }

    fn process(&self, entry: LogEntry) -> Option<LogEntry> {
        if entry.level >= self.min_level {
            Some(entry)
        } else {
            None // filter out
        }
    }
}

/// Add fields to every log entry
pub struct FieldEnricher {
    fields: HashMap<String, String>,
}

impl FieldEnricher {
    pub fn new(fields: HashMap<String, String>) -> Self {
        Self { fields }
    }
}

impl PipelineStage for FieldEnricher {
    fn name(&self) -> &str { "field_enricher" }

    fn process(&self, mut entry: LogEntry) -> Option<LogEntry> {
        for (k, v) in &self.fields {
            entry.fields.entry(k.clone()).or_insert_with(|| v.clone());
        }
        Some(entry)
    }
}

/// Redact sensitive information from log messages
pub struct Redactor {
    patterns: Vec<(String, String)>, // (pattern, replacement)
}

impl Redactor {
    pub fn new() -> Self {
        Self {
            patterns: vec![
                // Redact credit card numbers
                (r"\b\d{4}[- ]?\d{4}[- ]?\d{4}[- ]?\d{4}\b".to_string(), 
                 "[REDACTED-CC]".to_string()),
                // Redact email addresses
                (r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}".to_string(),
                 "[REDACTED-EMAIL]".to_string()),
            ],
        }
    }
}

impl PipelineStage for Redactor {
    fn name(&self) -> &str { "redactor" }

    fn process(&self, mut entry: LogEntry) -> Option<LogEntry> {
        // In a real implementation, use the `regex` crate
        // For simplicity, we just demonstrate the pattern
        for (pattern, replacement) in &self.patterns {
            if entry.message.contains(pattern.as_str()) {
                entry.message = entry.message.replace(pattern.as_str(), replacement);
            }
        }
        Some(entry)
    }
}

/// Parse structured data from log messages
pub struct JsonParser;

impl PipelineStage for JsonParser {
    fn name(&self) -> &str { "json_parser" }

    fn process(&self, mut entry: LogEntry) -> Option<LogEntry> {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&entry.message) {
            if let Some(obj) = value.as_object() {
                for (k, v) in obj {
                    if let Some(s) = v.as_str() {
                        entry.fields.insert(k.clone(), s.to_string());
                    }
                }
            }
        }
        Some(entry)
    }
}
```

### Milestone 4: Storage Engine

```rust
// crates/logforge-storage/src/lib.rs

use logforge_core::{LogEntry, LogQuery, LogStore, StoreError};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Write, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock, atomic::{AtomicU64, Ordering}};

const SEGMENT_SIZE: usize = 10_000; // entries per segment

struct Segment {
    id: u64,
    path: PathBuf,
    entries: Vec<LogEntry>,
    dirty: bool,
}

impl Segment {
    fn new(id: u64, dir: &Path) -> Self {
        Self {
            id,
            path: dir.join(format!("segment_{:08}.jsonl", id)),
            entries: Vec::new(),
            dirty: false,
        }
    }

    fn load(path: PathBuf) -> Result<Self, StoreError> {
        let id: u64 = path.file_stem()
            .and_then(|s| s.to_str())
            .and_then(|s| s.strip_prefix("segment_"))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let mut entries = Vec::new();
        if path.exists() {
            let file = File::open(&path)?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                if let Ok(entry) = serde_json::from_str(&line) {
                    entries.push(entry);
                }
            }
        }

        Ok(Self { id, path, entries, dirty: false })
    }

    fn flush(&mut self) -> Result<(), StoreError> {
        if !self.dirty { return Ok(()); }

        let temp_path = self.path.with_extension("tmp");
        {
            let file = File::create(&temp_path)?;
            let mut writer = BufWriter::new(&file);
            for entry in &self.entries {
                let line = serde_json::to_string(entry)
                    .map_err(|e| StoreError::Serialization(e.to_string()))?;
                writeln!(writer, "{line}")?;
            }
            writer.flush()?;
            file.sync_all()?;
        }
        fs::rename(&temp_path, &self.path)?;
        self.dirty = false;
        Ok(())
    }
}

/// Inverted index for fast text search
struct InvertedIndex {
    // word -> set of entry IDs
    index: HashMap<String, Vec<u64>>,
}

impl InvertedIndex {
    fn new() -> Self {
        Self { index: HashMap::new() }
    }

    fn index_entry(&mut self, entry: &LogEntry) {
        for word in entry.message.split_whitespace() {
            let word = word.to_lowercase();
            let word = word.trim_matches(|c: char| !c.is_alphanumeric());
            if !word.is_empty() {
                self.index.entry(word.to_string())
                    .or_default()
                    .push(entry.id);
            }
        }
    }

    fn search(&self, term: &str) -> Vec<u64> {
        self.index.get(&term.to_lowercase())
            .cloned()
            .unwrap_or_default()
    }
}

pub struct SegmentedStore {
    dir: PathBuf,
    segments: RwLock<Vec<Segment>>,
    index: RwLock<InvertedIndex>,
    next_id: AtomicU64,
}

impl SegmentedStore {
    pub fn open(dir: &Path) -> Result<Arc<Self>, StoreError> {
        fs::create_dir_all(dir)?;

        let mut segments = Vec::new();
        let mut max_id = 0u64;

        // Load existing segments
        let mut paths: Vec<_> = fs::read_dir(dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map(|e| e == "jsonl").unwrap_or(false))
            .collect();
        paths.sort();

        for path in paths {
            let seg = Segment::load(path)?;
            if let Some(last) = seg.entries.last() {
                max_id = max_id.max(last.id);
            }
            segments.push(seg);
        }

        if segments.is_empty() {
            segments.push(Segment::new(0, dir));
        }

        // Build index from loaded entries
        let mut index = InvertedIndex::new();
        for seg in &segments {
            for entry in &seg.entries {
                index.index_entry(entry);
            }
        }

        Ok(Arc::new(Self {
            dir: dir.to_path_buf(),
            segments: RwLock::new(segments),
            index: RwLock::new(index),
            next_id: AtomicU64::new(max_id + 1),
        }))
    }

    pub fn flush(&self) -> Result<(), StoreError> {
        let mut segments = self.segments.write().unwrap();
        for seg in segments.iter_mut() {
            seg.flush()?;
        }
        Ok(())
    }
}

impl LogStore for SegmentedStore {
    fn append(&self, mut entry: LogEntry) -> Result<u64, StoreError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        entry.id = id;

        // Update inverted index
        self.index.write().unwrap().index_entry(&entry);

        // Append to current segment
        let mut segments = self.segments.write().unwrap();
        let last = segments.last_mut().unwrap();
        last.entries.push(entry);
        last.dirty = true;

        // Rotate segment if full
        if last.entries.len() >= SEGMENT_SIZE {
            last.flush()?;
            let new_id = last.id + 1;
            segments.push(Segment::new(new_id, &self.dir));
        }

        Ok(id)
    }

    fn query(&self, query: &LogQuery) -> Result<Vec<LogEntry>, StoreError> {
        let segments = self.segments.read().unwrap();
        let index = self.index.read().unwrap();

        // Get candidate IDs from full-text search
        let text_candidates: Option<std::collections::HashSet<u64>> = 
            query.message_contains.as_ref().map(|term| {
                index.search(term).into_iter().collect()
            });

        let mut results = Vec::new();

        for seg in segments.iter() {
            for entry in &seg.entries {
                // Time range filter
                if let Some(start) = query.start_ms {
                    if entry.timestamp_ms < start { continue; }
                }
                if let Some(end) = query.end_ms {
                    if entry.timestamp_ms > end { continue; }
                }

                // Level filter
                if !query.levels.is_empty() && !query.levels.contains(&entry.level) {
                    continue;
                }

                // Source filter
                if !query.sources.is_empty() && !query.sources.contains(&entry.source) {
                    continue;
                }

                // Tag filter
                if !query.tags.is_empty() {
                    let has_all_tags = query.tags.iter()
                        .all(|t| entry.tags.contains(t));
                    if !has_all_tags { continue; }
                }

                // Full-text filter
                if let Some(ref candidates) = text_candidates {
                    if !candidates.contains(&entry.id) { continue; }
                }

                results.push(entry.clone());

                if let Some(limit) = query.limit {
                    if results.len() >= limit { break; }
                }
            }
        }

        Ok(results)
    }

    fn count(&self) -> Result<u64, StoreError> {
        Ok(self.next_id.load(Ordering::SeqCst))
    }
}
```

### Milestone 5: REST API with WebSocket Streaming

```rust
// crates/logforge-api/src/lib.rs

use axum::{
    extract::{State, WebSocketUpgrade, ws::{WebSocket, Message}},
    response::{Json, Response},
    routing::{get, post},
    Router,
};
use logforge_core::{LogEntry, LogQuery, LogStore};
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct AppState {
    pub store: Arc<dyn LogStore>,
    pub broadcast: broadcast::Sender<LogEntry>,
}

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/logs", post(ingest_log))
        .route("/query", post(query_logs))
        .route("/count", get(count_logs))
        .route("/stream", get(stream_logs))
        .with_state(state)
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok" }))
}

async fn ingest_log(
    State(state): State<Arc<AppState>>,
    Json(entry): Json<LogEntry>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let id = state.store.append(entry.clone())
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Broadcast to WebSocket subscribers
    let _ = state.broadcast.send(entry);

    Ok(Json(serde_json::json!({ "id": id })))
}

async fn query_logs(
    State(state): State<Arc<AppState>>,
    Json(query): Json<LogQuery>,
) -> Result<Json<Vec<LogEntry>>, (axum::http::StatusCode, String)> {
    let results = state.store.query(&query)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(results))
}

async fn count_logs(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let count = state.store.count()
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(serde_json::json!({ "count": count })))
}

async fn stream_logs(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

async fn handle_websocket(mut socket: WebSocket, state: Arc<AppState>) {
    let mut rx = state.broadcast.subscribe();

    loop {
        match rx.recv().await {
            Ok(entry) => {
                let json = serde_json::to_string(&entry).unwrap_or_default();
                if socket.send(Message::Text(json)).await.is_err() {
                    break; // client disconnected
                }
            }
            Err(broadcast::error::RecvError::Lagged(n)) => {
                eprintln!("WebSocket subscriber lagged by {n} messages");
            }
            Err(broadcast::error::RecvError::Closed) => break,
        }
    }
}
```

### Milestone 6: CLI and Integration

```rust
// logforge-cli/src/main.rs

use clap::{Parser, Subcommand};
use logforge_core::{LogEntry, LogLevel, LogQuery};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "logforge", about = "Distributed log aggregation engine")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the logforge server
    Serve {
        #[arg(long, default_value = "0.0.0.0:3000")]
        http_addr: String,

        #[arg(long, default_value = "0.0.0.0:5140")]
        tcp_addr: String,

        #[arg(long, default_value = "./data")]
        data_dir: PathBuf,
    },

    /// Query logs from a running server
    Query {
        #[arg(long)]
        server: Option<String>,

        #[arg(long)]
        level: Option<String>,

        #[arg(long)]
        source: Option<String>,

        #[arg(long)]
        contains: Option<String>,

        #[arg(long, default_value = "100")]
        limit: usize,
    },

    /// Send a log entry to a running server
    Send {
        #[arg(long, default_value = "INFO")]
        level: String,

        #[arg(long, default_value = "cli")]
        source: String,

        message: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Serve { http_addr, tcp_addr, data_dir } => {
            println!("Starting logforge server...");
            println!("HTTP: {http_addr}");
            println!("TCP:  {tcp_addr}");
            println!("Data: {}", data_dir.display());

            // Initialize storage
            let store = logforge_storage::SegmentedStore::open(&data_dir)?;

            // Initialize broadcast channel for WebSocket streaming
            let (tx, _) = tokio::sync::broadcast::channel(1024);

            let state = std::sync::Arc::new(logforge_api::AppState {
                store: store.clone(),
                broadcast: tx.clone(),
            });

            // Start HTTP server
            let router = logforge_api::create_router(state);
            let listener = tokio::net::TcpListener::bind(&http_addr).await?;

            println!("logforge ready!");
            axum::serve(listener, router).await?;
        }

        Commands::Query { server, level, source, contains, limit } => {
            let server = server.unwrap_or_else(|| "http://localhost:3000".to_string());

            let query = LogQuery {
                levels: level.and_then(|l| l.parse::<LogLevel>().ok())
                    .map(|l| vec![l])
                    .unwrap_or_default(),
                sources: source.map(|s| vec![s]).unwrap_or_default(),
                message_contains: contains,
                limit: Some(limit),
                ..Default::default()
            };

            let client = reqwest::Client::new();
            let entries: Vec<LogEntry> = client
                .post(format!("{server}/query"))
                .json(&query)
                .send()
                .await?
                .json()
                .await?;

            for entry in &entries {
                println!("[{}] [{}] {} — {}",
                    entry.timestamp_ms, entry.level, entry.source, entry.message);
            }
            println!("({} results)", entries.len());
        }

        Commands::Send { level, source, message } => {
            let level = level.parse::<LogLevel>()
                .unwrap_or(LogLevel::Info);

            let entry = LogEntry::new(level, &source, &message);

            let client = reqwest::Client::new();
            let response: serde_json::Value = client
                .post("http://localhost:3000/logs")
                .json(&entry)
                .send()
                .await?
                .json()
                .await?;

            println!("Sent: id={}", response["id"]);
        }
    }

    Ok(())
}
```

### Milestone 7: Testing the Full System

```rust
// tests/integration_test.rs

use logforge_core::{LogEntry, LogLevel, LogQuery, LogStore};
use logforge_storage::SegmentedStore;
use tempfile::TempDir;

fn make_store() -> (SegmentedStore, TempDir) {
    let dir = TempDir::new().unwrap();
    let store = SegmentedStore::open(dir.path()).unwrap();
    (store, dir)
}

#[test]
fn test_append_and_query() {
    let (store, _dir) = make_store();

    let entry = LogEntry::new(LogLevel::Info, "test", "hello world");
    let id = store.append(entry).unwrap();
    assert!(id > 0);

    let results = store.query(&LogQuery::default()).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].message, "hello world");
}

#[test]
fn test_level_filter() {
    let (store, _dir) = make_store();

    store.append(LogEntry::new(LogLevel::Debug, "test", "debug msg")).unwrap();
    store.append(LogEntry::new(LogLevel::Info, "test", "info msg")).unwrap();
    store.append(LogEntry::new(LogLevel::Error, "test", "error msg")).unwrap();

    let query = LogQuery {
        levels: vec![LogLevel::Error],
        ..Default::default()
    };

    let results = store.query(&query).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].level, LogLevel::Error);
}

#[test]
fn test_full_text_search() {
    let (store, _dir) = make_store();

    store.append(LogEntry::new(LogLevel::Info, "app", "user logged in successfully")).unwrap();
    store.append(LogEntry::new(LogLevel::Error, "app", "database connection failed")).unwrap();
    store.append(LogEntry::new(LogLevel::Info, "app", "user logged out")).unwrap();

    let query = LogQuery {
        message_contains: Some("logged".to_string()),
        ..Default::default()
    };

    let results = store.query(&query).unwrap();
    assert_eq!(results.len(), 2);
}

#[tokio::test]
async fn test_api_ingest_and_query() {
    use axum::http::StatusCode;
    use axum_test::TestServer;

    let (store, _dir) = make_store();
    let (tx, _) = tokio::sync::broadcast::channel(16);

    let state = std::sync::Arc::new(logforge_api::AppState {
        store: std::sync::Arc::new(store),
        broadcast: tx,
    });

    let app = logforge_api::create_router(state);
    let server = TestServer::new(app).unwrap();

    // Ingest a log entry
    let entry = LogEntry::new(LogLevel::Info, "test", "test message");
    let response = server.post("/logs").json(&entry).await;
    assert_eq!(response.status_code(), StatusCode::OK);

    // Query it back
    let query = LogQuery::default();
    let response = server.post("/query").json(&query).await;
    assert_eq!(response.status_code(), StatusCode::OK);

    let entries: Vec<LogEntry> = response.json();
    assert_eq!(entries.len(), 1);
}
```

### Stretch Goals

Once you have completed the 7 milestones, challenge yourself with these extensions:

1. **Compression**: Compress segment files using `zstd` or `lz4` to reduce disk usage
2. **Distributed mode**: Use `raft-rs` to implement consensus across multiple nodes
3. **S3 archival**: Archive old segments to S3 using the `aws-sdk-s3` crate
4. **WASM plugins**: Allow pipeline stages to be written in any language and compiled to WASM
5. **Metrics**: Expose Prometheus metrics using `prometheus` crate
6. **TLS**: Add TLS support to the HTTP and TCP servers using `rustls`
7. **Authentication**: Add JWT-based authentication to the API
8. **Alerting**: Send alerts when error rates exceed thresholds
9. **Dashboard**: Build a simple web dashboard using `axum` and HTMX
10. **Benchmarking**: Write comprehensive benchmarks and optimize the hot paths

---

## Appendix: Essential Crates Reference

| Category | Crate | Purpose |
|---|---|---|
| Async runtime | `tokio` | The standard async runtime |
| HTTP client | `reqwest` | HTTP client with async support |
| HTTP server | `axum` | Modern web framework |
| Serialization | `serde` + `serde_json` | JSON and other formats |
| Error handling | `anyhow` + `thiserror` | Application and library errors |
| CLI | `clap` | Command-line argument parsing |
| Logging | `tracing` | Structured, async-aware logging |
| Testing | `mockall` + `proptest` | Mocking and property testing |
| Benchmarking | `criterion` | Statistical benchmarking |
| Database | `sqlx` | Async SQL with compile-time checks |
| Config | `config` | Layered configuration |
| Regex | `regex` | Regular expressions |
| UUID | `uuid` | UUID generation |
| Time | `chrono` | Date and time handling |
| Compression | `flate2` | gzip/zlib compression |
| Crypto | `ring` | Cryptographic operations |
| Temp files | `tempfile` | Temporary files and directories |

---

*This guide is a living document. As Rust evolves, so should your understanding. The best way to learn is to build — take the concepts from each section and apply them to problems you care about. The Rust community is welcoming and the documentation is excellent. Happy hacking!*
