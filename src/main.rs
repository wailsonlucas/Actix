// #[derive(Debug)]
use std::io::{stdin};

fn main() {
    println!("Please inter a number");
    let mut input = String::new();

    loop {
        input.clear();

        stdin().read_line(&mut input).expect("Failed to read input");


        match input.trim().parse::<i8>() {
            Ok(_x) => break,
            Err(_x) => println!("Please enter a valid number"),
        }
    }
    
}


