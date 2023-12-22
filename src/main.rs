use rand::Rng; // include 'rand' crate's 'Rng' trait which defines methods that random number generators implement
use std::cmp::Ordering;
use std::io; // include input/output library, provides I/O functionality // the Ordering type is another enum, it has the variants: Less, Greater, and Equal

fn main() {
    println!("+ --------------------- Guess the Number! --------------------- +");

    // Generate a Random Number (1-100)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Create a Mutable String Variable
    let mut guess = String::new();

    // Guessing Loop
    loop {
        println!("[>] Please input your guess...");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadow the previous guess variable and convert the String type to integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // the '_' is a catchall, it matches with *all* Err values
        };

        println!("You guessed: {guess}");
        /*
         * The {} set of curly brackets is a placeholder: think of {} as little crab
         * pincers that hold a value in place. When printing the value of a
         * variable, the variable name can go inside the curly brackets. When
         * printing the result of evaluating an expression, place empty curly
         * brackets in the format string, then follow the format string with a
         * comma-separated list of expressions to print in each empty curly bracket
         * placeholder in the same order. Printing a variable and the result of an
         * expression in one call to println! would look like this:
         *
         *      let x = 5;
         *      let y = 10;
         *      println!("x = {x} and y + 2 = {}", y + 2);
         *
         * This code would print x = 5 and y + 2 = 12.
         */

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
