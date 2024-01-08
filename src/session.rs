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
	update_time: usize,
	running: bool,
	name: String,
}

impl Session {
	pub fn new(url: String) -> Self {
		let parsed_url = Url::parse(&url).unwrap();
		let segments = parsed_url.path_segments().unwrap();
		let name = segments.last().unwrap();
		Self {
			uid: Uuid::new_v4(),
			gid: String::new(),
			url,
			started: false,
			status: None,
			update_time: 0,
			running: false,
			name: name.to_string(),
		}
	}

	pub fn get_uid(&self) -> Uuid {
		self.uid
	}

	pub fn get_name(&self) -> String {
		self.name.clone()
	}

	pub fn get_process(&self) -> f32 {
		if self.started && !self.status.is_none() {
			let status = self.status.clone().unwrap();
			if status.total_length == 0 {
				0.0
			} else {
				(status.completed_length / status.total_length) as f32
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
			self.update_time += 1;
			if self.update_time == 100 {
				self.status = Some(aria2c::get_status(self.gid.clone()));
				self.update_time = 0;
			}
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
		format!("
Save Path: {}
Download Url: {}
{}
			",
			status.dir,
			self.url,
			err_msg
		).trim().to_string()
	}
}
