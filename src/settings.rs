#[derive(Debug, Clone)]
pub struct Settings {
	pub split_num: i32,
	pub proxy: String,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			split_num: 16,
			proxy: "".to_string(),
		}
	}
}
