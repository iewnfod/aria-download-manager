use std::thread;

use aria2_ws::{Client, TaskOptions};
use futures::executor::block_on;

use crate::{data::{get_settings, set_status_info}, session::Session};

const SERVER_URL: &str = "ws://127.0.0.1:6800/jsonrpc";

static mut CLIENT: Option<Client> = None;

fn get_client() -> Option<Client> {
	unsafe {
		if CLIENT.is_none() {
			CLIENT = match block_on(
				Client::connect(SERVER_URL, None)
			) {
				Ok(c) => Some(c),
				Err(e) => {
					set_status_info(format!("Connection Error: {:?}", e.to_string()));
					None
				}
			};
		}
		CLIENT.clone()
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
	if get_client().is_none() {
		return;
	}
	let gid = match block_on(
		get_client().unwrap()
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
	if get_client().is_none() {
		return;
	}
	pause(gid.clone());
	thread::scope(|s| {
		s.spawn(|| {
			let _ = block_on(
				get_client().unwrap()
				.remove(&gid)
			);
		});
	});
}

pub fn pause(gid: String) {
	if get_client().is_none() {
		return;
	}
	thread::scope(|s| {
		s.spawn(|| {
			let _ = block_on(
				get_client().unwrap()
				.pause(&gid)
			);
		});
	});
}

pub fn unpause(gid: String) {
	if get_client().is_none() {
		return;
	}
	thread::scope(|s| {
		s.spawn(|| {
			let _ = block_on(
				get_client().unwrap()
				.unpause(&gid)
			);
		});
	});
}

pub fn get_status(gid: String, target_session: &mut Session) {
	if get_client().is_none() {
		return;
	}
	thread::scope(|s| {
		s.spawn(|| {
			let status = block_on(
				get_client().unwrap()
				.tell_status(&gid)
			).unwrap();
			target_session.update_status_handler(status);
		});
	});
}
