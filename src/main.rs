use std::io;

fn main() {
    println!("Temperature converter\n");

    loop {
        println!("What type of conversion would you like to do?\n");
        println!("1. Fahrenheit -> Celsius");
        println!("2. Celsius -> Fahrenheit");
        println!("3. Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if choice == 1 {
            fahrenheit_to_celsius();
        } else if choice == 2 {
            celsius_to_fahrenheit();
        } else if choice == 3 {
            println!("Exiting the program");
            break;
        } else {
            println!("Invalid choice. Please select 1, 2, or 3.");
        }
    }
}

fn fahrenheit_to_celsius() {
    let mut fahrenheit = String::new();

    println!("Enter temperature in Fahrenheit:");

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

    println!("{:.2}°F is {:.2}°C\n", fahrenheit, celsius);
}

fn celsius_to_fahrenheit() {
    println!("Enter temperature in Celsius:");

    let mut celsius = String::new();

    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line");

    let celsius: f64 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;

    println!("{:.2}°C is {:.2}°F\n", celsius, fahrenheit)
}
