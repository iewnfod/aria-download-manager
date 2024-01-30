use std::{path::Path, time::Instant, process::Command};

use aria2_ws::{response::Status, Client};
use url::Url;
use uuid::Uuid;

use crate::{aria2c, data::set_status_info, server::Cookie};

const UNITS: [&str; 5] = [
	"B/s",
	"KB/s",
	"MB/s",
	"GB/s",
	"TB/s"
];

#[derive(Clone)]
pub struct Session {
	uid: String,
	gid: String,
	url: String,
	webpage: String,
	status: Option<Status>,
	update_time: Instant,
	update_frequency: u128,
	running: bool,
	name: String,
	cookie: Vec<Cookie>,
	referrer: String,
	client: Option<Client>,
}

impl Session {
	pub fn new(url: String, client: Option<Client>) -> Result<Self, ()> {
		let parsed_url = match Url::parse(&url) {
			Ok(u) => u,
			Err(_) => {
				set_status_info(format!("Invalid Url `{}`", &url));
				return Err(());
			}
		};
		let segments = match parsed_url.path_segments() {
			Some(s) => s,
			_ => {
				set_status_info(format!("Failed to Solve Url `{}`", &url));
				return Err(());
			}
		};

		let name = segments.last().unwrap();

		Ok(Self {
			uid: Uuid::new_v4().to_string(),
			gid: String::new(),
			url,
			webpage: String::new(),
			status: None,
			update_time: Instant::now(),
			update_frequency: 100,
			running: false,
			name: name.to_string(),
			cookie: vec![],
			referrer: String::new(),
			client,
		})
	}

	pub fn get_uid(&self) -> String {
		self.uid.clone()
	}

	pub fn get_url(&self) -> String {
		self.url.clone()
	}

	pub fn get_file(&self) -> String {
		if !self.status.is_none() {
			self.status.clone().unwrap().files[0].path.clone()
		} else {
			String::new()
		}
	}

	pub fn get_name(&self) -> String {
		let mut result = self.url.clone();
		if !self.name.is_empty() {
			result = self.name.clone();
		}
		if !self.status.is_none() {
			let files = self.status.clone().unwrap().files;
			let mut results = vec![];
			for file in files.iter() {
				if let Some(name_os) = Path::new(&file.path).file_name() {
					if let Some(name) = name_os.to_str() {
						results.push(name.to_string());
					}
				}
			}
			if !results.is_empty() {
				result = results.join(", ");
			}
		}
		result
	}

	pub fn get_process(&self) -> f32 {
		if !self.gid.is_empty() && !self.status.is_none() {
			let status = self.status.clone().unwrap();
			if status.total_length == 0 {
				0.0
			} else {
				status.completed_length as f32 / status.total_length as f32
			}
		} else {
			0.0
		}
	}

	pub fn get_speed(&self) -> String {
		if self.is_completed() {
			return "Completed!".to_string();
		}
		if self.get_verified_length() != 0 && !self.is_verified() {
			return "Verifying...".to_string();
		}
		if !self.gid.is_empty() && !self.status.is_none() && self.running {
			let speed = self.status.clone().unwrap().download_speed;
			let mut result_speed = speed as f32;
			let mut unit_index = 0;
			while result_speed > 1024.0 && unit_index < UNITS.len() {
				result_speed /= 1024.0;
				unit_index += 1;
			}
			result_speed.to_string() + UNITS[unit_index]
		} else {
			"0.0B/s".to_string()
		}
	}

	pub fn start(&mut self) {
		if !self.running {
			if self.gid.is_empty() {
				aria2c::add_uri(&self.client.clone(), self.url.clone(), self);
			} else {
				self.unpause();
			}
		}
	}

	pub fn start_handler(&mut self, gid: String) {
		self.gid = gid.clone();
		self.uid = gid.clone();
		self.running = true;
		set_status_info(format!("Start `{}`", self.get_name()));
	}

	pub fn remove(&mut self) {
		aria2c::remove(&self.client, self.gid.clone());
		self.running = false;
		set_status_info(format!("Remove `{}`", self.get_name()));
	}

	pub fn pause(&mut self) {
		aria2c::pause(&self.client, self.gid.clone());
		self.running = false;
		set_status_info(format!("Pause `{}`", self.get_name()));
	}

