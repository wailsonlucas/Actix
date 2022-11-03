
// #[derive(Debug)]


fn main(){
   let x:Option<i8> = Some(5);
   // println!("{:?}", x);

   if let Some(7) = x {
    println!("true",);
   } else {
    println!("false");
   }
   
}

