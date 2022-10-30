

fn main(){
    let arr:[i8; 5] = [21,12,46,42,45];
    let x = largest(arr);
    println!("{}", x);

    let text: [char; 5] = ['a', 'b','c','d','e'];
    let y = largest(text);
    println!("{}", y);
   
}

fn largest<T: PartialOrd + Copy>(arr: [T; 5]) -> T {
    let mut largest = arr[0];
    
    for number in arr {
        if number > largest {
            largest = number
        }
    }         
    
    largest
}