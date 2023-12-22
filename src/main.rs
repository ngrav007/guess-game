use rand::Rng; // include 'rand' crate's 'Rng' trait which defines methods that random number generators implement
use std::cmp::Ordering;
use std::io; // include input/output library, provides I/O functionality // the Ordering type is another enum, it has the variants: Less, Greater, and Equal
use std::io::Write; // bring flush() into scope

fn main() {
    println!("+ --------------------- Guess the Number! --------------------- +");

    // Generate a Random Number (1-100)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Create a Mutable String Variable
    
    // Guessing Loop
    loop {
        let mut guess = String::new();
        print!("[*] Please input your guess... ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut guess)
            .expect("[-] Failed to read line");

        // shadow / convert the previous value of 'guess' to a u32 type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // the '_' is a catchall, it matches with *all* Err values
        };

        println!("[*] You guessed: {guess}");

        // check guess
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("[-] Too small!"),
            Ordering::Greater => println!("[+] Too big!"),
            Ordering::Equal => {
                println!("[=] You win!");
                break;
            }
        }
    }
}
