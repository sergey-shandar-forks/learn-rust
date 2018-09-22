extern crate rand;

use rand::Rng;
use std::cmp::Ordering;

const MAX_POINTS: u32 = 100_000;

fn another_function(x: u8) -> u8 {
    println!("x = {}", x);
    let r = if x < 4 {
        x + 1
    } else if x < 10 {
        x - 1
    } else {
        x - 5
    };
    r
}

fn main() {
    
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_, _, _) = tup;
    let _ = tup.0;
    
    let xx = [1.0, 3.5];
    let _ = xx[1];

    another_function({
        let r = tup.2;
        r + 1
    });

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("Failed to read line 😻");
        // let guess: u32 = guess.trim().parse().expect("Please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; 
        println!("You guessed: {}", guess);
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