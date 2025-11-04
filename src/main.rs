use std::io;

// enumerator to represent conversion types and give user the option to exit the program
enum Conversion {
    FahrenheitToCelsius,
    CelsiusToFahrenheit,
    Exit,
}

fn main() {
    println!("Temperature converter\n");

    loop {
        println!("What type of conversion would you like to do?\n");
        println!("1. Fahrenheit -> Celsius");
        println!("2. Celsius -> Fahrenheit");
        println!("3. Exit");

        let choice = read_input("Enter your choice:");

        match parse_choice(&choice.to_string()) {
            Some(Conversion::FahrenheitToCelsius) => fahrenheit_to_celsius(),
            Some(Conversion::CelsiusToFahrenheit) => celsius_to_fahrenheit(),
            Some(Conversion::Exit) => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            None => {
                println!("Please enter a valid option (1, 2, or 3).");
                continue;
            }
        }
    }
}

// function to grab user input to keep main and other functions as clean as possible
fn read_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

// parser to convert user input into the appropriate enum variant
fn parse_choice(input: &str) -> Option<Conversion> {
    match input.trim() {
        "1" => Some(Conversion::FahrenheitToCelsius),
        "2" => Some(Conversion::CelsiusToFahrenheit),
        "3" => Some(Conversion::Exit),
        _ => None,
    }
}

// function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius() {
    let input = read_input("Enter temperature in Fahrenheit:");
    let fahrenheit: f64 = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

    println!("{:.2}°F is {:.2}°C\n", fahrenheit, celsius);
}

// function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit() {
    let input = read_input("Enter temperature in Celsius:");
    let celsius: f64 = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;

    println!("{:.2}°C is {:.2}°F\n", celsius, fahrenheit);
}

// test modules
#[cfg(test)]
mod tests {
    

    #[test]
    fn test_fahrenheit_to_celsius() {
        let fahrenheit = 100.0;
        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        assert_eq!(celsius, 37.77777777777778);
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        let celsius = 0.0;
        let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
        assert_eq!(fahrenheit, 32.0);
    }
}
