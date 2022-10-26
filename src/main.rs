// #[derive(Debug)]
// use std::io::{stdin};

fn main() {
    
    let input = String::from("install");
    let x = input_validator(&input);
     match x {
        Ok(value) => println!("installing the package with message {}", &value[2..5]),
        Err(error) => println!("the error is {}", error),
     }
}

fn input_validator(input: &String) -> Result<&str, &str> {
    let err = "Ops! something went wrong";

    if input == "install" {
        Ok("installing")
    } else {
       Err(err)
    }
}


