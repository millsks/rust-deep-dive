# 🦀 The Complete Rust Developer Roadmap
### From Novice to Pro — A Hands-On Learning Journey

---

## Table of Contents

1. [Section 1: Getting Started with Rust](#section-1-getting-started-with-rust)
2. [Section 2: Ownership, Borrowing & Lifetimes — Deep Dive](#section-2-ownership-borrowing--lifetimes--deep-dive)
3. [Section 3: Structs, Enums & Pattern Matching](#section-3-structs-enums--pattern-matching)
4. [Section 4: Smart Pointers & Interior Mutability](#section-4-smart-pointers--interior-mutability)
5. [Section 5: Error Handling — Novice to Pro](#section-5-error-handling--novice-to-pro)
6. [Section 6: Collections & Iterators — Deep Dive](#section-6-collections--iterators--deep-dive)
7. [Section 7: Traits & Generics — Deep Dive](#section-7-traits--generics--deep-dive)
8. [Section 8: Closures & Functional Programming](#section-8-closures--functional-programming)
9. [Section 9: Modules, Crates & Cargo — Deep Dive](#section-9-modules-crates--cargo--deep-dive)
10. [Section 10: Working with Files](#section-10-working-with-files)
11. [Section 11: Testing, Linting & Formatting](#section-11-testing-linting--formatting)
12. [Section 12: Concurrency & Async Programming — Deep Dive](#section-12-concurrency--async-programming--deep-dive)
13. [Section 13: Macros — Declarative & Procedural](#section-13-macros--declarative--procedural)
14. [Section 14: Unsafe Rust & FFI](#section-14-unsafe-rust--ffi)
15. [Section 15: Design Patterns in Rust](#section-15-design-patterns-in-rust)
16. [Section 16: Performance, Profiling & Optimization](#section-16-performance-profiling--optimization)
17. [Section 17: Networking & Web Development](#section-17-networking--web-development)
18. [Section 18: Data Serialization with Serde](#section-18-data-serialization-with-serde)
19. [Capstone Project: Distributed Log Aggregation & Analysis Engine](#capstone-project-distributed-log-aggregation--analysis-engine)

---

## Section 1: Getting Started with Rust

### 1.1 What is Rust?

Rust is a systems programming language focused on three goals: **safety**, **speed**, and **concurrency**. Unlike C or C++, Rust guarantees memory safety without a garbage collector, using a unique ownership model enforced at compile time.

**Why learn Rust?**
- No null pointer exceptions or dangling pointers
- No garbage collector — predictable, low-latency performance
- Fearless concurrency — the compiler prevents data races at compile time
- Zero-cost abstractions — high-level code compiles to optimal machine code
- Growing ecosystem: Mozilla, Microsoft, Amazon, Google, and the Linux kernel all use Rust

**Rust's core philosophy:** If it compiles, it's (memory) safe.

---

### 1.2 Installing Rust

```bash
# Install rustup (Linux/macOS)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows: Download rustup-init.exe from https://rustup.rs

# Verify installation
rustc --version
cargo --version
rustup --version

# Install useful components
rustup component add clippy        # Linter
rustup component add rustfmt       # Formatter
rustup component add rust-analyzer # LSP (use with VS Code or Neovim)
```

---

### 1.3 Your First Rust Program

```rust
fn main() {
    println!("Hello, Rustacean!");
}
```

```bash
cargo new hello_rust
cd hello_rust
cargo run
cargo build --release   # Optimized build
```

---

### 1.4 Variables, Mutability & Data Types

```rust
fn main() {
    // Immutable by default
    let x = 5;
    // x = 6; // ERROR: cannot assign twice to immutable variable

    // Explicit mutability
    let mut y = 10;
    y += 1;

    // Shadowing — redeclare with same name, can change type
    let x = x + 1;         // x is now 6
    let x = x.to_string(); // x is now a String — type changed!

    // Type annotations
    let integer: i32 = -42;
    let unsigned: u64 = 1_000_000; // underscores for readability
    let float: f64 = 3.14159;
    let boolean: bool = true;
    let character: char = '🦀';

    // Compound types
    let tuple: (i32, f64, bool) = (42, 3.14, true);
    let (a, b, c) = tuple; // destructuring
    let first = tuple.0;   // index access

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let zeros = [0; 10];   // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    println!("x={}, y={}, a={}, b={}, c={}", x, y, a, b, c);
}
```

**Integer types:**
| Type | Size | Range |
|------|------|-------|
| `i8` | 8-bit | -128 to 127 |
| `i32` | 32-bit | -2^31 to 2^31-1 |
| `i64` | 64-bit | -2^63 to 2^63-1 |
| `u8` | 8-bit | 0 to 255 |
| `u32` | 32-bit | 0 to 2^32-1 |
| `usize` | arch | pointer-sized |

---

### 1.5 Control Flow

```rust
fn main() {
    let number = 7;

    // if/else — expressions, not statements
    let description = if number % 2 == 0 { "even" } else { "odd" };

    // loop — infinite loop with break value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // returns 20
        }
    };

    // while
    let mut n = 3;
    while n != 0 {
        println!("{}!", n);
        n -= 1;
    }

    // for — idiomatic Rust iteration
    for i in 0..5 {        // 0, 1, 2, 3, 4
        print!("{} ", i);
    }
    for i in 0..=5 {       // 0, 1, 2, 3, 4, 5 (inclusive)
        print!("{} ", i);
    }

    // Labeled loops
    'outer: for x in 0..5 {
        for y in 0..5 {
            if x + y == 6 {
                break 'outer;
            }
        }
    }
}
```

---

### 1.6 Functions

```rust
// Functions are expressions — last expression is the return value
fn add(a: i32, b: i32) -> i32 {
    a + b  // no semicolon = return value
}

// Multiple return values via tuple
fn min_max(v: &[i32]) -> (i32, i32) {
    let mut min = v[0];
    let mut max = v[0];
    for &val in v.iter() {
        if val < min { min = val; }
        if val > max { max = val; }
    }
    (min, max)
}

fn main() {
    let sum = add(3, 4);
    let (lo, hi) = min_max(&[3, 1, 4, 1, 5, 9, 2, 6]);
    println!("sum={}, min={}, max={}", sum, lo, hi);
}
```

---

### 1.7 Strings

Rust has two string types — this confuses beginners constantly:

```rust
fn main() {
    // &str — string slice, immutable reference to string data
    let s1: &str = "hello";          // string literal, 'static lifetime
    let s2: &str = &s1[0..3];        // slice of s1 = "hel"

    // String — owned, heap-allocated, growable
    let mut s3: String = String::from("hello");
    s3.push(' ');
    s3.push_str("world");
    s3 += "!";

    // Converting between them
    let s4: &str = &s3;              // String -> &str (deref coercion)
    let s5: String = s4.to_string(); // &str -> String
    let s6: String = s4.to_owned();  // &str -> String (same thing)

    // String formatting
    let name = "Rustacean";
    let greeting = format!("Hello, {}!", name);

    // Useful string methods
    let upper = "hello".to_uppercase();
    let trimmed = "  hello  ".trim();
    let replaced = "hello world".replace("world", "Rust");
    let contains = "hello world".contains("world");
    let split: Vec<&str> = "a,b,c".split(',').collect();

    println!("{}", greeting);
}
```

> ⚠️ **Common Mistake:** Beginners try to index strings with `s[0]`. This doesn't work in Rust because strings are UTF-8 encoded and a single index might land in the middle of a multi-byte character. Use `.chars().nth(0)` or byte slices instead.

---

### Mini Project 1: Temperature Converter CLI

Build a command-line tool that converts between Celsius, Fahrenheit, and Kelvin.

**Requirements:**
- Accept temperature and unit as command-line arguments
- Convert to all other units
- Handle invalid input gracefully with error messages
- Use functions for each conversion formula

```bash
cargo new temp_converter
# Usage: cargo run -- 100 C
# Output:
#   100°C = 212.00°F = 373.15K
```

**Starter skeleton:**
```rust
use std::env;

fn celsius_to_fahrenheit(c: f64) -> f64 { /* ... */ }
fn celsius_to_kelvin(c: f64) -> f64 { /* ... */ }
fn fahrenheit_to_celsius(f: f64) -> f64 { /* ... */ }
fn kelvin_to_celsius(k: f64) -> f64 { /* ... */ }

fn main() {
    let args: Vec<String> = env::args().collect();
    // parse args, call converters, print results
}
```

---

## Section 2: Ownership, Borrowing & Lifetimes — Deep Dive

This is the most important section in the entire document. Rust's ownership system is what makes it unique. Take your time here.

### 2.1 The Three Rules of Ownership

1. Each value in Rust has exactly one **owner**
2. There can only be one owner at a time
3. When the owner goes out of scope, the value is **dropped** (memory freed)

```rust
fn main() {
    let s1 = String::from("hello"); // s1 owns the String
    let s2 = s1;                    // ownership MOVES to s2
    // println!("{}", s1);          // ERROR: s1 is no longer valid

    // Clone to keep both
    let s3 = String::from("world");
    let s4 = s3.clone();            // deep copy — both valid
    println!("{} {}", s3, s4);      // OK

    // Copy types (stack-only, cheap to copy)
    let x = 5;
    let y = x;  // x is COPIED, not moved — both valid
    println!("{} {}", x, y); // OK — integers implement Copy
}
```

**Types that implement `Copy`:** `i32`, `u64`, `f64`, `bool`, `char`, `()`, tuples of Copy types, arrays of Copy types.

**Types that do NOT implement `Copy`:** `String`, `Vec<T>`, `Box<T>`, any heap-allocated type.

---

### 2.2 Borrowing & References

Instead of transferring ownership, you can **borrow** a value:

```rust
fn calculate_length(s: &String) -> usize { // borrows, doesn't own
    s.len()
} // s goes out of scope but doesn't drop the String (it doesn't own it)

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // pass a reference
    println!("'{}' has {} characters", s1, len); // s1 still valid
}
```

**Mutable references:**

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

**The Borrowing Rules (enforced at compile time):**
1. You can have **any number of immutable references** (`&T`) OR
2. **Exactly one mutable reference** (`&mut T`)
3. But **never both at the same time**
4. References must always be **valid** (no dangling references)

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;     // OK
    let r2 = &s;     // OK — multiple immutable refs allowed
    // let r3 = &mut s; // ERROR: cannot borrow as mutable while immutable refs exist

    println!("{} {}", r1, r2); // r1 and r2 last used here

    // After r1 and r2 are no longer used, mutable borrow is OK
    let r3 = &mut s; // OK now
    r3.push_str(" world");
}
```

---

### 2.3 The Slice Type

Slices are references to a contiguous sequence of elements:

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
    let s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // ERROR: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("first word: {}", word);
}
```

---

### 2.4 Lifetimes — The Full Picture

Lifetimes are Rust's way of ensuring references are always valid. They are **annotations** that describe the relationship between the lifetimes of references.

**Why lifetimes exist:**

```rust
// This would be a dangling reference — Rust prevents it
fn dangle() -> &String {       // ERROR
    let s = String::from("hello");
    &s  // s is dropped here, reference would be invalid
}
```

**Lifetime annotations:**

```rust
// 'a means: the returned reference lives at least as long as both inputs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
        println!("longest: {}", result); // OK — result used within s2's scope
    }
    // println!("{}", result); // ERROR — s2 dropped, result might reference it
}
```

**Lifetimes in structs:**

```rust
// This struct cannot outlive the string it references
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 { 3 }

    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part  // lifetime elision: returns &'a str
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = ImportantExcerpt { part: first_sentence };
    println!("{}", excerpt.part);
}
```

**Lifetime Elision Rules** (when you don't need to write lifetimes explicitly):
1. Each reference parameter gets its own lifetime
2. If there's exactly one input lifetime, it's assigned to all output lifetimes
3. If one of the inputs is `&self` or `&mut self`, its lifetime is assigned to all outputs

**The `'static` lifetime:**

```rust
// 'static means the reference lives for the entire program duration
let s: &'static str = "I live forever";

// All string literals are 'static
// Be careful: don't use 'static as a lazy fix for lifetime errors
```

---

### 2.5 Common Borrow Checker Fights & Solutions

**Fight 1: Returning a reference to a local variable**
```rust
// BAD
fn bad() -> &String {
    let s = String::from("hello");
    &s // ERROR: s dropped at end of function
}

// GOOD: Return owned value
fn good() -> String {
    String::from("hello")
}
```

**Fight 2: Mutating while iterating**
```rust
// BAD
let mut v = vec![1, 2, 3];
for x in &v {
    v.push(*x * 2); // ERROR: cannot borrow as mutable while borrowed as immutable
}

// GOOD: Collect indices or clone
let additions: Vec<i32> = v.iter().map(|x| x * 2).collect();
v.extend(additions);
```

**Fight 3: Multiple mutable borrows**
```rust
// BAD
let mut v = vec![1, 2, 3];
let first = &mut v[0];
let second = &mut v[1]; // ERROR: two mutable borrows
*first += *second;

// GOOD: Use split_at_mut or indices
let (left, right) = v.split_at_mut(1);
left[0] += right[0];
```

---

### Mini Project 2: String Statistics Analyzer

Build a function library that analyzes text without unnecessary cloning.

**Requirements:**
- Count words, sentences, paragraphs using only `&str` references
- Find the longest word (return a `&str` slice)
- Calculate average word length
- All functions must take `&str`, not `String`
- Demonstrate lifetime annotations in at least one function

---

## Section 3: Structs, Enums & Pattern Matching

### 3.1 Structs

```rust
#[derive(Debug, Clone)]
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}

impl User {
    // Associated function (constructor)
    fn new(username: &str, email: &str, age: u32) -> Self {
        User {
            username: username.to_string(),
            email: email.to_string(),
            age,
            active: true,
        }
    }

    // Method — takes &self
    fn display_name(&self) -> &str {
        &self.username
    }

    // Mutable method
    fn deactivate(&mut self) {
        self.active = false;
    }

    // Consuming method — takes self
    fn into_email(self) -> String {
        self.email
    }
}

fn main() {
    let mut user = User::new("alice", "alice@example.com", 30);
    println!("{:?}", user);
    println!("Name: {}", user.display_name());
    user.deactivate();

    // Struct update syntax
    let user2 = User {
        email: String::from("bob@example.com"),
        username: String::from("bob"),
        ..user  // fill remaining fields from user (moves user!)
    };
}
```

**Tuple structs and unit structs:**

```rust
struct Point(f64, f64);       // tuple struct
struct Color(u8, u8, u8);
struct Marker;                 // unit struct (zero-size)

let p = Point(3.0, 4.0);
let distance = (p.0 * p.0 + p.1 * p.1).sqrt();
```

---

### 3.2 Enums

Rust enums are algebraic data types — each variant can hold different data:

```rust
#[derive(Debug)]
enum Shape {
    Circle(f64),                    // radius
    Rectangle(f64, f64),            // width, height
    Triangle { base: f64, height: f64 }, // named fields
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(r) => 2.0 * std::f64::consts::PI * r,
            Shape::Rectangle(w, h) => 2.0 * (w + h),
            Shape::Triangle { base, height } => {
                let hyp = (base * base + height * height).sqrt();
                base + height + hyp
            }
        }
    }
}
```

**Option<T> — Rust's null replacement:**

```rust
fn find_user(id: u32) -> Option<String> {
    if id == 1 { Some(String::from("Alice")) } else { None }
}

fn main() {
    // Pattern matching
    match find_user(1) {
        Some(name) => println!("Found: {}", name),
        None => println!("Not found"),
    }

    // if let — concise single-pattern match
    if let Some(name) = find_user(2) {
        println!("Found: {}", name);
    }

    // Chaining with ? in functions returning Option
    let name = find_user(1)?; // returns None if None

    // Combinators
    let upper = find_user(1).map(|n| n.to_uppercase());
    let default = find_user(99).unwrap_or_else(|| String::from("Guest"));
    let len = find_user(1).map(|n| n.len()).unwrap_or(0);
}
```

---

### 3.3 Pattern Matching — Advanced

```rust
fn classify(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1 | 2 | 3 => "small",
        4..=9 => "medium",
        10..=99 => "large",
        n if n < 0 => "negative",
        _ => "huge",
    }
}

// Destructuring in match
#[derive(Debug)]
struct Point { x: i32, y: i32 }

fn describe_point(p: Point) -> &'static str {
    match p {
        Point { x: 0, y: 0 } => "origin",
        Point { x, y: 0 } => "on x-axis",
        Point { x: 0, y } => "on y-axis",
        Point { x, y } if x == y => "on diagonal",
        _ => "somewhere else",
    }
}

// @ bindings — capture and test
fn check_range(n: u32) {
    match n {
        val @ 1..=10 => println!("{} is between 1 and 10", val),
        val @ 11..=20 => println!("{} is between 11 and 20", val),
        _ => println!("out of range"),
    }
}

// Nested destructuring
let ((a, b), Point { x, y }) = ((1, 2), Point { x: 3, y: 4 });
```

---

### Mini Project 3: Shape Calculator

Build a shape calculator that:
- Defines a `Shape` enum with at least 5 variants
- Implements `area()`, `perimeter()`, and `describe()` methods
- Parses shape descriptions from strings (e.g., `"circle 5.0"`)
- Uses pattern matching extensively
- Handles invalid input with `Option` or `Result`

---

## Section 4: Smart Pointers & Interior Mutability

### 4.1 Box<T> — Heap Allocation

```rust
// Box<T> allocates data on the heap
fn main() {
    let b = Box::new(5); // 5 is on the heap
    println!("b = {}", b); // auto-derefs

    // Primary use case: recursive types (can't have infinite size on stack)
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let list = List::Cons(1,
        Box::new(List::Cons(2,
            Box::new(List::Cons(3,
                Box::new(List::Nil))))));
    println!("{:?}", list);
}
```

---

### 4.2 Rc<T> — Reference Counted (Single-threaded)

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("shared data"));
    let b = Rc::clone(&a); // increments reference count, doesn't clone data
    let c = Rc::clone(&a);

    println!("Reference count: {}", Rc::strong_count(&a)); // 3
    println!("a={}, b={}, c={}", a, b, c);

    drop(b);
    println!("After drop: {}", Rc::strong_count(&a)); // 2
} // a and c dropped here, count reaches 0, data freed
```

> ⚠️ `Rc<T>` is **not thread-safe**. Use `Arc<T>` for multi-threaded scenarios.

---

### 4.3 Arc<T> — Atomic Reference Counted (Multi-threaded)

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);

    let handles: Vec<_> = (0..3).map(|i| {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            println!("Thread {}: sum = {}", i, data.iter().sum::<i32>());
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
}
```

---

### 4.4 RefCell<T> — Interior Mutability

`RefCell<T>` allows mutation through an immutable reference, with borrow checking at **runtime** instead of compile time:

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(Node {
            value,
            children: RefCell::new(vec![]),
        })
    }

    fn add_child(&self, child: Rc<Node>) {
        self.children.borrow_mut().push(child); // runtime borrow check
    }
}

fn main() {
    let root = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);

    root.add_child(Rc::clone(&child1));
    root.add_child(Rc::clone(&child2));

    println!("Root has {} children", root.children.borrow().len());
}
```

> ⚠️ `RefCell<T>` panics at runtime if you violate borrowing rules. Use it sparingly and only when you're sure the logic is correct.

---

### 4.5 Mutex<T> and RwLock<T>

```rust
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn mutex_example() {
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..10).map(|_| {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        })
    }).collect();
    for h in handles { h.join().unwrap(); }
    println!("Counter: {}", *counter.lock().unwrap()); // 10
}

fn rwlock_example() {
    // Multiple readers OR one writer
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    // Multiple concurrent readers
    let r1 = data.read().unwrap();
    let r2 = data.read().unwrap();
    println!("r1={:?}, r2={:?}", *r1, *r2);
    drop(r1); drop(r2);

    // Exclusive writer
    data.write().unwrap().push(4);
}
```

---

### 4.6 Weak<T> — Breaking Reference Cycles

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,   // Weak reference — doesn't prevent drop
    children: RefCell<Vec<Rc<Node>>>,
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

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // When branch is dropped, leaf.parent.upgrade() returns None
}
```

**When to use which:**

| Type | Thread-safe | Mutable | Use case |
|------|-------------|---------|----------|
| `Box<T>` | N/A | Via `&mut` | Heap allocation, recursive types |
| `Rc<T>` | ❌ | No | Shared ownership, single-threaded |
| `Arc<T>` | ✅ | No | Shared ownership, multi-threaded |
| `RefCell<T>` | ❌ | Runtime check | Interior mutability, single-threaded |
| `Mutex<T>` | ✅ | Lock-based | Shared mutable state, multi-threaded |
| `RwLock<T>` | ✅ | Lock-based | Many readers, few writers |
| `Weak<T>` | N/A | No | Break reference cycles |

---

### Mini Project 4: Graph Data Structure

Build a directed graph using `Rc<RefCell<Node>>`:
- Add/remove nodes and edges
- BFS and DFS traversal
- Detect cycles
- Print adjacency list

---

## Section 5: Error Handling — Novice to Pro

### 5.1 Result<T, E> Basics

```rust
use std::fs;
use std::num::ParseIntError;

fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse::<i32>()
}

fn main() {
    match parse_number("42") {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    // Combinators
    let doubled = parse_number("21").map(|n| n * 2);
    let default = parse_number("bad").unwrap_or(0);
    let chained = parse_number("10")
        .and_then(|n| if n > 0 { Ok(n) } else { Err("10".parse::<i32>().unwrap_err()) });
}
```

---

### 5.2 The ? Operator

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    let content = fs::read_to_string("username.txt")?; // returns Err if fails
    Ok(content.trim().to_string())
}

// Chaining ? operators
fn process_file(path: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let numbers: Result<Vec<i32>, _> = content
        .lines()
        .map(|line| line.trim().parse::<i32>())
        .collect();
    Ok(numbers?)
}
```

---

### 5.3 Custom Error Types

```rust
use std::fmt;
use std::num::ParseIntError;

// Simple custom error
#[derive(Debug)]
enum AppError {
    ParseError(ParseIntError),
    ValidationError(String),
    IoError(std::io::Error),
    NotFound { resource: String, id: u64 },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ParseError(e) => write!(f, "Parse error: {}", e),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::IoError(e) => write!(f, "IO error: {}", e),
            AppError::NotFound { resource, id } => {
                write!(f, "{} with id {} not found", resource, id)
            }
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::ParseError(e) => Some(e),
            AppError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

// From implementations enable ? operator conversion
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
    let n: i32 = s.parse()?; // ParseIntError auto-converts to AppError via From
    if n < 0 {
        return Err(AppError::ValidationError(format!("{} is negative", n)));
    }
    Ok(n as u32)
}
```

---

### 5.4 thiserror — Ergonomic Error Types

```toml
# Cargo.toml
[dependencies]
thiserror = "1"
```

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Query failed: {query}")]
    QueryFailed { query: String },

    #[error("Record not found: id={id}")]
    NotFound { id: u64 },

    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Serialization error")]
    Serde(#[from] serde_json::Error),
}
```

---

### 5.5 anyhow — Application-Level Error Handling

```toml
[dependencies]
anyhow = "1"
```

```rust
use anyhow::{Context, Result, bail, ensure, anyhow};

fn load_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path))?;

    let config: Config = serde_json::from_str(&content)
        .context("Failed to parse config JSON")?;

    ensure!(config.port > 1024, "Port must be > 1024, got {}", config.port);

    if config.workers == 0 {
        bail!("Worker count cannot be zero");
    }

    Ok(config)
}

// anyhow::Result is great for application code
// thiserror is better for library code (gives callers typed errors)
```

**Rule of thumb:**
- **Library crates:** Use `thiserror` — give callers typed errors they can match on
- **Application/binary crates:** Use `anyhow` — ergonomic, context-rich error messages

---

### Mini Project 5: Config File Parser

Build a configuration file parser that:
- Reads a custom `.conf` format (`key = value` per line)
- Returns typed values (strings, integers, booleans)
- Uses a custom error type with `thiserror`
- Provides helpful error messages with line numbers
- Handles missing required keys and type mismatches

---

## Section 6: Collections & Iterators — Deep Dive

### 6.1 Vec<T>

```rust
fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3]; // macro shorthand

    // Adding elements
    v.push(1);
    v.extend([2, 3, 4]);
    v.insert(0, 0); // insert at index

    // Accessing
    let third = &v[2];          // panics if out of bounds
    let third = v.get(2);       // returns Option<&i32>

    // Removing
    v.pop();                    // removes last
    v.remove(0);                // removes at index, shifts elements
    v.retain(|&x| x % 2 == 0); // keep only even numbers

    // Sorting
    v2.sort();
    v2.sort_by(|a, b| b.cmp(a));          // reverse sort
    v2.sort_by_key(|&x| std::cmp::Reverse(x));

    // Deduplication (must be sorted first)
    v2.dedup();

    // Slicing
    let slice: &[i32] = &v2[1..3];

    // Capacity management
    let mut v3: Vec<i32> = Vec::with_capacity(100); // pre-allocate
    println!("len={}, cap={}", v3.len(), v3.capacity());
}
```

---

### 6.2 HashMap<K, V>

```rust
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    // Inserting
    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"), 85);

    // Entry API — insert only if not present
    scores.entry(String::from("Alice")).or_insert(50); // Alice already exists, no change
    scores.entry(String::from("Carol")).or_insert(75); // Carol inserted

    // Modify existing or insert default
    let count = scores.entry(String::from("Dave")).or_insert(0);
    *count += 10;

    // Accessing
    let alice_score = scores.get("Alice");           // Option<&i32>
    let alice_score = scores["Alice"];               // panics if missing

    // Iterating
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // Collecting from iterators
    let word_count: HashMap<&str, usize> = "hello world hello rust"
        .split_whitespace()
        .fold(HashMap::new(), |mut map, word| {
            *map.entry(word).or_insert(0) += 1;
            map
        });
}
```

---

### 6.3 Other Collections

```rust
use std::collections::{HashSet, BTreeMap, BTreeSet, VecDeque, BinaryHeap};

// HashSet — unique values, O(1) lookup
let mut set: HashSet<i32> = HashSet::new();
set.insert(1); set.insert(2); set.insert(1); // duplicate ignored
let a: HashSet<_> = [1,2,3].iter().collect();
let b: HashSet<_> = [2,3,4].iter().collect();
let intersection: HashSet<_> = a.intersection(&b).collect();
let union: HashSet<_> = a.union(&b).collect();

// BTreeMap — sorted keys, O(log n) operations
let mut btree: BTreeMap<String, i32> = BTreeMap::new();
btree.insert("banana".to_string(), 3);
btree.insert("apple".to_string(), 1);
// Iterates in sorted key order: apple, banana

// VecDeque — double-ended queue
let mut deque: VecDeque<i32> = VecDeque::new();
deque.push_front(1);
deque.push_back(2);
deque.pop_front();

// BinaryHeap — max-heap priority queue
let mut heap: BinaryHeap<i32> = BinaryHeap::new();
heap.push(3); heap.push(1); heap.push(4);
println!("{}", heap.pop().unwrap()); // 4 (max)
```

---

### 6.4 Iterators — Deep Dive

The `Iterator` trait is the backbone of idiomatic Rust:

```rust
// Implementing Iterator from scratch
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self { Counter { count: 0, max } }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    // All these methods come for free from implementing Iterator
    let sum: u32 = Counter::new(5).sum();
    let doubled: Vec<u32> = Counter::new(5).map(|x| x * 2).collect();
    let evens: Vec<u32> = Counter::new(10).filter(|x| x % 2 == 0).collect();

    // Zip two iterators
    let pairs: Vec<(u32, u32)> = Counter::new(3)
        .zip(Counter::new(3).skip(1))
        .collect();

    // flat_map — map then flatten
    let words = vec!["hello world", "foo bar"];
    let all_words: Vec<&str> = words.iter()
        .flat_map(|s| s.split_whitespace())
        .collect();

    // fold — general reduction
    let factorial: u64 = (1..=10).fold(1, |acc, x| acc * x);

    // scan — like fold but yields intermediate values
    let running_sum: Vec<i32> = (1..=5)
        .scan(0, |acc, x| { *acc += x; Some(*acc) })
        .collect(); // [1, 3, 6, 10, 15]

    // take_while / skip_while
    let before_five: Vec<i32> = (1..10).take_while(|&x| x < 5).collect();

    // chain — concatenate iterators
    let combined: Vec<i32> = (1..=3).chain(7..=9).collect();

    // enumerate — add index
    for (i, val) in ["a", "b", "c"].iter().enumerate() {
        println!("{}: {}", i, val);
    }

    // peekable — look ahead without consuming
    let mut iter = [1, 2, 3].iter().peekable();
    if iter.peek() == Some(&&1) {
        println!("starts with 1");
    }

    // Lazy evaluation — nothing runs until consumed
    let lazy = (0..).filter(|x| x % 2 == 0).map(|x| x * x).take(5);
    // No computation yet!
    let result: Vec<i32> = lazy.collect(); // NOW it runs
}
```

**IntoIterator vs Iterator:**

```rust
// IntoIterator — can be converted into an iterator
// for loops use IntoIterator automatically

let v = vec![1, 2, 3];
for x in &v { }        // borrows: v.iter()
for x in &mut v { }   // mutable borrow: v.iter_mut()
for x in v { }        // consumes: v.into_iter()
```

---

### Mini Project 6: Data Pipeline

Build a data processing pipeline using iterators:
- Read a CSV-like string of student records (name, grade, score)
- Filter students above a threshold score
- Group by grade
- Calculate statistics per grade (mean, median, top scorer)
- Output a formatted report
- Use **zero cloning** — work with references and iterators throughout

---

## Section 7: Traits & Generics — Deep Dive

### 7.1 Defining and Implementing Traits

```rust
trait Animal {
    // Required method
    fn name(&self) -> &str;
    fn sound(&self) -> &str;

    // Default method
    fn describe(&self) -> String {
        format!("The {} goes {}", self.name(), self.sound())
    }

    // Associated constant
    const CATEGORY: &'static str = "animal";
}

struct Dog { name: String }
struct Cat { name: String }

impl Animal for Dog {
    fn name(&self) -> &str { &self.name }
    fn sound(&self) -> &str { "woof" }
}

impl Animal for Cat {
    fn name(&self) -> &str { &self.name }
    fn sound(&self) -> &str { "meow" }
    // Override default
    fn describe(&self) -> String {
        format!("{} says {} and ignores you", self.name(), self.sound())
    }
}
```

---

### 7.2 Trait Bounds

```rust
// impl Trait syntax (simpler)
fn print_animal(animal: &impl Animal) {
    println!("{}", animal.describe());
}

// where clause (cleaner for complex bounds)
fn largest<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}

// Multiple bounds
fn print_and_clone<T>(item: &T) -> T
where
    T: std::fmt::Display + Clone,
{
    println!("{}", item);
    item.clone()
}

// Conditional implementation
use std::fmt::Display;

struct Wrapper<T>(T);

impl<T: Display> Wrapper<T> {
    fn show(&self) {
        println!("{}", self.0);
    }
}
```

---

### 7.3 impl Trait vs dyn Trait

```rust
// impl Trait — static dispatch (monomorphization, zero-cost)
fn make_sound_static(animal: &impl Animal) {
    println!("{}", animal.sound());
}

// dyn Trait — dynamic dispatch (vtable, runtime cost)
fn make_sound_dynamic(animal: &dyn Animal) {
    println!("{}", animal.sound());
}

// When to use dyn Trait:
// - Heterogeneous collections
// - Return type depends on runtime condition
// - Reduce binary size (avoid monomorphization explosion)

fn get_animals() -> Vec<Box<dyn Animal>> {
    vec![
        Box::new(Dog { name: "Rex".to_string() }),
        Box::new(Cat { name: "Whiskers".to_string() }),
    ]
}

// Object safety rules — a trait is object-safe if:
// - No methods return Self
// - No generic methods
// - No associated functions without self

// This is NOT object-safe:
trait NotObjectSafe {
    fn clone_self(&self) -> Self; // returns Self
    fn generic<T>(&self, t: T);  // generic method
}
```

---

### 7.4 Associated Types vs Generic Parameters

```rust
// Associated type — one implementation per type
trait Container {
    type Item;
    fn first(&self) -> Option<&Self::Item>;
    fn last(&self) -> Option<&Self::Item>;
}

// Generic parameter — multiple implementations per type
trait Converter<T> {
    fn convert(&self) -> T;
}

struct Celsius(f64);

impl Converter<f64> for Celsius {
    fn convert(&self) -> f64 { self.0 }
}

impl Converter<String> for Celsius {
    fn convert(&self) -> String { format!("{}°C", self.0) }
}

// Use associated types when there's a natural "one output type"
// Use generic parameters when you need multiple implementations
```

---

### 7.5 Const Generics

```rust
// Arrays with compile-time size
fn sum_array<const N: usize>(arr: [i32; N]) -> i32 {
    arr.iter().sum()
}

// Generic struct with const parameter
struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize>
    Matrix<T, ROWS, COLS>
{
    fn new() -> Self {
        Matrix { data: [[T::default(); COLS]; ROWS] }
    }
}

fn main() {
    let m: Matrix<f64, 3, 3> = Matrix::new();
    let result = sum_array([1, 2, 3, 4, 5]);
}
```

---

### 7.6 Important Standard Traits

```rust
use std::fmt;
use std::ops::{Add, Mul, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Self { Vec2 { x, y } }
    fn magnitude(&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f64) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 { Vec2::new(-self.x, -self.y) }
}

// Default trait
impl Default for Vec2 {
    fn default() -> Self { Vec2::new(0.0, 0.0) }
}

// From/Into
impl From<(f64, f64)> for Vec2 {
    fn from((x, y): (f64, f64)) -> Self { Vec2::new(x, y) }
}

fn main() {
    let a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(3.0, 4.0);
    let c = a + b;
    let d = c * 2.0;
    println!("{} + {} = {}", a, b, c);

    let v: Vec2 = (1.0, 2.0).into(); // uses From impl
}
```

---

### Mini Project 7: Generic Matrix Library

Build a generic matrix library:
- `Matrix<T, const ROWS: usize, const COLS: usize>`
- Implement `Add`, `Mul` (matrix multiplication), `Display`
- Transpose operation
- Determinant for 2x2 and 3x3 (where T: Float)
- Use trait bounds to restrict operations to numeric types

---

## Section 8: Closures & Functional Programming

### 8.1 Closures

```rust
fn main() {
    // Closure syntax
    let add = |a, b| a + b;
    let square = |x: i32| -> i32 { x * x };
    let greet = |name| format!("Hello, {}!", name);

    // Closures capture their environment
    let offset = 10;
    let add_offset = |x| x + offset; // captures offset by reference

    // Move closure — takes ownership of captured variables
    let data = vec![1, 2, 3];
    let contains_two = move || data.contains(&2); // data moved into closure
    // println!("{:?}", data); // ERROR: data moved

    // Fn, FnMut, FnOnce
    fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
    fn apply_mut<F: FnMut() -> i32>(mut f: F) -> i32 { f() }
    fn apply_once<F: FnOnce() -> String>(f: F) -> String { f() }

    let result = apply(|x| x * 2, 5);

    let mut count = 0;
    let mut counter = || { count += 1; count }; // FnMut — mutates captured var
    println!("{}", counter()); // 1
    println!("{}", counter()); // 2

    let s = String::from("hello");
    let consume = move || s.to_uppercase(); // FnOnce — consumes captured var
    println!("{}", consume()); // can only call once
}
```

**Fn trait hierarchy:**
- `FnOnce` — can be called once (consumes captures)
- `FnMut` — can be called multiple times, mutates captures
- `Fn` — can be called multiple times, doesn't mutate captures

Every `Fn` implements `FnMut`, every `FnMut` implements `FnOnce`.

---

### 8.2 Returning Closures

```rust
// Must use Box<dyn Fn> or impl Fn
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn make_adder_boxed(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}

// When you need to return different closure types at runtime
fn make_transform(double: bool) -> Box<dyn Fn(i32) -> i32> {
    if double {
        Box::new(|x| x * 2)
    } else {
        Box::new(|x| x + 1)
    }
}
```

---

### 8.3 Functional Patterns

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Pipeline style
    let result: Vec<String> = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .filter(|&x| x > 10)
        .map(|x| format!("{}^2", (x as f64).sqrt() as i32))
        .collect();

    // Option as a functor
    let maybe_name: Option<String> = Some("alice".to_string());
    let upper = maybe_name.map(|s| s.to_uppercase());
    let length = upper.as_ref().map(|s| s.len());

    // Result chaining
    let result = "42"
        .parse::<i32>()
        .map(|n| n * 2)
        .map_err(|e| format!("Parse failed: {}", e));

    // Collecting Results
    let strings = vec!["1", "2", "three", "4"];
    let numbers: Result<Vec<i32>, _> = strings.iter()
        .map(|s| s.parse::<i32>())
        .collect(); // Err if any parse fails

    // Partition
    let (evens, odds): (Vec<i32>, Vec<i32>) = (1..=10).partition(|x| x % 2 == 0);

    // Unzip
    let pairs = vec![(1, 'a'), (2, 'b'), (3, 'c')];
    let (nums, chars): (Vec<i32>, Vec<char>) = pairs.into_iter().unzip();
}
```

---

### Mini Project 8: Functional Data Transformer

Build a composable data transformation pipeline:
- Define a `Pipeline<T>` struct that chains transformations
- Support `map`, `filter`, `reduce`, `flat_map` operations
- Make it lazy (don't evaluate until `.run()` is called)
- Demonstrate with a real dataset (e.g., log line parsing)

---

## Section 9: Modules, Crates & Cargo — Deep Dive

### 9.1 Module System

```rust
// src/lib.rs
pub mod geometry {
    pub mod shapes {
        #[derive(Debug)]
        pub struct Circle {
            pub radius: f64,
        }

        impl Circle {
            pub fn new(radius: f64) -> Self { Circle { radius } }
            pub fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
        }

        // Private helper — not accessible outside this module
        fn validate_radius(r: f64) -> bool { r > 0.0 }
    }

    pub use shapes::Circle; // re-export for convenience
}

// Using the module
use crate::geometry::shapes::Circle;
use crate::geometry::Circle; // via re-export

// Relative paths
mod parent {
    pub mod child {
        pub fn hello() { println!("hello from child"); }
        pub fn call_sibling() {
            super::sibling::world(); // go up one level
        }
    }
    mod sibling {
        pub fn world() { println!("world from sibling"); }
    }
}
```

---

### 9.2 Cargo Workspaces

For multi-crate projects:

```toml
# workspace/Cargo.toml
[workspace]
members = [
    "core",
    "cli",
    "server",
    "common",
]
resolver = "2"
```

```bash
workspace/
├── Cargo.toml          # workspace root
├── core/
│   ├── Cargo.toml
│   └── src/lib.rs
├── cli/
│   ├── Cargo.toml      # depends on core
│   └── src/main.rs
├── server/
│   ├── Cargo.toml      # depends on core
│   └── src/main.rs
└── common/
    ├── Cargo.toml
    └── src/lib.rs
```

```toml
# cli/Cargo.toml
[dependencies]
core = { path = "../core" }
common = { path = "../common" }
```

```bash
cargo build                    # build all
cargo build -p cli             # build specific package
cargo test --workspace         # test all
cargo run -p cli               # run specific binary
```

---

### 9.3 Feature Flags

```toml
# Cargo.toml
[features]
default = ["json"]
json = ["serde", "serde_json"]
yaml = ["serde", "serde_yaml"]
full = ["json", "yaml", "async"]
async = ["tokio"]

[dependencies]
serde = { version = "1", features = ["derive"], optional = true }
serde_json = { version = "1", optional = true }
serde_yaml = { version = "0.9", optional = true }
tokio = { version = "1", features = ["full"], optional = true }
```

```rust
// Conditional compilation
#[cfg(feature = "json")]
pub mod json_support {
    pub fn serialize<T: serde::Serialize>(val: &T) -> String {
        serde_json::to_string(val).unwrap()
    }
}

#[cfg(feature = "async")]
pub async fn async_operation() { /* ... */ }
```

```bash
cargo build --features yaml
cargo build --all-features
cargo build --no-default-features --features json
```

---

### 9.4 Build Scripts (build.rs)

```rust
// build.rs — runs before compilation
fn main() {
    // Tell Cargo to re-run if these files change
    println!("cargo:rerun-if-changed=src/ffi.h");
    println!("cargo:rerun-if-changed=build.rs");

    // Set environment variables accessible in code
    println!("cargo:rustc-env=BUILD_TIME={}", chrono::Utc::now());

    // Link a C library
    println!("cargo:rustc-link-lib=mylib");
    println!("cargo:rustc-link-search=native=/usr/local/lib");

    // Generate code from a schema
    // prost_build::compile_protos(&["proto/service.proto"], &["proto/"]).unwrap();
}
```

---

### 9.5 Essential Cargo Tools

```bash
# Install tools
cargo install cargo-expand      # expand macros
cargo install cargo-audit       # security audit
cargo install cargo-deny        # license/dependency policy
cargo install cargo-watch       # auto-rebuild on change
cargo install cargo-flamegraph  # profiling
cargo install cargo-criterion   # benchmarking
cargo install cargo-nextest     # faster test runner

# Usage
cargo expand                    # show macro-expanded code
cargo audit                     # check for vulnerabilities
cargo watch -x run              # auto-run on file change
cargo watch -x test             # auto-test on file change
cargo deny check                # check licenses and bans
```

---

### Mini Project 9: Multi-Crate Library

Create a workspace with:
- `math-core` — generic math operations (no dependencies)
- `math-stats` — statistics built on `math-core`
- `math-cli` — CLI tool using both
- Feature flags for optional output formats (JSON, CSV)
- Proper `pub` API design with re-exports

---

## Section 10: Working with Files

### 10.1 Basic File I/O

```rust
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Write, Read};
use std::path::Path;

fn main() -> io::Result<()> {
    // Write entire file at once
    fs::write("hello.txt", "Hello, file!")?;

    // Read entire file at once
    let content = fs::read_to_string("hello.txt")?;
    println!("{}", content);

    // Read as bytes
    let bytes = fs::read("hello.txt")?;

    // Buffered reading — efficient for large files
    let file = File::open("hello.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    // Buffered writing — efficient for many small writes
    let file = File::create("output.txt")?;
    let mut writer = BufWriter::new(file);
    for i in 0..1000 {
        writeln!(writer, "Line {}", i)?;
    }
    writer.flush()?; // ensure all buffered data is written

    // Append to file
    use std::fs::OpenOptions;
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")?;
    writeln!(file, "New log entry")?;

    Ok(())
}
```

---

### 10.2 Paths and Directories

```rust
use std::path::{Path, PathBuf};
use std::fs;

fn main() -> std::io::Result<()> {
    // PathBuf — owned, mutable path
    let mut path = PathBuf::from("/home/user");
    path.push("documents");
    path.push("file.txt");
    println!("{}", path.display()); // /home/user/documents/file.txt

    // Path operations
    let p = Path::new("/home/user/file.txt");
    println!("parent: {:?}", p.parent());
    println!("file name: {:?}", p.file_name());
    println!("extension: {:?}", p.extension());
    println!("stem: {:?}", p.file_stem());
    println!("exists: {}", p.exists());
    println!("is file: {}", p.is_file());
    println!("is dir: {}", p.is_dir());

    // Create directories
    fs::create_dir("new_dir")?;
    fs::create_dir_all("a/b/c/d")?; // creates all intermediate dirs

    // List directory contents
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        println!("{:?} — {:?}", entry.file_name(), entry.file_type()?);
    }

    // Recursive directory walk (using walkdir crate)
    // for entry in walkdir::WalkDir::new(".") {
    //     println!("{}", entry?.path().display());
    // }

    // Copy, rename, remove
    fs::copy("source.txt", "dest.txt")?;
    fs::rename("old.txt", "new.txt")?;
    fs::remove_file("file.txt")?;
    fs::remove_dir_all("dir")?;

    Ok(())
}
```

---

### 10.3 Temporary Files

```toml
[dependencies]
tempfile = "3"
```

```rust
use tempfile::{NamedTempFile, TempDir, tempfile};
use std::io::{Write, Read, Seek, SeekFrom};

fn main() -> std::io::Result<()> {
    // Anonymous temp file — deleted when handle dropped
    let mut anon = tempfile()?;
    writeln!(anon, "temporary data")?;
    anon.seek(SeekFrom::Start(0))?;
    let mut content = String::new();
    anon.read_to_string(&mut content)?;

    // Named temp file — has a path, deleted when handle dropped
    let mut named = NamedTempFile::new()?;
    writeln!(named, "named temp data")?;
    println!("Temp file at: {}", named.path().display());

    // Persist a named temp file (don't delete it)
    let path = named.into_temp_path();
    path.persist("/tmp/permanent_file.txt")?;

    // Temp directory
    let dir = TempDir::new()?;
    let file_path = dir.path().join("data.txt");
    std::fs::write(&file_path, "data")?;
    println!("Temp dir: {}", dir.path().display());
    // dir dropped here — entire directory deleted

    Ok(())
}
```

---

### 10.4 Atomic File Writing

Atomic writes prevent data corruption if the process crashes mid-write:

```rust
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write, BufWriter};
use std::path::Path;

/// Atomically write data to a file using temp file + rename pattern
fn atomic_write(path: &Path, data: &[u8]) -> io::Result<()> {
    // Write to a temp file in the same directory (same filesystem = atomic rename)
    let dir = path.parent().unwrap_or(Path::new("."));
    let mut temp = tempfile::NamedTempFile::new_in(dir)?;

    temp.write_all(data)?;

    // fsync — ensure data is flushed to disk before rename
    temp.as_file().sync_all()?;

    // Atomic rename — either old or new file exists, never partial
    temp.persist(path).map_err(|e| e.error)?;

    Ok(())
}

/// Atomic write with a writer callback
fn atomic_write_with<F>(path: &Path, writer_fn: F) -> io::Result<()>
where
    F: FnOnce(&mut BufWriter<&File>) -> io::Result<()>,
{
    let dir = path.parent().unwrap_or(Path::new("."));
    let temp = tempfile::NamedTempFile::new_in(dir)?;
    let mut buf_writer = BufWriter::new(temp.as_file());

    writer_fn(&mut buf_writer)?;
    buf_writer.flush()?;
    temp.as_file().sync_all()?;
    temp.persist(path).map_err(|e| e.error)?;

    Ok(())
}

fn main() -> io::Result<()> {
    let path = Path::new("config.json");
    atomic_write(path, b"{\"version\": 1}")?;

    atomic_write_with(Path::new("data.txt"), |w| {
        for i in 0..100 {
            writeln!(w, "line {}", i)?;
        }
        Ok(())
    })?;

    Ok(())
}
```

---

### 10.5 File Metadata and Permissions

```rust
use std::fs;
use std::os::unix::fs::PermissionsExt; // Unix only

fn main() -> std::io::Result<()> {
    let metadata = fs::metadata("file.txt")?;

    println!("size: {} bytes", metadata.len());
    println!("is file: {}", metadata.is_file());
    println!("is dir: {}", metadata.is_dir());
    println!("readonly: {}", metadata.permissions().readonly());

    // Timestamps
    let modified = metadata.modified()?;
    let created = metadata.created()?;
    println!("modified: {:?}", modified);

    // Unix permissions
    #[cfg(unix)]
    {
        let mode = metadata.permissions().mode();
        println!("permissions: {:o}", mode); // e.g., 644
        fs::set_permissions("file.txt", fs::Permissions::from_mode(0o644))?;
    }

    // Set read-only
    let mut perms = metadata.permissions();
    perms.set_readonly(true);
    fs::set_permissions("file.txt", perms)?;

    Ok(())
}
```

---

### 10.6 Async File I/O

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Simple async read/write
    fs::write("async_file.txt", "async content").await?;
    let content = fs::read_to_string("async_file.txt").await?;

    // Async buffered reading
    let file = File::open("large_file.txt").await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        println!("{}", line);
    }

    // Concurrent file reads
    let (r1, r2, r3) = tokio::join!(
        fs::read_to_string("file1.txt"),
        fs::read_to_string("file2.txt"),
        fs::read_to_string("file3.txt"),
    );

    // Async atomic write
    async fn atomic_write_async(path: &std::path::Path, data: &[u8]) -> std::io::Result<()> {
        let dir = path.parent().unwrap_or(std::path::Path::new("."));
        let temp = tempfile::NamedTempFile::new_in(dir)?;
        let mut file = File::from_std(temp.reopen()?);
        file.write_all(data).await?;
        file.sync_all().await?;
        temp.persist(path).map_err(|e| e.error)?;
        Ok(())
    }

    atomic_write_async(std::path::Path::new("output.json"), b"{}").await?;

    Ok(())
}
```

---

### 10.7 File Watching

```toml
[dependencies]
notify = "6"
```

```rust
use notify::{Watcher, RecursiveMode, Result, Event, recommended_watcher};
use std::sync::mpsc;
use std::time::Duration;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = recommended_watcher(tx)?;
    watcher.watch(std::path::Path::new("."), RecursiveMode::Recursive)?;

    println!("Watching for file changes...");
    for res in rx {
        match res {
            Ok(event) => println!("Event: {:?}", event),
            Err(e) => println!("Watch error: {:?}", e),
        }
    }

    Ok(())
}
```

---

### Mini Project 10: Atomic Config Manager

Build a configuration manager that:
- Reads/writes JSON config atomically
- Watches for external config changes and reloads
- Supports async read/write
- Validates config schema on load
- Backs up previous config before overwriting

---

## Section 11: Testing, Linting & Formatting

This section covers Rust's built-in testing framework (analogous to pytest in Python), plus the essential tooling every professional Rust developer uses daily.

### 11.1 Unit Tests

```rust
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 { a + b }
pub fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

#[cfg(test)]  // only compiled during testing
mod tests {
    use super::*; // import everything from parent module

    #[test]
    fn test_add_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, -1), -2);
    }

    #[test]
    fn test_divide_normal() {
        assert_eq!(divide(10.0, 2.0), Some(5.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10.0, 0.0), None);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_panics() {
        let v = vec![1, 2, 3];
        let _ = v[10]; // should panic
    }

    #[test]
    #[ignore] // skip this test by default
    fn expensive_test() {
        // run with: cargo test -- --ignored
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
```

```bash
cargo test                          # run all tests
cargo test test_add                 # run tests matching "test_add"
cargo test -- --nocapture           # show println! output
cargo test -- --test-threads=1      # run tests sequentially
cargo test -- --ignored             # run ignored tests
cargo test -- --list                # list all tests
```

---

### 11.2 Integration Tests

Integration tests live in the `tests/` directory and test your public API:

```
src/
  lib.rs
tests/
  integration_test.rs
  common/
    mod.rs          # shared test helpers
```

```rust
// tests/common/mod.rs
pub fn setup() -> Config {
    Config::default()
}

// tests/integration_test.rs
mod common;

use my_crate::{Client, Config};

#[test]
fn test_client_connects() {
    let config = common::setup();
    let client = Client::new(config);
    assert!(client.is_connected());
}

#[test]
fn test_full_workflow() {
    let config = common::setup();
    let mut client = Client::new(config);
    client.send("hello").unwrap();
    let response = client.receive().unwrap();
    assert_eq!(response, "HELLO");
}
```

---

### 11.3 Doc Tests

Doc tests are code examples in documentation comments that are automatically tested:

```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// use my_crate::add;
///
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// Negative numbers also work:
///
/// ```
/// # use my_crate::add;
/// assert_eq!(add(-1, 1), 0);
/// ```
///
/// This example shows a panic (not run as test):
/// ```no_run
/// // This would panic at runtime
/// ```
///
/// This example is compiled but not run:
/// ```compile_fail
/// let x: i32 = "not a number"; // compile error
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

```bash
cargo test --doc    # run only doc tests
```

---

### 11.4 Test Organization & Best Practices

```rust
// Arrange-Act-Assert pattern
#[test]
fn test_user_creation() {
    // Arrange
    let name = "Alice";
    let email = "alice@example.com";

    // Act
    let user = User::new(name, email);

    // Assert
    assert_eq!(user.name(), name);
    assert_eq!(user.email(), email);
    assert!(user.is_active());
}

// Custom assertion messages
#[test]
fn test_with_message() {
    let result = compute_something();
    assert!(
        result > 0,
        "Expected positive result, got {}",
        result
    );
    assert_eq!(
        result, 42,
        "Expected 42 but got {}. Input was: {:?}",
        result, input
    );
}

// Testing with fixtures using setup/teardown
struct TestFixture {
    temp_dir: tempfile::TempDir,
    db_path: std::path::PathBuf,
}

impl TestFixture {
    fn new() -> Self {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        TestFixture { temp_dir, db_path }
    }
}

#[test]
fn test_database_operations() {
    let fixture = TestFixture::new();
    let db = Database::open(&fixture.db_path).unwrap();
    // test...
} // fixture dropped here — temp dir cleaned up automatically
```

---

### 11.5 Async Tests

```toml
[dev-dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_function() {
        let result = async_fetch_data().await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn test_concurrent_operations() {
        let handles: Vec<_> = (0..10)
            .map(|i| tokio::spawn(async move { process(i).await }))
            .collect();
        for h in handles {
            h.await.unwrap().unwrap();
        }
    }
}
```

---

### 11.6 Mocking

Rust doesn't have a built-in mock framework, but `mockall` is the standard:

```toml
[dev-dependencies]
mockall = "0.12"
```

```rust
use mockall::{automock, predicate::*};

#[automock]
trait Database {
    fn get_user(&self, id: u64) -> Option<User>;
    fn save_user(&mut self, user: &User) -> Result<(), DbError>;
}

fn process_user(db: &dyn Database, id: u64) -> Result<String, String> {
    match db.get_user(id) {
        Some(user) => Ok(format!("Hello, {}!", user.name)),
        None => Err(format!("User {} not found", id)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_existing_user() {
        let mut mock_db = MockDatabase::new();
        mock_db
            .expect_get_user()
            .with(eq(1))
            .times(1)
            .returning(|_| Some(User { name: "Alice".to_string(), id: 1 }));

        let result = process_user(&mock_db, 1);
        assert_eq!(result, Ok("Hello, Alice!".to_string()));
    }

    #[test]
    fn test_process_missing_user() {
        let mut mock_db = MockDatabase::new();
        mock_db
            .expect_get_user()
            .with(eq(99))
            .times(1)
            .returning(|_| None);

        let result = process_user(&mock_db, 99);
        assert!(result.is_err());
    }
}
```

---

### 11.7 Property-Based Testing

```toml
[dev-dependencies]
proptest = "1"
```

```rust
use proptest::prelude::*;

fn reverse<T: Clone>(v: &[T]) -> Vec<T> {
    let mut result = v.to_vec();
    result.reverse();
    result
}

proptest! {
    // Test with randomly generated inputs
    #[test]
    fn test_reverse_length(v: Vec<i32>) {
        prop_assert_eq!(reverse(&v).len(), v.len());
    }

    #[test]
    fn test_reverse_twice_is_identity(v: Vec<i32>) {
        prop_assert_eq!(reverse(&reverse(&v)), v);
    }

    #[test]
    fn test_parse_roundtrip(n in 0i32..1000) {
        let s = n.to_string();
        let parsed: i32 = s.parse().unwrap();
        prop_assert_eq!(parsed, n);
    }

    #[test]
    fn test_sort_is_idempotent(mut v: Vec<i32>) {
        v.sort();
        let sorted_once = v.clone();
        v.sort();
        prop_assert_eq!(v, sorted_once);
    }
}
```

---

### 11.8 Benchmarking with Criterion

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
harness = false
```

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci 20", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
}

fn bench_fibonacci_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");
    for size in [10u64, 15, 20, 25].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| fibonacci(black_box(size)))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibonacci, bench_fibonacci_sizes);
criterion_main!(benches);
```

```bash
cargo bench                         # run all benchmarks
cargo bench -- fibonacci            # run matching benchmarks
# Results saved to target/criterion/
```

---

### 11.9 cargo-nextest — Faster Test Runner

```bash
cargo install cargo-nextest

cargo nextest run                   # run all tests (faster than cargo test)
cargo nextest run --test-threads 8  # parallel
cargo nextest run -p my_crate       # specific package
cargo nextest list                  # list tests
```

---

### 11.10 Linting with Clippy

Clippy is Rust's official linter with 700+ checks:

```bash
# Run clippy
cargo clippy

# Treat warnings as errors (use in CI)
cargo clippy -- -D warnings

# Specific lint categories
cargo clippy -- -W clippy::pedantic
cargo clippy -- -W clippy::nursery
cargo clippy -- -A clippy::too_many_arguments  # allow specific lint

# Fix automatically
cargo clippy --fix
```

**Configuring Clippy in code:**

```rust
// Allow a specific lint for a function
#[allow(clippy::too_many_arguments)]
fn complex_function(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) {}

// Deny a lint (make it an error)
#[deny(clippy::unwrap_used)]
fn safe_function() {
    // Using .unwrap() here would be a compile error
}

// Warn for the whole crate
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
```

**clippy.toml — project-wide configuration:**

```toml
# clippy.toml
msrv = "1.70.0"
cognitive-complexity-threshold = 30
too-many-arguments-threshold = 7
```

**Common Clippy lints to know:**

```rust
// clippy::unwrap_used — prefer ? or expect()
let x = some_option.unwrap();           // warned
let x = some_option.expect("msg");      // better
let x = some_option?;                   // best (in Result context)

// clippy::clone_on_ref_ptr — unnecessary clone
let a = Arc::new(1);
let b = a.clone();   // OK — this is intentional

// clippy::needless_pass_by_value
fn bad(s: String) -> usize { s.len() }  // should be &str
fn good(s: &str) -> usize { s.len() }

// clippy::map_unwrap_or
let x = opt.map(|v| v + 1).unwrap_or(0); // warned
let x = opt.map_or(0, |v| v + 1);        // better

// clippy::redundant_closure
let doubled: Vec<_> = v.iter().map(|x| x * 2).collect(); // OK
let doubled: Vec<_> = v.iter().map(double).collect();     // if double fn exists
```

---

### 11.11 Formatting with rustfmt

```bash
# Format all code
cargo fmt

# Check formatting without changing files (use in CI)
cargo fmt -- --check

# Format a single file
rustfmt src/main.rs
```

**rustfmt.toml — project-wide formatting config:**

```toml
# rustfmt.toml
edition = "2021"
max_width = 100
tab_spaces = 4
use_small_heuristics = "Default"
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
wrap_comments = true
format_code_in_doc_comments = true
```

**Opt out of formatting for specific blocks:**

```rust
#[rustfmt::skip]
fn hand_formatted() {
    let matrix = [
        1, 0, 0,
        0, 1, 0,
        0, 0, 1,
    ];
}
```

---

### 11.12 CI/CD Pipeline for Rust

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt

      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2

      - name: Check formatting
        run: cargo fmt -- --check

      - name: Clippy
        run: cargo clippy -- -D warnings

      - name: Tests
        run: cargo test --all-features

      - name: Security audit
        run: |
          cargo install cargo-audit
          cargo audit

  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin
      - name: Coverage
        run: cargo tarpaulin --out Xml
      - name: Upload coverage
        uses: codecov/codecov-action@v3
```

---

### Mini Project 11: Well-Tested Library

Take Mini Project 7 (Matrix Library) and add:
- 100% unit test coverage
- Integration tests
- Doc tests for every public function
- Property-based tests for mathematical properties (commutativity, associativity)
- Benchmarks comparing naive vs optimized implementations
- Clippy clean (zero warnings)
- rustfmt formatted
- CI workflow

---

## Section 12: Concurrency & Async Programming — Deep Dive

### 12.1 Threads

```rust
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    // Spawn a thread
    let handle = thread::spawn(|| {
        println!("Hello from thread!");
        42 // return value
    });

    let result = handle.join().unwrap(); // wait and get return value
    println!("Thread returned: {}", result);

    // Move closure — transfer ownership to thread
    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("data: {:?}", data);
    });
    handle.join().unwrap();

    // Thread pool pattern
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..10).map(|_| {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        })
    }).collect();
    for h in handles { h.join().unwrap(); }
    println!("Final count: {}", *counter.lock().unwrap());
}
```

---

### 12.2 Channels

```rust
use std::sync::mpsc; // multiple producer, single consumer
use std::thread;

fn main() {
    // Basic channel
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("hello").unwrap();
        tx.send("world").unwrap();
    });

    for msg in rx { // rx acts as an iterator
        println!("{}", msg);
    }

    // Multiple producers
    let (tx, rx) = mpsc::channel::<String>();
    let handles: Vec<_> = (0..5).map(|i| {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(format!("message from thread {}", i)).unwrap();
        })
    }).collect();
    drop(tx); // drop original sender so rx knows when all senders are done

    for msg in rx { println!("{}", msg); }
    for h in handles { h.join().unwrap(); }

    // Synchronous channel (bounded)
    let (tx, rx) = mpsc::sync_channel::<i32>(10); // buffer of 10
    // tx.send() blocks when buffer is full
}
```

---

### 12.3 How async/await Works Internally

Understanding the internals makes you a better async programmer:

```rust
// A Future is a state machine
// This is conceptually what the compiler generates:

