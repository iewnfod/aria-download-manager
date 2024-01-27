use std::{path::PathBuf, fs};

use serde::{Serialize, Deserialize};
use users::os::unix::UserExt;

const BUNDLE_ID: &str = "com.iewnfod.ariadownloadmanager";
const SETTINGS_FILE: &str = "settings.json";

pub fn get_app_support_path() -> PathBuf {
	let user = users::get_user_by_uid(users::get_current_uid()).unwrap();
	let path = user.home_dir();
	path.join("Library")
		.join("Application Support")
		.join(BUNDLE_ID)
}

fn get_save_path() -> PathBuf {
	get_app_support_path()
		.join(SETTINGS_FILE)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
	pub split_num: i32,
	pub proxy: String,
	pub user_agent: String,
	pub custom_theme: bool,
	pub dark_mode: bool,
	pub close_after_seconds: u64,
	save_path: PathBuf,
}

impl Default for Settings {
	fn default() -> Self {
		let save_path: PathBuf = get_save_path();
		Self {
			split_num: 16,
			proxy: "".to_string(),
			user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
			custom_theme: false,
			dark_mode: false,
			close_after_seconds: 0,
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

impl PartialEq for Settings {
	fn eq(&self, other: &Self) -> bool {
		self.split_num == other.split_num
		&& self.proxy == other.proxy
		&& self.user_agent == other.user_agent
		&& self.dark_mode == other.dark_mode
	}

	fn ne(&self, other: &Self) -> bool {
		!self.eq(other)
	}
}
