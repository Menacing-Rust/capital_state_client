use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
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