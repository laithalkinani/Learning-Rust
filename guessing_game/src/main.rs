//Build a guessing game. 

use std::io; //io comes from std library

fn main() {
    println!("Guess the number!");
    println!("Please input your number guess.");

    let mut guess = String::new();
    //let creates the variable
    //mut means we can expect the value to change at some point (more on this later)
    //new is a function associated with String, std library
    //so when we call new() we just create an empty string
    //and we assign it to guess, a mutable variable


    io::stdin() //handle standard input to terminal
        .read_line(&mut guess) //no semi colon cause command not done
        .expect("Failed to read line!"); 
        //references are immutable by default
        //so we pass &mut guess to tell the compiler to pass guess as a mutable reference
        //the newline and tab is for readability, we could've done read_line(blah).expect("blah") 
        //read_line returns Return, an enum with Ok and Err members
        //instance of Return has method expect which, if Return->Err, gets called and crashes the program
    println!("You guessed: {}", guess);
}