enum MyFuture {
    Start,
    WaitingForRead { file: File },
    Done,
}

impl Future for MyFuture {
    type Output = String;

    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<String> {
        loop {
            match self {
                MyFuture::Start => {
                    let file = File::open("data.txt")?;
                    *self = MyFuture::WaitingForRead { file };
                }
                MyFuture::WaitingForRead { file } => {
                    match file.poll_read(cx, &mut buf) {
                        Poll::Ready(Ok(n)) => {
                            *self = MyFuture::Done;
                            return Poll::Ready(String::from_utf8(buf).unwrap());
                        }
                        Poll::Pending => return Poll::Pending, // yield to executor
                    }
                }
                MyFuture::Done => panic!("polled after completion"),
            }
        }
    }
}
```

**Key insight:** `async fn` functions are syntactic sugar for functions that return `impl Future`. The `await` keyword is syntactic sugar for polling a future and yielding if it's not ready.

---

### 12.4 Tokio — The Async Runtime

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
use tokio::time::{sleep, Duration};
use tokio::sync::{mpsc, Mutex, RwLock, Semaphore};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Spawn async tasks
    let handle = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        42
    });
    let result = handle.await.unwrap();

    // join! — run concurrently, wait for all
    let (r1, r2) = tokio::join!(
        async { sleep(Duration::from_millis(100)).await; 1 },
        async { sleep(Duration::from_millis(100)).await; 2 },
    ); // takes ~100ms, not 200ms

    // select! — run concurrently, take first to complete
    tokio::select! {
        result = slow_operation() => println!("slow: {}", result),
        result = fast_operation() => println!("fast: {}", result),
        _ = sleep(Duration::from_secs(5)) => println!("timeout!"),
    }

    // Async channels
    let (tx, mut rx) = mpsc::channel::<String>(32);
    tokio::spawn(async move {
        tx.send("hello".to_string()).await.unwrap();
    });
    while let Some(msg) = rx.recv().await {
        println!("{}", msg);
    }

    // Semaphore — limit concurrency
    let semaphore = Arc::new(Semaphore::new(10)); // max 10 concurrent
    let handles: Vec<_> = (0..100).map(|i| {
        let sem = Arc::clone(&semaphore);
        tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            // only 10 of these run at a time
            process(i).await;
        })
    }).collect();
    for h in handles { h.await.unwrap(); }
}
```

