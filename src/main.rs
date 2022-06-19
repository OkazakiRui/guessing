use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guessing the number");

    let secret_number = rand::thread_rng().gen_range(0..=100);

    println!("secret number is {}", secret_number);

    loop {
        println!("please input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("failed to read");

        let guess = match guess.trim().parse::<u32>() {
            Ok(n) => n,
            Err(_) => continue,
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
