use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a number to check if it is prime:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let num: u64 = input.trim().parse().expect("Please type a number!");

    if is_prime(num) {
        println!("{} is a prime number.", num);
    } else {
        println!("{} is not a prime number.", num);
    }
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
