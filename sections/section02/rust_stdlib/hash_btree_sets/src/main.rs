use std::collections::{HashSet, BTreeSet};

fn demo_hashset() {
    println!("=== HashSet Demo ===");
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(String::from("page_1"));
    visited.insert(String::from("page_2"));
    visited.insert(String::from("page_1")); // duplicate — ignored

    println!("Visited {} unique pages", visited.len()); // 2

    // Set operations
    let a: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let b: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();

    // Union: all elements from both sets
    let union: HashSet<_> = a.union(&b).collect();
    println!("Union: {:?}", union);

    // Intersection: elements in both sets
    let intersection: HashSet<_> = a.intersection(&b).collect();
    println!("Intersection: {:?}", intersection); // {3, 4}

    // Difference: elements in a but not b
    let difference: HashSet<_> = a.difference(&b).collect();
    println!("Difference (a - b): {:?}", difference); // {1, 2}

    // Symmetric difference: elements in one but not both
    let sym_diff: HashSet<_> = a.symmetric_difference(&b).collect();
    println!("Symmetric difference: {:?}", sym_diff); // {1, 2, 5, 6}
    println!();
}

fn demo_btreeset() {
    println!("=== BTreeSet Demo ===");
    let mut visited: BTreeSet<String> = BTreeSet::new();
    visited.insert(String::from("page_1"));
    visited.insert(String::from("page_2"));
    visited.insert(String::from("page_1")); // duplicate — ignored

    println!("Visited {} unique pages", visited.len()); // 2

    // Set operations
    let a: BTreeSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let b: BTreeSet<i32> = [3, 4, 5, 6].iter().cloned().collect();

    // Union: all elements from both sets (sorted order)
    let union: BTreeSet<_> = a.union(&b).cloned().collect();
    println!("Union: {:?}", union);

    // Intersection: elements in both sets
    let intersection: BTreeSet<_> = a.intersection(&b).cloned().collect();
    println!("Intersection: {:?}", intersection); // {3, 4}

    // Difference: elements in a but not b
    let difference: BTreeSet<_> = a.difference(&b).cloned().collect();
    println!("Difference (a - b): {:?}", difference); // {1, 2}

    // Symmetric difference: elements in one but not both
    let sym_diff: BTreeSet<_> = a.symmetric_difference(&b).cloned().collect();
    println!("Symmetric difference: {:?}", sym_diff); // {1, 2, 5, 6}
    println!();
}

fn main() {
    demo_hashset();
    demo_btreeset();
}
