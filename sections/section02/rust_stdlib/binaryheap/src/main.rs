use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(3);
    heap.push(1);
    heap.push(4);
    heap.push(1);
    heap.push(5);

    // Always pops the largest element first
    while let Some(val) = heap.pop() {
        print!("{} ", val); // 5 4 3 1 1
    }
    println!();
}