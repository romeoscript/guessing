use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
    
    println!("please input your guess");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failded to read the code ");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("you guessed : {guess}");
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
