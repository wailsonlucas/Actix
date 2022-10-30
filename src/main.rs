#[derive(Debug)]
enum Vehicle {
    Car(String)
}

impl Vehicle {
    fn color(&self){
        println!("red");
    }
}
    
fn main(){
    let car = Vehicle::Car("BMW".to_string()).color();
}

