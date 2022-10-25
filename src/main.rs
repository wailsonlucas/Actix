// #[derive(Debug)]
use std::io::{ stdin, stdout, Write };

fn main() {
    println!("Please input a number: ");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    
    number_validator(&input);

}

fn number_validator(n: &String) {
    let number = n.trim().parse::<i32>().expect("Please insert a number");
    println!("{:?}", number);
}
