use eframe::{App, egui::{CentralPanel, ScrollArea, ProgressBar, TopBottomPanel, Id, TextEdit}, epaint::ahash::{HashMap, HashMapExt}};
use uuid::Uuid;
use crate::{session::Session, aria2c, data::{set_status_info, get_status_info, set_split_num}};

pub struct DownloadManager {
	sessions: HashMap<Uuid, Session>,
	url_input: String,
	info: String,
	wait_to_remove: Vec<Session>,
	split_num_input: String,
}

impl DownloadManager {
	fn new_session(&mut self) {
		self.url_input = self.url_input.trim().to_string();
		if !self.url_input.is_empty() {
			let mut session = Session::new(self.url_input.clone());
			session.start();
			let name = session.get_name();
			self.sessions.insert(session.get_uid(), session);
			set_status_info(format!("New session to `{}`", name));
		} else {
			set_status_info("Target url cannot be empty".to_string());
		}
	}

	fn update_setting(&self) {
		set_split_num(self.split_num_input.clone());
	}
}

impl Default for DownloadManager {
	fn default() -> Self {
		Self {
			sessions: HashMap::new(),
			url_input: String::new(),
			info: String::new(),
			wait_to_remove: vec![],
			split_num_input: String::new(),
		}
	}
}

impl App for DownloadManager {
	fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
		// 处理内容
		if !self.wait_to_remove.is_empty() {
			for s in self.wait_to_remove.iter_mut() {
				s.remove();
				self.sessions.remove(&s.get_uid());
			}
			self.wait_to_remove.clear();
		}
		// 获取状态栏数据
		self.info = get_status_info();

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
			ui.horizontal(|ui| {
				ui.add(TextEdit::singleline(&mut self.split_num_input).hint_text("Split Number (16)"));
				if ui.button("Apply").clicked() {
					self.update_setting()
				}
			});
			ui.add_space(5.0);
		});

		CentralPanel::default().show(ctx, |ui| {
			ScrollArea::vertical().show(ui, |ui| {
				for (uid, session) in self.sessions.iter_mut() {
					session.update_status();
					ScrollArea::horizontal().id_source(uid).show(ui, |ui| {
						ui.horizontal(|ui| {
							ui.label(session.get_name());
							if ui.button("Remove").clicked() {
								self.wait_to_remove.push(session.clone());
							}
						});
						ui.horizontal(|ui| {
							if ui.button("Continue").clicked() {
								session.start();
							}
							if ui.button("Pause").clicked() {
								session.pause();
							}
							ui.add(
								ProgressBar::new(session.get_process())
								.text(session.get_speed())
							);
						});
						ui.collapsing("Detailed Information", |ui| {
							ui.label(
								session.get_status()
							);
						});
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
