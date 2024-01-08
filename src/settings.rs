pub struct Settings {
	pub split_num: String,
	pub proxy: bool,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			split_num: String::new(),
			proxy: false,
		}
	}
}
