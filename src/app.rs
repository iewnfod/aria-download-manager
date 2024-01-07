use eframe::{App, egui::{CentralPanel, ScrollArea, ProgressBar, TopBottomPanel, Id, TextEdit}, epaint::ahash::{HashMap, HashMapExt}};
use uuid::Uuid;
use crate::{session::Session, aria2c};

pub struct DownloadManager {
	sessions: HashMap<Uuid, Session>,
	url_input: String,
	info: String,
	wait_to_remove: Vec<Session>,
}

impl DownloadManager {
	fn new_session(&mut self) {
		if !self.url_input.trim().is_empty() {
			let session = Session::new(self.url_input.clone());
			let name = session.get_name();
			self.sessions.insert(session.get_uid(), session);
			self.info = format!("New session to `{}`", name);
		} else {
			self.info = "Target cannot be empty".to_string();
		}
	}
}

impl Default for DownloadManager {
	fn default() -> Self {
		Self {
			sessions: HashMap::new(),
			url_input: "".to_string(),
			info: "Welcome to Aria Download Manager".to_string(),
			wait_to_remove: vec![],
		}
	}
}

impl App for DownloadManager {
	fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
		// 处理内容
		if !self.wait_to_remove.is_empty() {
			for s in self.wait_to_remove.iter() {
				self.sessions.remove(&s.get_uid());
			}
			self.wait_to_remove.clear();
		}

		// 绘制 ui
		TopBottomPanel::top(Id::new("top")).show(ctx, |ui| {
			ui.add_space(5.0);
			ui.horizontal(|ui| {
				ui.add(TextEdit::singleline(&mut self.url_input).hint_text("Target Url"));
				if ui.button("New Session").clicked() {
					self.new_session();
				}
			});
			ui.add_space(5.0);
		});

		CentralPanel::default().show(ctx, |ui| {
			ScrollArea::vertical().show(ui, |ui| {
				for (_uid, session) in self.sessions.iter_mut() {
					ui.horizontal(|ui| {
						ui.label(session.get_name());
						if ui.button("Remove").clicked() {
							self.wait_to_remove.push(session.clone());
						}
					});
					ui.horizontal(|ui| {
						if ui.button("Start").clicked() {
							session.start();
						}
						ui.add(
							ProgressBar::new(session.get_process())
							.text(session.get_speed())
						);
					});
					ui.separator();
				}
			});
		});

		TopBottomPanel::bottom(Id::new("bottom")).show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label(&self.info);
			});
		});
	}

	fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
		aria2c::stop();
	}
}
