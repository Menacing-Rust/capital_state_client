#[derive(Debug, Default)]
pub struct Payload {
	author: String,
	message: String
}

impl Payload {
	pub fn new(author: impl Into<String>, message: impl Into<String>) -> Payload {
		let author = author.into();
		let message = message.into();
		Payload { author, message }
	}
}