use std::io;

fn main() {
    // Constants to set first two Fibonacci numbers
    const NUMBER_ZERO: u32 = 0;
    const NUMBER_ONE: u32 = 1;

    // User input prompt
    println!("Enter the Fibonacci number you want to calculate.");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Please enter a number");

    // Assign user entered number to variable
    let number: u64 = number.trim().parse().expect("Expected a number");

    let mut lead = NUMBER_ONE;
    let mut lag = NUMBER_ZERO;
    let mut sum = lag + lead;
    if number == 1 {
        sum = 0;
        println!("The {}st Fibonacci number is: {}", number, sum);
    } else if number < 4 {
        sum = 1;
        println!("The {} Fibonacci number is: {}", number, sum);
    } else {
        // Generate Fibonacci numbers - following number in sequence is the sum of the two proceeding numbers
        for i in 3..number {
            lag = lead;
            lead = sum;
            sum = lead + lag;
    }

    println!("The {}th Fibonacci number is: {}", number, sum);
    }
}
