use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    // First Number asdfg
    println!("Enter first number:");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    println!("Enter second number:");
    io::stdin().read_line(&mut input2).expect("Failed to read line");

    // we need to convert input because input data in String or $str
    let num1: i32 = input1.trim().parse().expect("Please type a number!");
    let num2: i32 = input2.trim().parse().expect("Please type a number!");

    let sum = num1 + num2;

    println!("The sum is: {}", sum);
}
