use std::io;

fn celsius_to_fahrenheit(c: f64) -> f64 {
    // Formula: F = C * 9/5 + 32
    c * 9.0 / 5.0 + 32.0
}

fn celsius_to_kelvin(c: f64) -> f64 {
    // Formula: K = C + 273.15
    c + 273.15
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    // Formula: C = (F - 32) * 5/9
    (f - 32.0) * 5.0 / 9.0
}

fn kelvin_to_celsius(k: f64) -> f64 {
    // Formula: C = K - 273.15
    k - 273.15
}

fn parse_unit(input: &str) -> Option<&'static str> {
    // Convert the input to uppercase to make the comparison case-insensitive
    let input_upper = input.to_uppercase();
    
    // Check if the input is a substring of the full unit name.
    // This allows for flexible input like "C", "CELSIUS", "F", "FAHR", "FAHRENHEIT", "K", "KEL", "KELVIN".
    if "CELSIUS".contains(&input_upper) {
        Some("C")
    } else if "FAHRENHEIT".contains(&input_upper) {
        Some("F")
    } else if "KELVIN".contains(&input_upper) {
        Some("K")
    } else {
        None
    }
}

fn validate_temperature(value: f64, unit: &str) -> Option<String> {
    // Check for physically impossible temperatures and unreasonable values.
    if value < 0.0 && unit == "K" {
        Some("Temperature in Kelvin cannot be negative.".to_string())
    } else if unit == "C" && value < -273.15 {
        Some("Temperature in Celsius cannot be below absolute zero (-273.15°C).".to_string())
    } else if unit == "F" && value < -459.67 {
        Some("Temperature in Fahrenheit cannot be below absolute zero (-459.67°F).".to_string())
    } else if value > 1_000_000.0 {
        Some("Temperature value is too high. Please enter a reasonable value.".to_string())
    } else if value < -1000.0 {
        Some("Temperature value is too low. Please enter a reasonable value.".to_string())
    } else {
        None
    }
}

fn print_conversion_results(value: f64, unit: &str) {
    // First, convert the input temperature to Celsius, regardless of the original unit.
    let celsius = match unit {
        "C" => value,
        "F" => fahrenheit_to_celsius(value),
        "K" => kelvin_to_celsius(value),
        _ => return,
    };

    // Now we can convert from Celsius to the other units and print the results.
    let fahrenheit = celsius_to_fahrenheit(celsius);
    let kelvin = celsius_to_kelvin(celsius);

    // Print the results with two decimal places for better readability.
    println!("\nResults:");
    println!("  Celsius:    {:.2}°C", celsius);
    println!("  Fahrenheit: {:.2}°F", fahrenheit);
    println!("  Kelvin:     {:.2}K\n", kelvin);
}

fn main() {
    // include a loop to allow multiple conversions until the user decides to exit
    loop {
        println!("=== Temperature Converter ===");
        println!("Enter temperature (e.g., '100 C' or '212 F' or '373 K') or 'exit' to quit:");

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        // Check if the user wants to exit the program
        if input.trim().eq_ignore_ascii_case("exit") || input.trim().eq_ignore_ascii_case("quit") 
            || input.trim().eq_ignore_ascii_case("x") || input.trim().eq_ignore_ascii_case("q") {
            break;
        }

        // Split the input into parts and check if we have exactly 2 parts (value and unit).
        // If not, print an error message and continue the loop.
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 2 {
            println!("Invalid input. Please enter a number followed by C, F, or K.");
            continue;
        }

        // Try to parse the first part as a floating-point number. If it fails, print an error message and continue the loop.
        let value: f64 = match parts[0].parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid number.");
                continue;
            }
        };

        // We want to allow the user to enter a unit that has a letter, an abbreviation or the full name of the unit.
        // For example, "C", "CELSIUS", "F", "FAHR", "FAHRENHEIT", "K", "KEL", "KELVIN".
        // We can use a helper function to check if the input is a substring of the full unit name.
        let unit = match parse_unit(parts[1]) {
            Some(u) => u,
            None => {
                println!("Invalid unit. Use C, F, or K.");
                continue;
            }
        };

        // Validate the temperature value
        if let Some(error) = validate_temperature(value, unit) {
            println!("{}", error);
            continue;
        }

        // If we reach this point, the input is valid, so we can perform the conversions and print the results.
        print_conversion_results(value, unit);
    }
}