use std::process::Command;

use aria2_ws::Client;
use futures::executor::block_on;

const SERVER_URL: &str = "ws://127.0.0.1:6800/jsonrpc";
static mut ARIA2C_PROCESS: Option<std::process::Child> = None;

fn get_client() -> Option<Client> {
	match block_on(
		Client::connect(SERVER_URL, None)
	) {
		Ok(client) => Some(client),
		Err(_) => {
			println!("Some Error Occurred");
			None
		}
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

pub fn add_uri(url: String) {
	block_on({
		get_client().unwrap()
		.add_uri(
			vec![url],
			None,
			None,
			None
		)
	}).unwrap();
}
