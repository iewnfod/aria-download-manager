use std::{collections::HashMap, path::PathBuf, process::Command};

use aria2_ws::Client;
use chrono::{Local, Datelike, Timelike};
use serde::{Serialize, Deserialize};

use crate::{settings::get_app_support_path, session::Session};

const HISTORY_FILE: &str = "history.json";

fn get_history_path() -> PathBuf {
	get_app_support_path()
		.join(HISTORY_FILE)
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct HistorySession {
	url: String,
	webpage: String,
	file: String,
	name: String,
	time: (i32, u32, u32, u32, u32, u32),
}

impl HistorySession {
	pub fn new(url: String, webpage: String, file: String, name: String) -> Self {
		let time = Local::now();
		Self {
			url,
			webpage,
			file,
			name,
			time: (
				time.year(), time.month(), time.day(),
				time.hour(), time.minute(), time.second(),
			),
		}
	}

	pub fn get_url(&self) -> String {
		self.url.clone()
	}

	pub fn get_file(&self) -> String {
		self.file.clone()
	}

	pub fn get_name(&self) -> String {
		self.name.clone()
	}

	pub fn resume(&self, sessions: &mut HashMap<String, Session>, client: Option<Client>) {
		let mut session = Session::new(self.url.clone(), client).unwrap();
		session.start();
		sessions.insert(session.get_uid(), session);
	}

	pub fn get_time(&self) -> String {
		format!("{}-{}-{} {}:{}:{}", self.time.0, self.time.1, self.time.2, self.time.3, self.time.4, self.time.5)
	}

	pub fn get_webpage(&self) -> String {
		self.webpage.clone()
	}

	pub fn open_webpage(&self) {
		let mut command = Command::new("open");
		command.arg(&self.webpage);
		command.spawn().unwrap();
	}
}

impl PartialEq for HistorySession {
	fn eq(&self, other: &Self) -> bool {
		self.url == other.url
		&& self.file == other.file
		&& self.name == other.name
	}
}

impl Ord for HistorySession {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		if self.time == other.time {
			(&self.url, &self.file, &self.name).cmp(&(&other.url, &other.file, &other.name))
		} else {
			self.time.cmp(&other.time)
		}
	}
}

impl PartialOrd for HistorySession  {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
	sessions: HashMap<String, HistorySession>
}

impl Default for History {
	fn default() -> Self {
		Self {
			sessions: HashMap::new()
		}
	}
}

impl History {
	pub fn new() -> Self {
		let f = Self::from_file();
		if f.is_none() {
			Self::default()
		} else {
			f.unwrap()
		}
	}

	pub fn from_file() -> Option<Self> {
		let path = get_history_path();
		if path.exists() {
			let contents = std::fs::read_to_string(path).unwrap();
			match serde_json::from_str(&contents) {
				Ok(f) => f,
				Err(_) => None,
			}
		} else {
			None
		}
	}

	pub fn save(&self) {
		let path = get_history_path();
		let contents = serde_json::to_string_pretty(self).unwrap();
		std::fs::write(path, contents).unwrap();
	}

	pub fn add_session(&mut self, session: Session) {
		let history_session = HistorySession::new(
			session.get_url(),
			session.get_webpage(),
			session.get_file(),
			session.get_name(),
		);
		// 如果和之前的相同，那就不需要重新写一遍文件
		if self.sessions.contains_key(&session.get_uid()) {
			if self.sessions[&session.get_uid()] == history_session {
				return;
			}
		}
		self.sessions.insert(session.get_uid(), history_session);
		self.save();
	}

	pub fn get_sessions(&self) -> Vec<(String, HistorySession)> {
		let mut data = vec![];
		for (uid, s) in self.sessions.iter() {
			data.push((uid.clone(), s.clone()));
		}
		data.sort_by(|a, b| b.1.cmp(&a.1));
		data
	}

	pub fn remove(&mut self, uid: &String) {
		if self.sessions.contains_key(uid) {
			self.sessions.remove(uid);
			self.save();
		}
	}
}
