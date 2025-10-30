use std::io;

fn main() {
    println!("Temperature Conversion CLI");

    loop {
        println!("What type of conversion would you like to process?\n");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("3. Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        println!("User chose: {choice}");
    }
}
