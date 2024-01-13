use std::{path::PathBuf, fs};

use serde::{Serialize, Deserialize};
use users::os::unix::UserExt;

const BUNDLE_ID: &str = "com.iewnfod.ariadownloadmanager";
const SETTINGS_FILE: &str = "settings.json";

fn get_save_path() -> PathBuf {
	let user = users::get_user_by_uid(users::get_current_uid()).unwrap();
	let path = user.home_dir();
	path.join("Library")
		.join("Application Support")
		.join(BUNDLE_ID)
		.join(SETTINGS_FILE)
		.to_path_buf()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
	pub split_num: i32,
	pub proxy: String,
	save_path: PathBuf,
}

impl Default for Settings {
	fn default() -> Self {
		let save_path: PathBuf = get_save_path();
		Self {
			split_num: 16,
			proxy: "".to_string(),
			save_path
		}
	}
}

impl Settings {
	pub fn new() -> Self {
		let save_setting = Self::from_save();
		if save_setting.is_none() {
			println!("default settings");
			Self::default()
		} else {
			println!("settings from save");
			save_setting.unwrap()
		}
	}

	pub fn from_save() -> Option<Self> {
		let save_path = get_save_path();
		if save_path.exists() {
			let string_data = match std::fs::read_to_string(save_path) {
				Ok(s) => s,
				Err(_) => "".to_string()
			};
			let value = serde_json::from_str(&string_data).unwrap();
			match serde_json::from_value::<Self>(value) {
				Ok(j) => Some(j),
				Err(_) => None
			}
		} else {
			None
		}
	}

	pub fn save(&self) {
		let json = serde_json::to_string_pretty(self).unwrap();
		if !self.save_path.exists() {
			fs::create_dir_all(self.save_path.parent().unwrap()).unwrap();
			fs::File::create(&self.save_path).unwrap();
		}
		fs::write(&self.save_path, json).unwrap();
	}
}
