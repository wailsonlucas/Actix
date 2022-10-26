fn main() {
    let text = String::from("Hello World this is Rust lang");
    first_word(&text);
}


fn first_word(text: &str){
    let mut blank_indecies:Vec<i8> = vec![];
    for (i, j) in text.bytes().enumerate() {
        if j == b' ' {
            println!("{}", &text[..i]);
            blank_indecies.push(i.try_into().unwrap());
        }
    }
    let _first_index: i8 = blank_indecies[0];
    // println!("{}", &text[..first_index]);
}