---

### 12.5 Pin and Unpin

```rust
use std::pin::Pin;
use std::marker::PhantomPinned;

// Self-referential struct — cannot be moved after creation
struct SelfReferential {
    data: String,
    ptr: *const String, // points to data above
    _pin: PhantomPinned,
}

impl SelfReferential {
    fn new(data: String) -> Pin<Box<Self>> {
        let mut boxed = Box::pin(SelfReferential {
            data,
            ptr: std::ptr::null(),
            _pin: PhantomPinned,
        });
        // Safe because we're pinned — won't move
        let ptr = &boxed.data as *const String;
        unsafe { boxed.as_mut().get_unchecked_mut().ptr = ptr; }
        boxed
    }
}

// Why Pin exists:
// async functions create self-referential state machines
// Pin<Box<dyn Future>> ensures the future won't move in memory
// which would invalidate internal pointers
```

---

### 12.6 Async Traits

```toml
[dependencies]
async-trait = "0.1"
```

```rust
use async_trait::async_trait;

// Async methods in traits require async-trait crate (pre-Rust 1.75)
#[async_trait]
trait DataStore {
    async fn get(&self, key: &str) -> Option<String>;
    async fn set(&mut self, key: String, value: String) -> Result<(), StoreError>;
    async fn delete(&mut self, key: &str) -> bool;
}

struct RedisStore { /* ... */ }

#[async_trait]
impl DataStore for RedisStore {
    async fn get(&self, key: &str) -> Option<String> {
        // actual Redis call
        todo!()
    }
    async fn set(&mut self, key: String, value: String) -> Result<(), StoreError> {
        todo!()
    }
    async fn delete(&mut self, key: &str) -> bool {
        todo!()
    }
}

// Rust 1.75+ supports async fn in traits natively (with some limitations)
trait ModernStore {
    async fn get(&self, key: &str) -> Option<String>;
}
```

