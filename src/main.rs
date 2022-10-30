
#[derive(Debug)]
struct GenericStruct<T, U> {
    x:T,
    y:U
}


fn main(){
    let A:GenericStruct<i8, String> = GenericStruct {
        x: 3,
        y:10.to_string()
    };
    println!("{:#?}", A);
   
}

