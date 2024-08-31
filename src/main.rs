use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Starting the 'Guessing Game'...!");
    println!("Guess the number");
    let mut res = 0;
    while res.clone() < 1 {
        // if answer is wrong then keep res=0 and if answer right then res=1
        comparer(&mut res);
    }
}

fn comparer(res: &mut i32) {
    let mut guess: String = String::new();
    let numb = rand::thread_rng().gen_range(1..=10);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read user input");
    // println!("{}", numb);
    let guess_number = guess.trim().parse::<i8>().expect("Failed to parse");
    match guess_number.cmp(&numb) {
        Ordering::Less => println!("Too low! Try again. {numb}"),
        Ordering::Greater => println!("Too high! Try again. {numb}"),
        Ordering::Equal => {
            println!("Congratulations! You guessed the correct number.");
            *res = 1;
        }
    }
}
