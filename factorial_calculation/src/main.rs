use std::io;

fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter a number to calculate its factorial:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let num: u32 = input.trim().parse().expect("Please type a number!");

    println!("The factorial of {} is {}", num, factorial(num));
}
