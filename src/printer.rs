#[derive(Debug)]
pub struct Server {
	address: String,
	price: u8
}

pub trait ServerTrait {
	fn init(address:&String, price:u8) -> Server;
}


impl ServerTrait for Server {
	fn init(address:&String, price:u8) -> Server {
		Server {
			address: address.to_string(),
			price
		}
	}
}

