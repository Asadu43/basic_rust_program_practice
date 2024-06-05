use std::io;

fn main() {
    let mut choice = String::new();

    println!("Choose conversion: ");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    match choice.trim() {
        "1" => celsius_to_fahrenheit(),
        "2" => fahrenheit_to_celsius(),
        _ => println!("Invalid choice"),
    }
}

fn celsius_to_fahrenheit() {
    let mut celsius = String::new();
    println!("Enter temperature in Celsius:");
    io::stdin().read_line(&mut celsius).expect("Failed to read line");
    let celsius: f64 = celsius.trim().parse().expect("Please type a number!");
    let fahrenheit = (celsius * 9.0) / 5.0 + 32.0;
    println!("Temperature in Fahrenheit: {:.2}", fahrenheit);
}

fn fahrenheit_to_celsius() {
    let mut fahrenheit = String::new();
    println!("Enter temperature in Fahrenheit:");
    io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");
    let fahrenheit: f64 = fahrenheit.trim().parse().expect("Please type a number!");
    let celsius = ((fahrenheit - 32.0) * 5.0) / 9.0;
    println!("Temperature in Celsius: {:.2}", celsius);
}