---

### 12.7 Cancellation Safety

```rust
use tokio::select;
use tokio::sync::mpsc;

// NOT cancellation-safe — data can be lost
async fn bad_receive(rx: &mut mpsc::Receiver<String>) -> String {
    let msg = rx.recv().await.unwrap(); // if cancelled here, message is lost
    msg
}

// Cancellation-safe — use a wrapper that handles partial state
async fn process_with_timeout(
    rx: &mut mpsc::Receiver<String>,
    timeout: std::time::Duration,
) -> Option<String> {
    select! {
        msg = rx.recv() => msg,
        _ = tokio::time::sleep(timeout) => None,
    }
}
```

---

### 12.8 Rayon — Data Parallelism

```toml
[dependencies]
rayon = "1"
```

```rust
use rayon::prelude::*;

fn main() {
    let data: Vec<i64> = (0..1_000_000).collect();

    // Parallel iterator — just change .iter() to .par_iter()
    let sum: i64 = data.par_iter().sum();
    let doubled: Vec<i64> = data.par_iter().map(|&x| x * 2).collect();
    let evens: Vec<&i64> = data.par_iter().filter(|&&x| x % 2 == 0).collect();

    // Parallel sort
    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    v.par_sort();

    // Custom parallel work
    let results: Vec<_> = (0..100).into_par_iter().map(|i| {
        expensive_computation(i) // runs on thread pool
    }).collect();
}
```

