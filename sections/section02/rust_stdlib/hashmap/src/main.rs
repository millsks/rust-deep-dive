use std::collections::HashMap;

fn main() {
    // Creating a HashMap
    let mut scores: HashMap<String, u32> = HashMap::new();

    // Inserting values
    scores.insert(String::from("Alice"), 95);
    scores.insert(String::from("Bob"), 87);
    scores.insert(String::from("Charlie"), 92);

    // Accessing values — returns Option<&V>
    if let Some(score) = scores.get("Alice") {
        println!("Alice's score: {}", score);
    }

    // The entry API — the most idiomatic way to insert-or-update
    // This avoids double-lookups and is very common in real code
    scores.entry(String::from("Dave")).or_insert(0);

    // Increment a counter (classic pattern)
    let text = "hello world hello rust hello";
    let mut word_count: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1; // dereference to modify the value in-place
    }
    println!("{:?}", word_count); // {"hello": 3, "world": 1, "rust": 1}

    // Iterating — order is NOT guaranteed
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // Checking existence
    println!("Has Eve? {}", scores.contains_key("Eve"));

    // Removing
    scores.remove("Bob");

    // Getting the number of entries
    println!("Total students: {}", scores.len());
}