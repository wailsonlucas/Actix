pub fn run() {
	let x:Vec<i32> = vec![1,2,3,4,5];
	let third:Option<&i32> = x.get(2);

	if let Some(x) = third {
		println!("{:?}", x);
	} 
	
}