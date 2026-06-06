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
            break count * 2; // loops can return values
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