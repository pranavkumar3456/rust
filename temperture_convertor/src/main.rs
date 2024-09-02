use std::io;
fn main() {

    let mut input = String::new();
    println!("Please provide the temperature in celcius");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let float_input: f64  = input.trim().parse().expect("Please type a valid float value!");

    let fahren = (float_input*1.8)+32.0;

    println!("Please find the temperature in fahrenheit: {}",fahren);

}
