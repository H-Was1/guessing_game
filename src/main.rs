use rand::Rng;
use std::io;
fn main() {
    println!("Starting the 'Guessing Game'...!");
    println!("Guess the number");
    let mut guess: String = String::new();
    let numb = rand::thread_rng().gen_range(1..10);
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read user input");
    println!("{}", numb);
    // if &guess.parse::<i8>().expect("Something went wrong") > &numb {
    //     println!("Too high! Try again.");
    // } else if &guess.parse::<i8>().expect("Something went wrong") < &numb {
    //     println!("Too low! Try again.");
    // } else if &guess.parse::<i8>().expect("Something went wrong") == &numb {
    //     println!("Congratulations! You guessed the correct number.");
    // }
}
