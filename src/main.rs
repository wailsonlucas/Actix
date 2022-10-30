#[derive(Debug)]
struct TuppleStruct(String);
    
fn main(){
    let n:TuppleStruct = TuppleStruct(String::from("Mohammed"));
    println!("{:?}", n);
}

