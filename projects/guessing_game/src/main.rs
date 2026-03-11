use std::io;

fn main() {
    println!("Guess a number");
    println!("Input the guess please");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read anything");

    println!("Your Guess is {guess} ");

}
