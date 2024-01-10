use std::{path::Path, time::Instant};

use aria2_ws::response::Status;
use url::Url;
use uuid::Uuid;

use crate::{aria2c, data::set_status_info};

const UNITS: [&str; 5] = [
	"B/s",
	"KB/s",
	"MB/s",
	"GB/s",
	"TB/s"
];

#[derive(Debug, Clone)]
pub struct Session {
	uid: Uuid,
	gid: String,
	url: String,
	started: bool,
	status: Option<Status>,
	update_time: Instant,
	update_frequency: u128,
	running: bool,
	name: String,
}

impl Session {
	pub fn new(url: String) -> Result<Self, ()> {
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
			uid: Uuid::new_v4(),
			gid: String::new(),
			url,
			started: false,
			status: None,
			update_time: Instant::now(),
			update_frequency: 100,
			running: false,
			name: name.to_string(),
		})
	}

	pub fn get_uid(&self) -> Uuid {
		self.uid
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
		if self.started && !self.status.is_none() {
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
		if self.started && !self.status.is_none() {
			let speed = self.status.clone().unwrap().download_speed;
			let mut result_speed = speed as f32;
			let mut unit_index = 0;
			while result_speed > 1024.0 && unit_index < UNITS.len() {
				result_speed /= 1024.0;
				unit_index += 1;
			}
			result_speed.to_string() + UNITS[unit_index]
		} else {
			0.0.to_string()
		}
	}

	pub fn start(&mut self) {
		if !self.running {
			if !self.started {
				self.gid = aria2c::add_uri(self.url.clone());
				self.started = true;
				self.running = true;
				set_status_info(format!("Start `{}`", self.get_name()));
			} else {
				self.unpause();
			}
		}
	}

	pub fn remove(&mut self) {
		aria2c::remove(self.gid.clone());
		self.running = false;
		set_status_info(format!("Remove `{}`", self.get_name()));
	}

	pub fn pause(&mut self) {
		aria2c::pause(self.gid.clone());
		self.running = false;
		set_status_info(format!("Pause `{}`", self.get_name()));
	}

	pub fn unpause(&mut self) {
		aria2c::unpause(self.gid.clone());
		self.running = true;
		set_status_info(format!("Continue `{}`", self.get_name()));
	}

	pub fn update_status(&mut self) {
		if !self.gid.is_empty() {
			if self.update_time.elapsed().as_millis() > self.update_frequency {
				aria2c::get_status(self.gid.clone(), self);
				// self.status = Some(aria2c::get_status(self.gid.clone()));
				self.update_time = Instant::now();
			}
		}
	}

	pub fn update_status_handler(&mut self, new_status: Status) {
		self.status = Some(new_status);
	}

	pub fn is_completed(&self) -> bool {
		if let Some(status) = self.status.clone() {
			if self.started {
				status.completed_length == status.total_length && status.completed_length != 0
			} else {
				false
			}
		} else {
			false
		}
	}

	pub fn get_status(&self) -> String {
		if self.gid.is_empty() || self.status.is_none() {
			return "This session has not started!".to_string();
		}
		let status = self.status.clone().unwrap();
		let mut err_msg = String::new();
		if let Some(err) = status.error_code {
			err_msg = format!("
Error Code: {}
Error Message: {}
			", err, status.error_message.unwrap()).trim().to_string();
			if err == "0" {
				err_msg = String::new();
			}
		}

		let mut files = vec![];
		for file in status.files {
			files.push(file.path);
		}
		format!("
Download Url: {}
Save Dir: {}
Files: {}
Completed: {}% ( {} / {} )
{}
			",
			self.url,
			status.dir,
			files.join("\n"),
			status.completed_length as f32 / status.total_length as f32 * 100.0,
			status.completed_length, status.total_length,
			err_msg
		).trim().to_string()
	}
}
