use std::{thread, collections::HashMap};

use aria2_ws::{Client, TaskOptions};
use futures::executor::block_on;

use crate::{data::{get_settings, set_status_info}, session::Session};

pub const SERVER_URL: &str = "ws://127.0.0.1:6800/jsonrpc";

fn get_options(session: &Session) -> TaskOptions {
	let mut opt = TaskOptions::default();
	let settings = get_settings();
	opt.split = Some(settings.split_num);
	if !settings.proxy.is_empty() {
		opt.all_proxy = Some(settings.proxy.clone());
	}
	opt.header = Some(vec![
		format!("Cookie: {}", session.get_cookie()),
		format!("User-Agent: {}", settings.user_agent),
	]);
	opt.dir = Some(format!("/Users/{}/Downloads", users::get_current_username().unwrap().to_str().unwrap()));
	opt
}

pub fn add_uri(client: &Option<Client>, url: String, target_session: &mut Session) {
	if client.is_none() {
		set_status_info("Client is none. Please try to reconnect.".to_string());
		return;
	}
	let gid = match block_on(
		client.clone().unwrap()
		.add_uri(
			vec![url],
			Some(get_options(&target_session)),
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

pub fn remove(client: &Option<Client>, gid: String) {
	if client.is_none() {
		return;
	}
	pause(client, gid.clone());
	thread::scope(|s| {
		s.spawn(|| {
			let _ = block_on(
				client.clone().unwrap()
				.remove(&gid)
			);
		});
	});
}

pub fn pause(client: &Option<Client>, gid: String) {
	if client.is_none() {
		return;
	}
	thread::scope(|s| {
		s.spawn(|| {
			let _ = block_on(
				client.clone().unwrap()
				.pause(&gid)
			);
		});
	});
}

pub fn unpause(client: &Option<Client>, gid: String) {
	if client.is_none() {
		return;
	}
	thread::scope(|s| {
		s.spawn(|| {
			let _ = block_on(
				client.clone().unwrap()
				.unpause(&gid)
			);
		});
	});
}

pub fn get_status(client: &Option<Client>, gid: String, target_session: &mut Session) {
	if client.is_none() {
		return;
	}
	thread::scope(|s| {
		s.spawn(|| {
			let status = block_on(
				client.clone().unwrap()
				.tell_status(&gid)
			).unwrap();
			target_session.update_status_handler(status);
		});
	});
}

pub fn get_active(client: &Option<Client>, sessions: &mut HashMap<String, Session>) {
	if client.is_none() {
		return;
	}
	thread::scope(|s| {
		s.spawn(|| {
			let active = block_on(
				client.clone().unwrap()
				.tell_active()
			).unwrap();
			for status in active {
				if !sessions.contains_key(&status.gid) {
					let url = status.files[0].uris[0].clone().uri;
					let mut session = Session::new(url, client.clone()).unwrap();
					session.start_handler(status.gid.clone());
					sessions.insert(session.get_uid(), session);
				}
			}
		});
	});
}
