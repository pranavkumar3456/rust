use std::io; // Rust has set of items defined in standard library. This set of library called
             // prelude. std::io library provides you with number of useful features including
             // ability to accept user input.

fn main() {
    println!("Guess the number!");

    println!("Please input  your guess.");

    let mut guess = String::new();  // we create a variable guess to store a value like this. eg.
                                   // let apple = 5;immutable variable --> once value added can't
                                   // change
                                   
                                   // let mut banana = 5; mutable variable  --> to make variable
                                   // mutable we add mut.

                                   // let mut guess will create a mutable variable named guess.
                                   // equal sign (=) tell rust that we want to bind something to
                                   // varibable now.
                                   
                                   // String::new, a function that return new instance of String.
                                   // :: syntax in ::new indicates that new is associated with
                                   // fuction of string. i.e., new called associated function(this
                                   // type of function implemented on type, in this case String)
                                   // new() creates a empty string.

// Recieving User input
    io::stdin()                     // this module will allow to handle user input. we can write
                                    // same in the beginneing of the line aswell. like
                                    // std::io:stdin                      
        .read_line(&mut guess)
        .expect("Failed to read lines"); 
                                    // read_line method handle input from user. We are also passing
                                    // &mut guess as the argument to read_line to tell it what
                                    // string to store the user input in.
                                    // The full job of &mut guess is to take whatever the user type
                                    // into standard input and append that into string without
                                    // overwritting it's content.  & represent a reference, which
                                    // gives a way to let multiple parts of  your code access once
                                    // piece of data without needing to copy that data in memory
                                    // multiple times.
                                    // .read_line(&mut guess).expect("Failed to read lines"); it
                                    // can be written in this way too.

    println!("You guessed: {guess}");
}
