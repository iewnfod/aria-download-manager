use std::process::Command;

use aria2_ws::{Client, response::Status, TaskOptions};
use futures::executor::block_on;

use crate::data::{get_settings, set_status_info};

const SERVER_URL: &str = "ws://127.0.0.1:6800/jsonrpc";
static mut ARIA2C_PROCESS: Option<std::process::Child> = None;

static mut CLIENT: Option<Client> = None;

fn get_client() -> Client {
	unsafe {
		while CLIENT.is_none() {
			CLIENT = Some(
				block_on(
					Client::connect(SERVER_URL, None)
				).unwrap()
			);
		}
		CLIENT.clone().unwrap()
	}
}

pub fn startup() {
	let mut cmd = Command::new("aria2c");
	cmd.arg("--enable-rpc");
	let process = cmd.spawn().unwrap();
	unsafe { ARIA2C_PROCESS = Some(process) };
}

pub fn stop() {
	println!("Stop Aria2c");
	unsafe {
		if let Some(process) = &mut ARIA2C_PROCESS {
			process.kill().unwrap();
		}
	}
}

fn get_options() -> TaskOptions {
	let mut opt = TaskOptions::default();
	let settings = get_settings();
	opt.split = Some(settings.split_num);
	opt.all_proxy = Some(settings.proxy.clone());
	opt.dir = Some(format!("/Users/{}/Downloads", users::get_current_username().unwrap().to_str().unwrap()));
	opt
}

pub fn add_uri(url: String) -> String {
	match
	block_on(
		get_client()
		.add_uri(
			vec![url],
			Some(get_options()),
			None,
			None
		)
	) {
		Ok(gid) => gid,
		Err(msg) => {
			set_status_info(format!("{}", msg));
			String::new()
		},
	}
}

pub fn remove(gid: String) {
	pause(gid.clone());
	match block_on(
		get_client()
		.remove(gid.as_str())
	) {
		Ok(_) => (),
		Err(_) => (),
	}
}

pub fn pause(gid: String) {
	match block_on(
		get_client()
		.pause(gid.as_str())
	) {
		Ok(_) => (),
		Err(_) => (),
	}
}

pub fn unpause(gid: String) {
	match block_on(
		get_client()
		.unpause(gid.as_str())
	) {
		Ok(_) => (),
		Err(_) => (),
	}
}

pub fn get_status(gid: String) -> Status {
	let result = block_on(
		get_client()
		.tell_status(gid.as_str())
	).unwrap();
	result
}
