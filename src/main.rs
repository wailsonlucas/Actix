fn main(){
   let connection: Vec<i8> = vec![1,2,3];
   let closure_fn =  || {
      println!("connectivity state {:?}", connection);
   };
   closure_fn();
   println!("{:?}", connection);
}