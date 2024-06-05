use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a string to reverse:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let reversed: String = input.trim().chars().rev().collect();

    println!("Reversed string: {}", reversed);
}
