use std::collections::BTreeMap;
use std::ops::Bound::Included;

fn main() {
    let mut map = BTreeMap::new();
    map.insert("banana", 3);
    map.insert("apple", 5);
    map.insert("cherry", 1);

    // Iteration is always in sorted key order
    for (fruit, count) in &map {
        println!("{}: {}", fruit, count); // apple, banana, cherry
    }

    // Range query with explicit bounds.
    // This is more verbose, but it lets you mix Included / Excluded / Unbounded when needed.
    for (fruit, count) in map.range::<str, _>((Included("apple"), Included("cherry"))) {
        println!("Explicit bounds: {} = {}", fruit, count);
    }

    // Range query with shorthand syntax.
    // "..=" means both ends are inclusive and is usually the cleanest form for this common case.
    for (fruit, count) in map.range("apple"..="banana") {
        println!("Shorthand ..=: {} = {}", fruit, count);
    }
}
