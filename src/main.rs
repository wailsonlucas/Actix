#[derive(Debug)]

enum Gender {
    Male, Female
}

#[derive(Debug)]
struct Person {
    name: String,
    gender: Gender
}

fn main() {

    let yassine:Person = Person {
        name: String::from("Yassine"),
        gender: Gender::Male
    };

    let amina:Person = Person {
        name: String::from("Amina"),
        gender: Gender::Female
    };

    println!("{:?}", yassine);
    println!("{:?}", amina);

}

