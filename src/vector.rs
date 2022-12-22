pub fn run() {
	#[derive(Debug)]
	enum Types {
		Int(i32),
		Float(f64),
		Text(String)
	}	
	let row = vec![
		Types::Int(10),
		Types::Float(0.13),
		Types::Text(String::from("Hello World"))
	];
	println!("{:?}", row);
}