---

### Mini Project 12: Async Web Scraper

Build a concurrent web scraper:
- Fetch multiple URLs concurrently with `tokio` + `reqwest`
- Limit concurrency with a `Semaphore`
- Parse HTML with `scraper` crate
- Store results atomically to disk
- Handle timeouts and retries with exponential backoff
- Report progress with a channel

---

## Section 13: Macros — Declarative & Procedural

### 13.1 Declarative Macros (macro_rules!)

```rust
// Simple macro
macro_rules! say_hello {
    () => { println!("Hello!"); };
    ($name:expr) => { println!("Hello, {}!", $name); };
}

say_hello!();           // Hello!
say_hello!("Alice");    // Hello, Alice!

// Variadic macro
macro_rules! vec_of_strings {
    ($($x:expr),*) => {
        vec![$($x.to_string()),*]
    };
}

let v = vec_of_strings!["hello", "world", "rust"];

// HashMap macro
macro_rules! hashmap {
    ($($key:expr => $val:expr),* $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(map.insert($key, $val);)*
        map
    }};
}

let m = hashmap! {
    "one" => 1,
    "two" => 2,
    "three" => 3,
};

// Recursive macro
macro_rules! count {
    () => { 0 };
    ($head:tt $($tail:tt)*) => { 1 + count!($($tail)*) };
}

let n = count!(a b c d e); // 5
```

