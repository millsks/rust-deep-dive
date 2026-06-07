use std::collections::VecDeque;

fn main() {
    let mut deque: VecDeque<i32> = VecDeque::new();

    // Push to back (like a queue)
    deque.push_back(1);
    deque.push_back(2);
    deque.push_back(3);

    // Push to front (like a stack)
    deque.push_front(0);

    println!("{:?}", deque); // [0, 1, 2, 3]

    // Pop from front (FIFO queue behavior)
    println!("Popped from front");
    println!("{}", "=".repeat(20));
    while let Some(val) = deque.pop_front() {
        println!("Processing: {}", val);
    }


    // Push to front (like a stack)
    deque.push_front(23);
    deque.push_front(16);
    deque.push_front(9);

    // Push to back (like a queue)
    deque.push_back(99);

    println!("{:?}", deque); // [0, 1, 2, 3]


    // Pop from back (LIFO stack behavior)
    println!("Popped from back");
    println!("{}", "=".repeat(20));
    while let Some(val) = deque.pop_back() {
        println!("Processing: {}", val);
    }
}
