use crate::{settings::Settings, server::Info};

static mut STATUS_INFO: String = String::new();
static mut WAIT_TO_START: Vec<Info> = vec![];
static mut QUIT_REQUEST: bool = false;

static mut SETTINGS: Option<Settings> = None;

pub fn set_settings(new_settings: Settings) {
	unsafe {
		SETTINGS = Some(new_settings);
	}
}

pub fn get_settings() -> Settings {
	unsafe {
		if SETTINGS.is_none() {
			Settings::new()
		} else {
			SETTINGS.clone().unwrap()
		}
	}
}

pub fn set_status_info(info: String) {
	unsafe {
		STATUS_INFO = info;
	}
}

pub fn get_status_info() -> String {
	unsafe {
		STATUS_INFO.clone()
	}
}

pub fn add_wait_to_start(data: Info) {
	unsafe {
		WAIT_TO_START.push(data);
	}
}

pub fn get_wait_to_start() -> Vec<Info> {
	unsafe {
		WAIT_TO_START.clone()
	}
}

pub fn clear_wait_to_start() {
	unsafe {
		WAIT_TO_START = vec![];
	}
}

pub fn get_quit_request() -> bool {
	unsafe {
		QUIT_REQUEST
	}
}

pub fn set_quit_request(q: bool) {
	unsafe {
		QUIT_REQUEST = q;
	}
}
