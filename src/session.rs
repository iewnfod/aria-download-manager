use uuid::Uuid;

use crate::aria2c;

#[derive(Debug, Clone)]
pub struct Session {
	uid: Uuid,
	url: String,
	started: bool,
}

impl Session {
	pub fn new(url: String) -> Self {
		Self {
			uid: Uuid::new_v4(),
			url,
			started: false,
		}
	}

	pub fn get_uid(&self) -> Uuid {
		self.uid
	}

	pub fn get_name(&self) -> String {
		self.url.clone()
	}

	pub fn get_process(&self) -> f32 {
		if self.started {
			1.0
		} else {
			0.0
		}
	}

	pub fn get_speed(&self) -> String {
		if self.started {
			1.0.to_string()
		} else {
			0.0.to_string()
		}
	}

	pub fn start(&mut self) {
		if !self.started {
			aria2c::add_uri(self.url.clone());
			self.started = true;
		}
	}
}
