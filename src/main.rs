use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Take a guess");

    loop {
        println!("Please input your number.");

        let secret_number = rand::rng().random_range(1..=100);

        println!("The secret number is {}", secret_number);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }






}
fn hello() {
    println!("Hello, world!");

}

fn hello2() {
    println!("Hello, world!");
}


fn hello3() {
    println!("Hello, world!");
}