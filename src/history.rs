use std::{path::PathBuf, collections::HashMap};

use serde::{Serialize, Deserialize};

use crate::{settings::get_app_support_path, session::Session};

const HISTORY_FILE: &str = "history.json";

fn get_history_path() -> PathBuf {
	get_app_support_path()
		.join(HISTORY_FILE)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistorySession {
	url: String,
	file: String,
	name: String,
}

impl HistorySession {
	pub fn get_url(&self) -> String {
		self.url.clone()
	}

	pub fn get_file(&self) -> String {
		self.file.clone()
	}

	pub fn get_name(&self) -> String {
		self.name.clone()
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
		let history_session =  HistorySession {
			url: session.get_url(),
			file: session.get_file(),
			name: session.get_name(),
		};
		self.sessions.insert(session.get_uid(), history_session);
		self.save();
	}

	pub fn get_sessions(&self) -> HashMap<String, HistorySession> {
		self.sessions.clone()
	}
}
