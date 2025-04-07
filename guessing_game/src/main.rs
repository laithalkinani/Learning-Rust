//Build a guessing game. 

use std::io; //io comes from std library
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is : {secret_number}");
    println!("Please input your guess:");

    loop {

    let mut guess = String::new();
    //let creates the variable
    //mut means we can expect the value to change at some point (more on this later)
    //new is a function associated with String, std library
    //so when we call new() we just create an empty string
    //and we assign it to guess, a mutable variable
    io::stdin() //handle standard input to terminal
        .read_line(&mut guess) //no semi colon cause command not done
        .expect("Failed to read line!"); //for some reason didn't recognize the . on this line 
//references are immutable by default
//so we pass &mut guess to tell the compiler to pass guess as a mutable reference
//the newline and tab is for readability, we could've done read_line(blah).expect("blah") 
//read_line returns Return, an enum with Ok and Err members
//instance of Return has method expect which, if Return->Err, gets called and crashes the program
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    //parse can fail so it has a Return type as well
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) { //compare secret_number to guess, return Less, Equal, or Greater
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
        Ordering::Greater => println!("Too big"),
    }
}   

}

/*
Couple things:
match is an expression, and the arm/block of the expression is inside of it
So when you do match blah.parse(), you're saying:
hey, based on the different result values of parse(), "match" to a different action
hence the syntax of match which uses => and commas to delineate
*/