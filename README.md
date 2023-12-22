# Guess Game

## Description

This is a simple guess game. The computer will generate a random number between 1 and 100. The user will have to guess the number. The computer will tell the user if the number is too high or too low. The user will have 10 attempts to guess the number.

## How to run

1. Clone the repository
2. Open the terminal and navigate to the project folder
3. Run the following commmand: `cargo run`
4. Follow the instructions on the screen

## Developer Notes

### Generating a random number

`rand::thread_rng`

This function gives us an instance of the random number generator we will use,
i.e. one that is 'local' to the current thread of execution and seeded by the
operating system.

`gen_range`

This method is called on the random number generator. It is defined by the `Rng`
trait brought into scope via `use rand::Rng`. This method takes a range
expression using the following form:
```
start..=end
```
Where 'start' is the lower bound and 'end' is the upper bound.

### Creating a String Variable

```
let mut guess = String::new();
```

The `let` statement is used to create a new variable:

```
let apples = 5
```

By default, Rust's variables are immutable, meaning once a value is given to a variable, the value won't change. We can change the default behavior by added
the keyword `mut` before the variable name.

```
let mut bananas = 3
```

`String::new()`

This is a function that returns a String instance. This is a "string" type that
is provided by the Rust standard library. It is a "growable", UTF-8 encoded bit
of text.

The `::` Syntax

This is seen in the `::new` line below. It indicates that `new` is an associated 
function of the `String` type. An "associated function" is a function that is
implemented on a type, in this case 'String'. The 'new' associated `String`
function will create a new, empty string. The `new` function is common to many
types and will make a new value of some kind.

### Reading User Input

#### Standard Input and Output
We can use a standard library by including the following line to the top of the
source file: `use std::io`. If we had not included the `use std::io` line we can
still call the function explicitly by writing the function call in the following
way: `std::io::stdin`

The "stdin" function returns an instance of `std::io::Stdin`, which is a type
that represents a handle to the standard input for your terminal.

##### `Stdin` Methods

The line `.read_line(&mut guess)` calls the `read_line` method on the standard
input handle to get input from the user. We pass `&mut guess` to tell the method
to store the user's input into the `guess` variable. This method returns a
`Result` value.

`Result` Enumeration

`Result` is an enumeration, which is a type that can be in one of multiple
possible states. Each of these possible states is called a "variant". The
purpose of these `Result` types is to encode error handling information.

Result's "variations" are `Ok` and `Err`:
- `Ok`: indicates the operation was successful, the `Ok` contains the
successfully generated value.
- `Err`: indicates that the operation failed, the `Err` contains information
about how or why the operation failed.

Like other types, the `Result` type has its own set of methods. An instance of
`Result` has an `expect` method that can be called to crash the program and
display the message passed as an argument to `expect` if the result is an `Err`
value.

### References

The `&` indicates that this argument is a "reference", which gives you a way to
let multiple parts of your code access one piece of data without needing to copy
that data into memory multiple times. Like variables, references are also
immutable by default, which is why we need to write `&mut guess` rather than
`&guess` to make it mutable.

### Expectations and Handling Potential Failure

To handle potential failures the `expect` call makes use of the value returned
by the `Result` instance.

If the instance of `Result` is an `Err` value, `expect` will cause the program
to crash and display the message that was passed as an argument to `expect`.
This message should explain why the program crashed so the user knows what they
need to fix.

If the instance of `Result` is an `Ok` value, `expect` will use the return
value that `Ok` is holding and return that value so that it can be used.

If you do not call `expect`, the program will compile but a warning will be
generated. Rust warns that you have not used the `Result` value returned from 
`read_line`, indicating that the program has not handled a possible errors.

### Crate: rand

A "crate" is a collection of Rust source code files. Specifically, this project
is a "binary crate", i.e. an executable. The `rand` crate is a "library crate",
which contains code that is intended to be used in other programs and cannot be
executed on its own.

Before using the 'rand' crate we need to modify this projects Cargo.toml file to
include the 'rand' crate as a dependency. To do so we add the following line to
Cargo.toml:

```
[dependencies]
rand = "0.8.5"
```

Above we indicate that this project depends on the `rand` crate (version 0.8.5
or above) be available. The specifier `0.8.5` is shorthand for ^0.8.5, i.e. any
version that is at least 0.8.5 but below 0.9.0.

Cargo considers these versions to have public APIs compatible with version
0.8.5, and this specification ensures you will get the latest patch release that
will still compile with the code in this chapter.

Cargo uses Semantic Versioning (SemVer) to ensure that versions are all
compatible with one another such that the code can compile. When an external
dependency is included, Cargo fetches the latest versions of everything that
dependency needs from the "registry", which is a copy of data from Crates.io,
where people in the Rust community post their open source Rust projects for
others to use.

After updating the registry, Cargo checks `[dependencies]` section and downloads
any crates listed that are not already downloaded and any dependencies that
dependency depends on. After downloading, Rust compiles them and then compiles
the project with the dependencies now available.

### Handling Invalid Input

The `expect` method is used to crash the program when an error occurs. This is
not a good way to handle errors in a production program. Instead, we can write
code to handle the `Result` value returned by `read_line` and take different
actions for `Ok` and `Err` variants.

```
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

The `match` expression is made up of "arms". An arm consists of a pattern and
the code that should be run if the value given to the beginning of the `match`
expression fits that arm's pattern. Rust takes the value given to `match` and
looks through each arm's pattern in turn. The `trim` method on a `String`
instance will eliminate any whitespace at the beginning and end. The `parse`
method on strings parses a string into some kind of number. The `parse` method
returns a `Result` type. If `parse` is able to successfully turn the string into
a number, it will return an `Ok` value that contains the resulting number. If it
is not able to turn the string into a number, it will return an `Err` value that
contains more information about the error.

### Printing Values

The `println!` macro prints a string to the screen. The `!` indicates that this
is a macro rather than a function. The `println!` macro takes a string as its
first argument. The `{}` is a placeholder that `println!` will replace with
values passed to `println!` after the string.

### Comparing Values

The `cmp` method compares two values and can be called on anything that can be
compared. It takes a reference to whatever you want to compare with the thing in
the `match` expression and returns one of the `Ordering` variants: `Less`,
`Greater`, or `Equal`.

### Looping

The `loop` keyword creates an infinite loop. The `break` keyword can be used to
exit a loop. The `continue` keyword can be used to skip the rest of the current
iteration and start a new one.