**Macro fragment specifiers:**
| Specifier | Matches |
|-----------|---------|
| `expr` | expressions |
| `stmt` | statements |
| `ty` | types |
| `ident` | identifiers |
| `path` | paths |
| `tt` | token tree (anything) |
| `literal` | literal values |
| `block` | `{ ... }` blocks |

---

### 13.2 Procedural Macros

Procedural macros are functions that take a `TokenStream` and return a `TokenStream`:

```toml
# proc-macro crate Cargo.toml
[lib]
proc-macro = true

[dependencies]
syn = { version = "2", features = ["full"] }
quote = "1"
proc-macro2 = "1"
```

**Custom Derive macro:**

```rust
// In proc-macro crate: src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    gen.into()
}

// Usage in another crate:
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro(); // Hello, Macro! My name is Pancakes!
}
```

**Attribute macro:**

```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let method_and_path = attr.to_string();
    let input = parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input.sig.ident;

    let gen = quote! {
        #input

        // Register the route
        inventory::submit! {
            Route {
                method_path: #method_and_path,
                handler: #fn_name,
            }
        }
    };
    gen.into()
}

// Usage:
#[route("GET /users")]
async fn get_users() -> Response { /* ... */ }
```

---

### 13.3 When to Use Macros

**Use macros when:**
- You need variadic arguments (variable number of args)
- You need to generate repetitive code based on types
- You need to implement a DSL (domain-specific language)
- You need compile-time code generation

**Prefer functions when:**
- A function can do the job — macros are harder to debug
- You need clear error messages
- You need IDE support (macros have limited autocomplete)

---

### Mini Project 13: Builder Derive Macro

Create a procedural macro `#[derive(Builder)]` that:
- Generates a builder struct for any struct
- Each field gets a setter method
- `build()` returns `Result<T, BuildError>` for required fields
- Optional fields use `Option<T>`

```rust
#[derive(Builder)]
struct Config {
    host: String,
    port: u16,
    timeout: Option<Duration>,
}

// Generated:
let config = ConfigBuilder::new()
    .host("localhost")
    .port(8080)
    .timeout(Duration::from_secs(30))
    .build()?;
```

---

## Section 14: Unsafe Rust & FFI

### 14.1 What Unsafe Allows

`unsafe` unlocks five capabilities not available in safe Rust:
1. Dereference raw pointers
2. Call unsafe functions or methods
3. Access or modify mutable static variables
4. Implement unsafe traits
5. Access fields of `union`s

```rust
fn main() {
    // Raw pointers
    let x = 5;
    let r = &x as *const i32;  // raw pointer (safe to create)
    let y = 10;
    let r_mut = &mut y as *mut i32; // wait, y isn't mut — this is a problem

    unsafe {
        println!("r = {}", *r);  // dereference (unsafe)
    }

    // Unsafe function
    unsafe fn dangerous() {
        println!("I'm dangerous!");
    }
    unsafe { dangerous(); }

    // Safe abstraction over unsafe code
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();
        assert!(mid <= len);
        unsafe {
            (
                std::slice::from_raw_parts_mut(ptr, mid),
                std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
}
```

---

### 14.2 FFI — Calling C from Rust

```rust
// Link to C's math library
extern "C" {
    fn abs(input: i32) -> i32;
    fn sqrt(x: f64) -> f64;
}

fn main() {
    unsafe {
        println!("abs(-3) = {}", abs(-3));
        println!("sqrt(4.0) = {}", sqrt(4.0));
    }
}
```

**Calling Rust from C:**

```rust
// Expose Rust functions to C
#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

// C-compatible struct
#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[no_mangle]
pub extern "C" fn create_point(x: f64, y: f64) -> Point {
    Point { x, y }
}
```

---

### 14.3 Using bindgen

```toml
[build-dependencies]
bindgen = "0.69"
```

```rust
// build.rs
fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).unwrap();
}
```

---

### 14.4 Common Unsafe Patterns

```rust
// Transmute — reinterpret bytes (extremely dangerous)
let x: u32 = 0x41424344;
let bytes: [u8; 4] = unsafe { std::mem::transmute(x) };

// Extend lifetime (almost always wrong — use with extreme care)
fn extend_lifetime<'a, 'b, T>(r: &'a T) -> &'b T {
    unsafe { &*(r as *const T) }
}

// Global mutable state
static mut COUNTER: u32 = 0;
unsafe {
    COUNTER += 1;
    println!("{}", COUNTER);
}
// Better: use atomic types
use std::sync::atomic::{AtomicU32, Ordering};
static ATOMIC_COUNTER: AtomicU32 = AtomicU32::new(0);
ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst);
```

---

### Mini Project 14: Safe Wrapper for a C Library

Wrap a C library (e.g., `zlib` for compression) with a safe Rust API:
- Use `bindgen` to generate raw bindings
- Create a safe wrapper that handles errors and memory
- Write tests that verify the wrapper works correctly
- Ensure no memory leaks (use `valgrind` or address sanitizer)

---

## Section 15: Design Patterns in Rust

### 15.1 Builder Pattern

```rust
#[derive(Debug)]
struct HttpRequest {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<Vec<u8>>,
    timeout: std::time::Duration,
}

struct HttpRequestBuilder {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<Vec<u8>>,
    timeout: std::time::Duration,
}

impl HttpRequestBuilder {
    fn new(url: impl Into<String>) -> Self {
        HttpRequestBuilder {
            url: url.into(),
            method: "GET".to_string(),
            headers: vec![],
            body: None,
            timeout: std::time::Duration::from_secs(30),
        }
    }

    fn method(mut self, method: impl Into<String>) -> Self {
        self.method = method.into();
        self
    }

    fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = timeout;
        self
    }

    fn build(self) -> HttpRequest {
        HttpRequest {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: self.body,
            timeout: self.timeout,
        }
    }
}

let request = HttpRequestBuilder::new("https://api.example.com/users")
    .method("POST")
    .header("Content-Type", "application/json")
    .header("Authorization", "Bearer token123")
    .body(b"{\"name\": \"Alice\"}".to_vec())
    .timeout(std::time::Duration::from_secs(10))
    .build();
```

---

### 15.2 Typestate Pattern — Compile-Time State Machines

```rust
// States as zero-size types
struct Locked;
struct Unlocked;

struct Safe<State> {
    contents: String,
    _state: std::marker::PhantomData<State>,
}

impl Safe<Locked> {
    fn new(contents: String) -> Self {
        Safe { contents, _state: std::marker::PhantomData }
    }

    fn unlock(self, password: &str) -> Result<Safe<Unlocked>, Safe<Locked>> {
        if password == "secret" {
            Ok(Safe { contents: self.contents, _state: std::marker::PhantomData })
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
        Safe { contents: self.contents, _state: std::marker::PhantomData }
    }
}

fn main() {
    let safe = Safe::<Locked>::new("diamonds".to_string());
    // safe.get_contents(); // COMPILE ERROR: method doesn't exist on Safe<Locked>

    let unlocked = safe.unlock("secret").unwrap();
    println!("{}", unlocked.get_contents()); // OK

    let locked_again = unlocked.lock();
    // locked_again.get_contents(); // COMPILE ERROR again
}
```

---

### 15.3 Newtype Pattern

```rust
// Prevent mixing up semantically different values of the same type
struct Meters(f64);
struct Kilograms(f64);
struct UserId(u64);
struct OrderId(u64);

impl Meters {
    fn value(&self) -> f64 { self.0 }
}

impl std::fmt::Display for Meters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m", self.0)
    }
}

fn calculate_bmi(weight: Kilograms, height: Meters) -> f64 {
    weight.0 / (height.0 * height.0)
}

// calculate_bmi(height, weight); // COMPILE ERROR: wrong order
calculate_bmi(Kilograms(70.0), Meters(1.75)); // correct
```

---

### 15.4 Strategy Pattern

