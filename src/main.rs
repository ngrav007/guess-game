use std::io; // include input/output library, provides I/O functionality
use rand::Rng;  // include 'rand' crate's 'Rng' trait which defines methods that random number generators implement
use std::cmp::Ordering  // the Ordering type is another enum, it has the variants: Less, Greater, and Equal

fn main() {
    println!("+ --------------------- Guess the Number! --------------------- +");

    /* generate a random secret number 
     * 
     * `rand::thread_rng`
     *      This function gives us an instance of the random number generator
     *      we will use, i.e. one that is 'local' to the current thread of
     *      execution and seeded by the operating system.
     * 
     * `gen_range`
     *      This method is called on the random number generator. It is defined
     *      by the Rng trait brought into scope via `use rand::Rng`. This
     *      method takes a range expression using the following form:
     * 
     *          start..=end
     *      
     *      Where 'start' is the lower bound and 'end' is the upper bound.
     */ 
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("[*] The secret number is {secret_number}");

    println!("[>] Please input your guess...");

    /*
     * Create a Variable
     *  `let` statement is used to create a new variable
     *      Example: `let apples = 5`
     *  
     *  By default, Rust's variables are immutable, meaning once a value is
     *  given to a variable, the value won't change. We can change the default
     *  behavior by added the keyword `mut` before the variable name.
     *      Example: `let mut bananas = 3`
     * 
     * String::new()
     *  This is a function that returns a String instance. This is a "string"
     *  type that is provided by the Rust standard library. It is a "growable",
     *  UTF-8 encoded bit of text.
     * 
     * The "::" syntax
     *  This is seen in the `::new` line below. It indicates that `new` is an
     *  associated function of the `String` type. An "associated function" is a
     *  function that is implemented on a type, in this case 'String'. The 'new'
     *  associated String function will create a new, empty string. The `new`
     *  function is common to many types and will make a new value of some kind.
     */
    let mut guess = String::new();

    /*
     * Standard Input and Output
     *  If we had not included the `use std::io` line we can still call the
     *  function by writing the function call in the following way:
     *      `std::io::stdin`
     *  The "stdin" function returns an instance of `std::io::Stdin`, which is
     *  a type that represents a handle to the standard input for your terminal.
     * 
     * Methods
     *  The line `.read_line(&mut guess)` calls the `read_line` method on the
     *  standard input handle to get input from the user. We pass `&mut guess`
     *  to tell the method to store the user's input into the `guess` variable.
     *  This method returns a `Result` value. 
     * 
     * Result Enumeration
     *  `Result` is an enumeration, which is a type that can be in one of
     *  multiple possible states. Each of these  possible states is called a
     *  "variant". The purpose of these `Result` types is to encode error
     *  handling information. 
     * 
     *  Result's variations are `Ok` and `Err`.
     *      `Ok`  - indicates the operation was successful, inside Ok is the
     *              successfully generated value.
     *      `Err` - means that the operation failed, and `Err` contains
     *              information about how or why the operation failed
     * 
     *  Like other types, the Result type has its own set of methods. An
     *  instance of Result has an `expect` method that can be called.
     * 
     * References
     *  The '&' indicates that this argument is a 'reference', which gives you a
     *  way to let multiple parts of your code access one piece of data without
     *  needing to copy that data into memory multiple times. Like variables,
     *  references are also immutable by default, which is why we need to write
     *  `&mut guess` rather than `&guess` to make it mutable.
     * 
     * Exceptions
     *  To handle potential failures the `except` call makes use of the value
     *  returned by the Result instance.
     * 
     *  If the instance of Result is an `Err` value, `except` will cause the
     *  program to crash and display the message that was passed as an argument
     *  to `except`
     * 
     *  If the instance of Result is an `Ok` value, `except` will take the
     *  return value that `Ok` is holding and return just that value so that
     *  it can be used. In this case, the value is the number of bytes in the
     *  user's input. 
     * 
     *  If you do not call `except`, the program will compile but a warning will
     *  be generated. Rust warns that you haven’t used the Result value returned
     * from read_line, indicating that the program hasn’t handled a possible
     * error.
     * 
     * The right way to suppress the warning is to actually write error-handling
     * code, but in our case we just want to crash this program when a problem
     * occurs, so we can use expect. 
     */
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
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
    println!("You guessed: {guess}")

    /*
     * CRATE: rand
     * 
     * A create is a collection of Rust source code files. Specifically, this
     * 'project' is a "binary crate", i.e. an executable. The 'rand' crate is
     * a "library crate", which contains code that is intended to be used in 
     * other programs and cannot be executed on its own.
     * 
     * Before using the 'rand' crate we need to modify this projects Cargo.toml
     * file to include the 'rand' crate as a dependency. To do so we add the
     * following line to Cargo.toml:
     * 
     *      [dependencies]
     *      rand = "0.8.5"
     * 
     * Above we indicate that this project depends on the 'rand' crate (version
     * 0.8.5 or above) be available. The specifier '0.8.5' is shorthand for 
     * '^0.8.5', which means any version that is at least 0.8.5 but below 0.9.0.
     * 
     * Cargo considers these versions to have public APIs compatible with
     * version 0.8.5, and this specification ensures you will get the latest
     * patch release that will still compile with the code in this chapter.
     * 
     * Cargo uses Semantic Versioning (SemVer) to ensure that versions are all
     * compatible with one another such that the code can compile. When an
     * external dependency is included, Cargo fetches the latest versions of
     * everything that dependency needs from the "registry", which is a copy of
     * data from Crates.io, where people in the Rust ecosystem post their open
     * source Rust projects for others to use.
     * 
     * After updating the registry, Cargo checks `[dependencies]` section and
     * downloads any crates listed that are not already downloaded and any
     * dependencies that dependency depends on. After downloading, Rust compiles
     * them and then compiles the project with the dependencies now available.
     */
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

}