	pub fn unpause(&mut self) {
		aria2c::unpause(&self.client, self.gid.clone());
		self.running = true;
		set_status_info(format!("Continue `{}`", self.get_name()));
	}

	pub fn update_status(&mut self) {
		if !self.gid.is_empty() {
			if self.update_time.elapsed().as_millis() > self.update_frequency {
				aria2c::get_status(&self.client.clone(), self.gid.clone(), self);
				self.update_time = Instant::now();
			}
		}
	}

	pub fn update_status_handler(&mut self, new_status: Status) {
		self.status = Some(new_status);
	}

	fn get_verified_length(&self) -> u64 {
		if self.status.is_none() {
			0
		} else {
			if let Some(verified_length) = self.status.clone().unwrap().verified_length {
				verified_length
			} else {
				0
			}
		}
	}

	fn is_verified(&self) -> bool {
		if self.status.is_none() {
			return false;
		}
		self.get_verified_length() == self.status.clone().unwrap().total_length
	}

	pub fn is_completed(&self) -> bool {
		if let Some(status) = self.status.clone() {
			if !self.gid.is_empty() {
				status.completed_length == status.total_length
				&& status.completed_length != 0
			} else {
				false
			}
		} else {
			false
		}
	}

	pub fn is_error(&self) -> bool {
		if let Some(status) = self.status.clone() {
			if status.error_code.is_none() {
				false
			} else {
				if status.error_code.unwrap() == "0" {
					true
				} else {
					false
				}
			}
		} else {
			false
		}
	}

	pub fn open(&self) {
		if self.status.is_none() {
			set_status_info("This session has not started".to_string());
		}
		let mut command = Command::new("open");
		command.arg(&self.status.clone().unwrap().files[0].path);
		command.spawn().unwrap();
	}

	pub fn open_folder(&self) {
		if self.status.is_none() {
			set_status_info("This session has not started".to_string());
		}
		let mut command = Command::new("open");
		command.arg(&self.status.clone().unwrap().dir);
		command.spawn().unwrap();
	}

	pub fn set_cookie(&mut self, cookie: Vec<Cookie>) {
		self.cookie = cookie;
	}

	pub fn get_cookie(&self) -> String {
		self.cookie.iter().map(|c| c.to_string()).collect::<Vec<String>>().join("; ")
	}

	pub fn set_referer(&mut self, referrer: String) {
		self.referrer = referrer;
	}

	pub fn get_referer(&self) -> String {
		self.referrer.clone()
	}

	pub fn set_webpage(&mut self, webpage: String) {
		self.webpage = webpage;
	}

	pub fn get_webpage(&self) -> String {
		self.webpage.clone()
	}

	pub fn get_gid(&self) -> String {
		self.gid.clone()
	}

	pub fn get_complete_data(&self) -> String {
		if self.status.is_none() {
			String::new()
		} else {
			let status = self.status.clone().unwrap();
			format!("{}% ( {} / {} )",
				status.completed_length as f32 / status.total_length as f32 * 100.0,
				status.completed_length, status.total_length,
			)
		}
	}

	pub fn get_verified_data(&self) -> String {
		if self.status.is_none() {
			String::new()
		} else {
			let status = self.status.clone().unwrap();
			format!("{}% ( {} / {} )",
				self.get_verified_length() as f32 / status.total_length as f32 * 100.0,
				self.get_verified_length(), status.total_length,
			)
		}
	}

	pub fn get_connections_num(&self) -> u64 {
		if self.status.is_none() {
			0
		} else {
			self.status.clone().unwrap().connections
		}
	}

	pub fn get_pieces_num(&self) -> u64 {
		if self.status.is_none() {
			0
		} else {
			self.status.clone().unwrap().num_pieces
		}
	}

	pub fn get_pieces_length(&self) -> u64 {
		if self.status.is_none() {
			0
		} else {
			self.status.clone().unwrap().piece_length
		}
	}

	pub fn get_error_code(&self) -> String {
		if self.status.is_none() {
			String::new()
		} else {
			self.status.clone().unwrap().error_code.unwrap_or("0".to_string())
		}
	}

	pub fn get_error_msg(&self) -> String {
		if self.status.is_none() {
			String::new()
		} else {
			self.status.clone().unwrap().error_message.unwrap_or("".to_string())
		}
	}

	pub fn set_client(&mut self, client: Option<Client> ) {
		self.client = client;
	}
}