```rust
trait SortStrategy {
    fn sort(&self, data: &mut Vec<i32>);
    fn name(&self) -> &str;
}

struct BubbleSort;
struct QuickSort;
struct MergeSort;

impl SortStrategy for BubbleSort {
    fn sort(&self, data: &mut Vec<i32>) {
        let n = data.len();
        for i in 0..n {
            for j in 0..n-i-1 {
                if data[j] > data[j+1] { data.swap(j, j+1); }
            }
        }
    }
    fn name(&self) -> &str { "Bubble Sort" }
}

impl SortStrategy for QuickSort {
    fn sort(&self, data: &mut Vec<i32>) { data.sort_unstable(); }
    fn name(&self) -> &str { "Quick Sort" }
}

struct Sorter {
    strategy: Box<dyn SortStrategy>,
}

impl Sorter {
    fn new(strategy: Box<dyn SortStrategy>) -> Self { Sorter { strategy } }
    fn sort(&self, data: &mut Vec<i32>) {
        println!("Using {}", self.strategy.name());
        self.strategy.sort(data);
    }
}
```

---

### 15.5 Command Pattern

```rust
trait Command {
    fn execute(&mut self);
    fn undo(&mut self);
}

struct TextEditor {
    content: String,
    history: Vec<Box<dyn Command>>,
}

struct InsertCommand {
    text: String,
    position: usize,
    editor_content: *mut String, // simplified
}

impl TextEditor {
    fn execute(&mut self, mut cmd: Box<dyn Command>) {
        cmd.execute();
        self.history.push(cmd);
    }

    fn undo(&mut self) {
        if let Some(mut cmd) = self.history.pop() {
            cmd.undo();
        }
    }
}
```

---

### Mini Project 15: Compile-Time HTTP Client Builder

Build a type-safe HTTP client using the typestate pattern:
- States: `NoUrl` → `HasUrl` → `HasMethod` → `Ready`
- Can only call `.send()` when in `Ready` state
- Compile-time enforcement of required fields
- Support for headers, body, auth

---

## Section 16: Performance, Profiling & Optimization

### 16.1 Understanding Zero-Cost Abstractions

```rust
// These compile to the same machine code:

// High-level iterator
let sum: i32 = (0..1000).filter(|x| x % 2 == 0).map(|x| x * x).sum();

// Manual loop
let mut sum = 0i32;
for i in 0..1000 {
    if i % 2 == 0 { sum += i * i; }
}

// Verify with: cargo build --release && objdump -d target/release/myapp
```

---

### 16.2 Avoiding Unnecessary Allocations

```rust
// BAD: allocates a new String
fn bad_process(s: &str) -> String {
    let upper = s.to_uppercase(); // allocation
    upper.trim().to_string()      // another allocation
}

// GOOD: use Cow<str> to avoid allocation when possible
use std::borrow::Cow;

fn good_process(s: &str) -> Cow<str> {
    if s.chars().all(|c| c.is_uppercase()) {
        Cow::Borrowed(s) // no allocation
    } else {
        Cow::Owned(s.to_uppercase()) // allocate only when needed
    }
}

// BAD: collect into Vec just to iterate
fn bad_sum(data: &[i32]) -> i32 {
    let doubled: Vec<i32> = data.iter().map(|x| x * 2).collect(); // unnecessary alloc
    doubled.iter().sum()
}

// GOOD: chain iterators
fn good_sum(data: &[i32]) -> i32 {
    data.iter().map(|x| x * 2).sum() // no intermediate allocation
}

// Pre-allocate when size is known
let mut v = Vec::with_capacity(1000);
for i in 0..1000 { v.push(i); } // no reallocations
```

---

### 16.3 Profiling with cargo-flamegraph

```bash
# Install
cargo install cargo-flamegraph

# Profile (Linux — requires perf)
cargo flamegraph --bin myapp -- arg1 arg2

# macOS
cargo flamegraph --bin myapp -- arg1 arg2
# Opens flamegraph.svg in browser
```

---

### 16.4 Heap Profiling

```bash
# Using heaptrack (Linux)
heaptrack cargo run --release
heaptrack_gui heaptrack.myapp.*.gz

# Using DHAT (via valgrind)
valgrind --tool=dhat --dhat-out-file=dhat.out ./target/release/myapp
dhat-viewer dhat.out
```

---

### 16.5 SIMD Basics

```rust
// Portable SIMD (nightly)
#![feature(portable_simd)]
use std::simd::f32x8;

fn sum_simd(data: &[f32]) -> f32 {
    let chunks = data.chunks_exact(8);
    let remainder = chunks.remainder();

    let sum_vec = chunks
        .map(f32x8::from_slice)
        .fold(f32x8::splat(0.0), |acc, x| acc + x);

    sum_vec.reduce_sum() + remainder.iter().sum::<f32>()
}

// Stable SIMD via std::arch (platform-specific)
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
```

---

### 16.6 Cache-Friendly Data Structures

```rust
// BAD: Array of Structs (AoS) — poor cache locality for field access
struct Particle { x: f32, y: f32, z: f32, mass: f32, charge: f32 }
let particles: Vec<Particle> = vec![/* ... */];

// GOOD: Struct of Arrays (SoA) — excellent cache locality
struct Particles {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<f32>,
    mass: Vec<f32>,
    charge: Vec<f32>,
}

// When updating all x positions, SoA loads only x data into cache
impl Particles {
    fn update_positions(&mut self, dt: f32) {
        for (x, &vx) in self.x.iter_mut().zip(self.vx.iter()) {
            *x += vx * dt; // sequential memory access — cache friendly
        }
    }
}
```

---

### Mini Project 16: Performance Comparison

Take a computationally intensive algorithm (e.g., matrix multiplication):
- Implement naive version
- Implement cache-friendly version (SoA, blocking)
- Implement SIMD version
- Benchmark all three with Criterion
- Profile with flamegraph
- Document the performance differences

---

## Section 17: Networking & Web Development

### 17.1 TCP/UDP with Tokio

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// TCP Echo Server
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on :8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection: {}", addr);
        tokio::spawn(handle_connection(socket));
    }
}

async fn handle_connection(mut socket: TcpStream) {
    let mut buf = vec![0u8; 1024];
    loop {
        match socket.read(&mut buf).await {
            Ok(0) => break, // connection closed
            Ok(n) => {
                if socket.write_all(&buf[..n]).await.is_err() { break; }
            }
            Err(_) => break,
        }
    }
}
```

---

### 17.2 HTTP Client with reqwest

```toml
[dependencies]
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Serialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("my-app/1.0")
        .build()?;

    // GET with JSON deserialization
    let user: User = client
        .get("https://jsonplaceholder.typicode.com/users/1")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    println!("{:?}", user);

    // POST with JSON body
    let new_user = CreateUser {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    let response = client
        .post("https://jsonplaceholder.typicode.com/users")
        .json(&new_user)
        .send()
        .await?;
    println!("Status: {}", response.status());

    Ok(())
}
```

---

### 17.3 REST API with Axum

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }
```

```rust
use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

type Db = Arc<RwLock<HashMap<u64, User>>>;

async fn get_user(
    Path(id): Path<u64>,
    State(db): State<Db>,
) -> impl IntoResponse {
    match db.read().unwrap().get(&id) {
        Some(user) => (StatusCode::OK, Json(user.clone())).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn create_user(
    State(db): State<Db>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    let mut db = db.write().unwrap();
    db.insert(user.id, user.clone());
    (StatusCode::CREATED, Json(user))
}

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(RwLock::new(HashMap::new()));

    let app = Router::new()
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on :3000");
    axum::serve(listener, app).await.unwrap();
}
```

---

### Mini Project 17: REST API with Persistence

Build a REST API for a resource of your choice:
- Full CRUD operations
- Persistent storage (JSON file with atomic writes)
- Input validation with custom error responses
- Middleware: logging, CORS, rate limiting
- Integration tests using `reqwest` against a test server

---

## Section 18: Data Serialization with Serde

### 18.1 Serde Basics

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
```

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
    tags: Vec<String>,
    database: DatabaseConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
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
        },
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&config)?;
    println!("{}", json);

    // Deserialize from JSON
    let config2: Config = serde_json::from_str(&json)?;

    // Serialize to TOML
    let toml_str = toml::to_string(&config)?;

    Ok(())
}
```

---

### 18.2 Serde Attributes

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]  // serialize as camelCase
struct ApiResponse {
    user_id: u64,           // serialized as "userId"
    first_name: String,     // serialized as "firstName"

    #[serde(skip_serializing_if = "Option::is_none")]
    middle_name: Option<String>, // omitted if None

    #[serde(default)]
    is_active: bool,        // uses Default if missing during deserialization

    #[serde(rename = "pwd")]
    password_hash: String,  // serialized as "pwd"

    #[serde(skip)]
    internal_data: String,  // never serialized or deserialized

    #[serde(flatten)]
    metadata: Metadata,     // inline fields from Metadata
}

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    created_at: String,
    updated_at: String,
}

// Enum serialization
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
enum Event {
    UserCreated { id: u64, name: String },
    UserDeleted { id: u64 },
    OrderPlaced { order_id: u64, amount: f64 },
}
```

---

### 18.3 Custom Serializers/Deserializers

```rust
use serde::{Deserializer, Serializer, de::Visitor};
use std::fmt;

// Custom serializer for Duration as seconds
mod duration_secs {
    use super::*;
    use std::time::Duration;

    pub fn serialize<S: Serializer>(d: &Duration, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u64(d.as_secs())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
        let secs = u64::deserialize(d)?;
        Ok(Duration::from_secs(secs))
    }
}

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    #[serde(with = "duration_secs")]
    timeout: std::time::Duration,
}
```

---

### Mini Project 18: Multi-Format Config System

Build a configuration system that:
- Supports JSON, TOML, and YAML formats
- Auto-detects format from file extension
- Merges configs from multiple files (later files override earlier)
- Validates required fields
- Supports environment variable overrides
- Custom serialization for special types (Duration, PathBuf, etc.)

---

## Capstone Project: Distributed Log Aggregation & Analysis Engine

### Overview

Build **`logforge`** — a production-grade distributed log aggregation and analysis engine. This project integrates every concept from the roadmap into a real-world system that could be deployed in a production environment.

**What it does:**
- Collects logs from multiple sources (files, TCP, UDP, stdin)
- Parses structured and unstructured log formats
- Filters, transforms, and enriches log events in real-time
- Stores logs with efficient indexing for fast querying
- Exposes a REST API for querying and streaming logs
- Provides a real-time dashboard via WebSocket
- Supports distributed operation (multiple nodes)

---

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        logforge                              │
│                                                             │
│  ┌──────────┐    ┌──────────────┐    ┌──────────────────┐  │
│  │ Ingestion│───▶│   Pipeline   │───▶│    Storage       │  │
│  │  Layer   │    │  (Transform) │    │  (Index + Write) │  │
│  └──────────┘    └──────────────┘    └──────────────────┘  │
│       │                                       │             │
│  ┌────▼─────┐                        ┌────────▼──────────┐  │
│  │  Sources │                        │   Query Engine    │  │
│  │ - Files  │                        │   REST API        │  │
│  │ - TCP    │                        │   WebSocket       │  │
│  │ - UDP    │                        └───────────────────┘  │
│  │ - stdin  │                                               │
│  └──────────┘                                               │
└─────────────────────────────────────────────────────────────┘
```

---

### Workspace Structure

```
logforge/
├── Cargo.toml                  # workspace
├── crates/
│   ├── logforge-core/          # shared types, traits
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── event.rs        # LogEvent type
│   │       ├── filter.rs       # Filter trait
│   │       └── error.rs        # error types (thiserror)
│   ├── logforge-parser/        # log format parsers
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── json.rs
│   │       ├── syslog.rs
│   │       ├── nginx.rs
│   │       └── regex_parser.rs
│   ├── logforge-pipeline/      # transformation pipeline
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── stage.rs        # PipelineStage trait
│   │       ├── filter.rs
│   │       ├── transform.rs
│   │       └── enrich.rs
│   ├── logforge-storage/       # storage engine
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── index.rs        # inverted index
│   │       ├── segment.rs      # log segment files
│   │       └── query.rs        # query engine
│   ├── logforge-api/           # REST + WebSocket API
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── routes.rs
│   │       ├── ws.rs           # WebSocket handler
│   │       └── auth.rs
│   └── logforge-cli/           # binary entry point
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
├── tests/                      # integration tests
│   ├── ingestion_test.rs
│   ├── pipeline_test.rs
│   └── api_test.rs
└── benches/
    └── throughput.rs
```

---

### Core Types (logforge-core)

```rust
// crates/logforge-core/src/event.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEvent {
    pub id: uuid::Uuid,
    pub timestamp: SystemTime,
    pub level: LogLevel,
    pub source: String,
    pub message: String,
    pub fields: HashMap<String, FieldValue>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl LogEvent {
    pub fn new(level: LogLevel, source: impl Into<String>, message: impl Into<String>) -> Self {
        LogEvent {
            id: uuid::Uuid::new_v4(),
            timestamp: SystemTime::now(),
            level,
            source: source.into(),
            message: message.into(),
            fields: HashMap::new(),
            tags: vec![],
        }
    }

    pub fn with_field(mut self, key: impl Into<String>, value: FieldValue) -> Self {
        self.fields.insert(key.into(), value);
        self
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
}
```

