#[derive(Debug)]
pub struct Article {
	pub title: String,
	pub url: String,
	pub desc: Option<String>
}
impl Article {
	pub fn init(&self) -> &str {
		println!("{:?}", &self.url);
		&self.url
	}
}