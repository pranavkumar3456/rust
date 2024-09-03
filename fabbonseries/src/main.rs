use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Enter the number of terms for the Fibonacci series:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: usize = input.trim().parse().expect("Please type a valid number!");

    let mut fib_series = Vec::new();

    // Generate the Fibonacci series
    for i in 0..n {
        if i == 0 {
            fib_series.push(0);
        } else if i == 1 {
            fib_series.push(1);
        } else {
            let next_value = fib_series[i - 1] + fib_series[i - 2];
            fib_series.push(next_value);
        }
    }

    // Print the Fibonacci series
    println!("Fibonacci series: {:?}", fib_series);
}
