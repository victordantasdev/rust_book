use std::io;
// use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    let secret = rand::thread_rng().gen_range(1..100);
    println!("secret: {secret}");
    
    println!("==> Guessing Game <==");
    
    loop {
        println!("Please input your guess: ");
    
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        
        let guess: i32 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
    
        println!("You guessed: {guess}");
    
        // match guess.cmp(&secret) {
        //     Ordering::Less => println!("{}", "Too small!".red()),
        //     Ordering::Greater => println!("{}", "Too big!".red()),
        //     Ordering::Equal => {
        //         println!("{}", "You win!".green());
        //         break;
        //     },
        // }

        let rang = -2..2;
        // let dif = &guess - &secret;
        // let x = rang.contains(&dif);
        // println!("{:#?}", x);

        match &guess {
            g if rang.contains(g) => println!("{}", "Near!".yellow()),
            g if g > &secret => println!("{}", "Too big!".red()),
            g if g < &secret => println!("{}", "Too small!".red()),
            g if g == &secret => {
                println!("{}", "You win!".green());
                break;
            },
            _ => print!("Not match!")
        }
    }
}
