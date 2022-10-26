
#[derive(Debug)]

struct User {
    username: String,
    email: String,
    balance: u64,
    active: bool
}

fn main() {
    let u1 = User {
        username: String::from("Yassine"),
        email: String::from("yassine@rust.com"),
        balance: 1000,
        active: true
    };

    let u2 = User {
        username: String::from("Wailson"),
        email: String::from("wailson@rust.com"),
        ..u1
    };

    println!("{:?}", u1);
    println!("{:?}", u2);
}