---

### Pipeline Stage Trait

```rust
// crates/logforge-pipeline/src/stage.rs
use async_trait::async_trait;
use logforge_core::{LogEvent, error::PipelineError};

#[async_trait]
pub trait PipelineStage: Send + Sync {
    async fn process(&self, event: LogEvent) -> Result<Option<LogEvent>, PipelineError>;
    fn name(&self) -> &str;
}

pub struct Pipeline {
    stages: Vec<Box<dyn PipelineStage>>,
    metrics: Arc<PipelineMetrics>,
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline {
            stages: vec![],
            metrics: Arc::new(PipelineMetrics::default()),
        }
    }

    pub fn add_stage(mut self, stage: Box<dyn PipelineStage>) -> Self {
        self.stages.push(stage);
        self
    }

    pub async fn process(&self, event: LogEvent) -> Option<LogEvent> {
        let mut current = Some(event);
        for stage in &self.stages {
            match current {
                None => return None,
                Some(e) => {
                    self.metrics.record_stage_input(stage.name());
                    match stage.process(e).await {
                        Ok(result) => current = result,
                        Err(err) => {
                            tracing::error!("Pipeline stage '{}' error: {}", stage.name(), err);
                            self.metrics.record_stage_error(stage.name());
                            return None;
                        }
                    }
                }
            }
        }
        current
    }
}

// Example stages
pub struct LevelFilter { min_level: LogLevel }
pub struct FieldEnricher { enrichments: HashMap<String, FieldValue> }
pub struct RegexRedactor { pattern: regex::Regex, replacement: String }
pub struct GeoIpEnricher { db: Arc<maxminddb::Reader<Vec<u8>>> }
pub struct RateLimiter { limiter: Arc<governor::RateLimiter<...>> }
```

---

### Storage Engine

```rust
// crates/logforge-storage/src/segment.rs
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};

pub struct Segment {
    path: PathBuf,
    writer: BufWriter<File>,
    index: SegmentIndex,
    size: u64,
    event_count: u64,
}

impl Segment {
    pub async fn create(dir: &Path, id: u64) -> std::io::Result<Self> {
        let path = dir.join(format!("{:020}.log", id));
        let file = File::create(&path).await?;
        Ok(Segment {
            path,
            writer: BufWriter::new(file),
            index: SegmentIndex::new(),
            size: 0,
            event_count: 0,
        })
    }

    pub async fn append(&mut self, event: &LogEvent) -> std::io::Result<u64> {
        let offset = self.size;
        let bytes = serde_json::to_vec(event).unwrap();
        let len = bytes.len() as u32;

        // Write length-prefixed record
        self.writer.write_all(&len.to_le_bytes()).await?;
        self.writer.write_all(&bytes).await?;

        self.index.add(event, offset);
        self.size += 4 + bytes.len() as u64;
        self.event_count += 1;

        Ok(offset)
    }

    pub async fn flush_and_sync(&mut self) -> std::io::Result<()> {
        self.writer.flush().await?;
        self.writer.get_ref().sync_all().await?;
        Ok(())
    }
}

// Inverted index for fast querying
pub struct SegmentIndex {
    level_index: HashMap<LogLevel, Vec<u64>>,    // level -> offsets
    source_index: HashMap<String, Vec<u64>>,     // source -> offsets
    tag_index: HashMap<String, Vec<u64>>,        // tag -> offsets
    time_index: BTreeMap<SystemTime, u64>,       // time -> offset (sorted)
}
```

---

### Query Engine

```rust
// crates/logforge-storage/src/query.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Query {
    pub from: Option<SystemTime>,
    pub to: Option<SystemTime>,
    pub level: Option<LogLevel>,
    pub source: Option<String>,
    pub tags: Vec<String>,
    pub message_contains: Option<String>,
    pub fields: HashMap<String, FieldValue>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub order: QueryOrder,
}

#[derive(Debug, Deserialize, Default)]
pub enum QueryOrder {
    #[default]
    Newest,
    Oldest,
}

pub struct QueryResult {
    pub events: Vec<LogEvent>,
    pub total: usize,
    pub took_ms: u64,
}

pub struct QueryEngine {
    segments: Arc<RwLock<Vec<Arc<Segment>>>>,
}

impl QueryEngine {
    pub async fn query(&self, q: &Query) -> QueryResult {
        let start = std::time::Instant::now();
        let segments = self.segments.read().await;

        // Use index to find candidate offsets
        let candidates = self.find_candidates(&segments, q).await;

        // Filter candidates
        let mut results: Vec<LogEvent> = candidates
            .into_iter()
            .filter(|e| self.matches(e, q))
            .collect();

        // Sort and paginate
        match q.order {
            QueryOrder::Newest => results.sort_by(|a, b| b.timestamp.cmp(&a.timestamp)),
            QueryOrder::Oldest => results.sort_by(|a, b| a.timestamp.cmp(&b.timestamp)),
        }

        let total = results.len();
        let offset = q.offset.unwrap_or(0);
        let limit = q.limit.unwrap_or(100);
        let events = results.into_iter().skip(offset).take(limit).collect();

        QueryResult { events, total, took_ms: start.elapsed().as_millis() as u64 }
    }
}
```

---

### REST API (Axum)

```rust
// crates/logforge-api/src/routes.rs
use axum::{
    extract::{Query as AxumQuery, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/logs", get(query_logs).post(ingest_log))
        .route("/api/v1/logs/stream", get(stream_logs_ws))
        .route("/api/v1/stats", get(get_stats))
        .route("/api/v1/sources", get(list_sources))
        .route("/health", get(health_check))
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::cors::CorsLayer::permissive())
}

async fn query_logs(
    State(state): State<AppState>,
    AxumQuery(params): AxumQuery<QueryParams>,
) -> impl IntoResponse {
    let query = Query::from_params(params);
    let result = state.query_engine.query(&query).await;
    Json(result)
}

async fn stream_logs_ws(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_ws(socket, state))
}

async fn handle_ws(socket: axum::extract::ws::WebSocket, state: AppState) {
    let mut rx = state.event_bus.subscribe();
    let (mut sender, mut receiver) = socket.split();

    // Forward new events to WebSocket client
    loop {
        tokio::select! {
            Ok(event) = rx.recv() => {
                let json = serde_json::to_string(&event).unwrap();
                if sender.send(axum::extract::ws::Message::Text(json)).await.is_err() {
                    break;
                }
            }
            Some(Ok(msg)) = receiver.next() => {
                // Handle client messages (e.g., filter updates)
                if let axum::extract::ws::Message::Close(_) = msg { break; }
            }
        }
    }
}
```

---

### Ingestion Sources

```rust
// crates/logforge-cli/src/main.rs
use tokio::net::UdpSocket;
use tokio::net::TcpListener;

async fn start_tcp_ingestion(addr: &str, pipeline: Arc<Pipeline>, storage: Arc<Storage>) {
    let listener = TcpListener::bind(addr).await.unwrap();
    loop {
        let (socket, peer) = listener.accept().await.unwrap();
        let pipeline = Arc::clone(&pipeline);
        let storage = Arc::clone(&storage);
        tokio::spawn(async move {
            let reader = BufReader::new(socket);
            let mut lines = reader.lines();
            while let Some(Ok(line)) = lines.next_line().await.transpose() {
                if let Ok(event) = serde_json::from_str::<LogEvent>(&line) {
                    if let Some(processed) = pipeline.process(event).await {
                        storage.append(processed).await.unwrap();
                    }
                }
            }
        });
    }
}

async fn tail_file(path: &Path, pipeline: Arc<Pipeline>, storage: Arc<Storage>) {
    use notify::{Watcher, RecursiveMode, recommended_watcher};
    // Watch file for changes and process new lines
    // ...
}
```

---

### Configuration

```toml
# logforge.toml
[server]
http_port = 8080
tcp_port = 5140
udp_port = 5141

[storage]
data_dir = "/var/lib/logforge"
segment_size_mb = 256
retention_days = 30
compression = "zstd"

[pipeline]
workers = 8
buffer_size = 10000

[[pipeline.stages]]
type = "level_filter"
min_level = "info"

[[pipeline.stages]]
type = "regex_redact"
pattern = "password=\\S+"
replacement = "password=***"

[[pipeline.stages]]
type = "geoip_enrich"
db_path = "/etc/logforge/GeoLite2-City.mmdb"
field = "client_ip"

[auth]
enabled = true
api_key_header = "X-API-Key"
```

---

### Milestones

**Milestone 1 — Core Types & Parsing**
- [ ] Define `LogEvent`, `LogLevel`, `FieldValue` in `logforge-core`
- [ ] Implement JSON log parser
- [ ] Implement syslog (RFC 5424) parser
- [ ] Implement nginx access log parser
- [ ] Unit tests for all parsers with real log samples

**Milestone 2 — Pipeline**
- [ ] Implement `PipelineStage` trait
- [ ] `LevelFilter` stage
- [ ] `FieldEnricher` stage
- [ ] `RegexRedactor` stage (PII removal)
- [ ] `RateLimiter` stage
- [ ] Pipeline metrics (events/sec, drop rate)
- [ ] Property-based tests for pipeline correctness

**Milestone 3 — Storage Engine**
- [ ] Segment-based log storage with length-prefixed records
- [ ] Inverted index (level, source, tags)
- [ ] Time-range index (BTreeMap)
- [ ] Atomic segment rotation (when segment reaches size limit)
- [ ] Segment compaction and retention policy
- [ ] Query engine with filtering and pagination
- [ ] Benchmark: target 100k events/sec write throughput

**Milestone 4 — Ingestion Sources**
- [ ] TCP ingestion (JSON-lines)
- [ ] UDP ingestion (syslog)
- [ ] File tail ingestion (with `notify`)
- [ ] stdin ingestion
- [ ] Backpressure handling (bounded channels)

**Milestone 5 — REST API & WebSocket**
- [ ] `GET /api/v1/logs` — query with filters
- [ ] `POST /api/v1/logs` — ingest single event
- [ ] `GET /api/v1/logs/stream` — WebSocket live stream
- [ ] `GET /api/v1/stats` — throughput, storage stats
- [ ] API key authentication middleware
- [ ] Rate limiting middleware
- [ ] Integration tests using `reqwest`

**Milestone 6 — Configuration & CLI**
- [ ] TOML configuration file
- [ ] CLI: `logforge start`, `logforge query`, `logforge tail`
- [ ] Graceful shutdown (handle SIGTERM/SIGINT)
- [ ] Structured logging of logforge itself (using `tracing`)

**Milestone 7 — Testing & Quality**
- [ ] Unit tests for all modules
- [ ] Integration tests for full pipeline
- [ ] Property-based tests for storage correctness
- [ ] Benchmarks: ingestion throughput, query latency
- [ ] Clippy clean, rustfmt formatted
- [ ] CI/CD pipeline

---

### Stretch Goals

1. **Distributed Mode** — multiple nodes with consistent hashing for log routing
2. **Compression** — zstd compression for stored segments
3. **TLS** — TLS for TCP ingestion and HTTP API
4. **Alerting** — trigger webhooks when log patterns match
5. **Lua Scripting** — embed Lua for custom pipeline stages
6. **WASM Plugins** — pipeline stages as WebAssembly modules
7. **S3 Archival** — archive old segments to S3/MinIO
8. **Prometheus Metrics** — expose `/metrics` endpoint
9. **OpenTelemetry** — emit traces and metrics
10. **TUI Dashboard** — real-time terminal dashboard with `ratatui`

---

### Sample Usage

```bash
# Start the server
logforge start --config logforge.toml

# Query logs
logforge query --level error --source "nginx" --from "1h ago" --limit 100

# Tail live logs
logforge tail --level warn

# Ingest from stdin
cat access.log | logforge ingest --format nginx

# Send a log event via TCP
echo '{"level":"error","message":"disk full","source":"app"}' | nc localhost 5140
```

---

## 🎓 What's Next?

Congratulations on completing the Rust Developer Roadmap! You are now equipped to work on production Rust systems.

### Official Resources
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [The Async Book](https://rust-lang.github.io/async-book/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) — unsafe Rust
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)

### Advanced Topics to Explore
- **WebAssembly** — `wasm-pack`, `wasm-bindgen`
- **Embedded Systems** — `no_std`, `embedded-hal`
- **Game Development** — `Bevy` engine
- **Cryptography** — `ring`, `rustls`, `dalek`
- **Databases** — `sqlx`, `diesel`, `sled`
- **Distributed Systems** — `raft`, `tikv`
- **Compiler Development** — contribute to `rustc`

### Community
- [r/rust](https://www.reddit.com/r/rust/)
- [Rust Users Forum](https://users.rust-lang.org/)
- [This Week in Rust](https://this-week-in-rust.org/)
- [Rust Discord](https://discord.gg/rust-lang)

---

*Happy coding, Rustacean! 🦀*
