use std::thread;

use aria2_ws::{Client, TaskOptions};
use futures::executor::block_on;

use crate::{data::{get_settings, set_status_info}, session::Session};

const SERVER_URL: &str = "ws://127.0.0.1:6800/jsonrpc";

static mut CLIENT: Option<Client> = None;

fn get_client() -> Client {
	unsafe {
		while CLIENT.is_none() {
			CLIENT = match block_on(
				Client::connect(SERVER_URL, None)
			) {
				Ok(c) => Some(c),
				Err(_e) => {
					// set_status_info(format!("Connect Error: {:?}", e));
					None
				}
			};
		}
		CLIENT.clone().unwrap()
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

pub fn add_uri(url: String, target_session: &mut Session) {
	let gid = match block_on(
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
		}
	};
	target_session.start_handler(gid);
}

pub fn remove(gid: String) {
	pause(gid.clone());
	thread::scope(|s| {
		s.spawn(|| {
			let _ = block_on(
				get_client()
				.remove(&gid)
			);
		});
	});
}

pub fn pause(gid: String) {
	thread::scope(|s| {
		s.spawn(|| {
			let _ = block_on(
				get_client()
				.pause(&gid)
			);
		});
	});
}

pub fn unpause(gid: String) {
	thread::scope(|s| {
		s.spawn(|| {
			let _ = block_on(
				get_client()
				.unpause(&gid)
			);
		});
	});
}

pub fn get_status(gid: String, target_session: &mut Session) {
	thread::scope(|s| {
		s.spawn(|| {
			let status = block_on(
				get_client()
				.tell_status(&gid)
			).unwrap();
			target_session.update_status_handler(status);
		});
	});
}
