
#[derive(Debug)]
struct Point {
    x:f64,
    y:f64
}
    
fn main(){

    let a: Point = Point {
        x: 10.00,
        y: 12.00
    };

    let b: Point = Point {
        x: 5.00,
        y: 4.00
    };

    distance(a, b);

}

fn distance(a:Point ,b:Point) {
    let long = a.x - b.x;
    let width = a.y - b.y;
    let distance = (long.powf(2.0) + width.powf(2.0)).sqrt();
    println!("{}", distance);
}