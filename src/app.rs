use std::time::Duration;

use eframe::{App, egui::{CentralPanel, ScrollArea, ProgressBar, TopBottomPanel, Id, TextEdit, CollapsingHeader, Grid, DragValue}, epaint::ahash::{HashMap, HashMapExt}};
use uuid::Uuid;
use crate::{session::Session, data::{set_status_info, get_status_info, get_wait_to_start, clear_wait_to_start, set_settings, get_quit_request}, settings::Settings};

pub struct DownloadManager {
	sessions: HashMap<Uuid, Session>,
	url_input: String,
	info: String,
	wait_to_remove: Vec<Session>,
	settings: Settings,
}

impl DownloadManager {
	fn new_session(&mut self) {
		self.url_input = self.url_input.trim().to_string();
		if !self.url_input.is_empty() {
			let mut session = match Session::new(self.url_input.clone()) {
				Ok(s) => s,
				Err(_) => return,
			};
			session.start();
			let name = session.get_name();
			self.sessions.insert(session.get_uid(), session);
			set_status_info(format!("New session to `{}`", name));
		} else {
			set_status_info("Target url cannot be empty".to_string());
		}
	}

	fn apply_settings(&self) {
		set_settings(self.settings.clone());
		set_status_info("Apply Settings".to_string());
	}

	fn remove_all(&mut self) {
		for (_uid, session) in self.sessions.iter_mut() {
			session.remove();
		}
	}
}

impl Default for DownloadManager {
	fn default() -> Self {
		Self {
			sessions: HashMap::new(),
			url_input: String::new(),
			info: String::new(),
			wait_to_remove: vec![],
			settings: Settings::default(),
		}
	}
}

impl App for DownloadManager {
	fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
		// 判断是否需要退出
		if get_quit_request() {
			self.remove_all();
			frame.close();
		}
		// 处理内容
		if !self.wait_to_remove.is_empty() {
			for s in self.wait_to_remove.iter_mut() {
				s.remove();
				self.sessions.remove(&s.get_uid());
			}
			self.wait_to_remove.clear();
		}
		// 读取待添加的任务
		let wait_to_start = get_wait_to_start();
		for u in wait_to_start.iter() {
			self.url_input = u.to_string();
			self.new_session();
		}
		clear_wait_to_start();
		// 获取状态栏数据
		self.info = get_status_info();
		// 判断是否需要刷新
		let mut all_finished = true;

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
				for (uid, session) in self.sessions.iter_mut() {
					session.update_status();
					if !session.is_completed() {
						all_finished = false;
					}
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
						CollapsingHeader::new("Detailed Information").id_source(uid.to_string() + "detail")
						.show(ui, |ui| {
							ui.label(session.get_status());
						});
					});
					ui.separator();
				}
				ui.add_space(ctx.used_size().y);
			});
		});

		TopBottomPanel::bottom(Id::new("bottom")).show(ctx, |ui| {
			ui.add_space(5.0);
			ui.horizontal(|ui| {
				ui.label(&self.info);
			});
			ui.collapsing("Settings", |ui| {
				ScrollArea::vertical().show(ui, |ui| {
					Grid::new("settings")
					.num_columns(2)
					.show(ui, |ui| {
						ui.label("Connection Number");
						ui.add(DragValue::new(&mut self.settings.split_num).clamp_range(1..=64));
						ui.end_row();

						ui.label("All Proxy Url");
						ui.text_edit_singleline(&mut self.settings.proxy);
						ui.end_row();
					});
					if ui.button("Apply").clicked() {
						self.apply_settings();
					}
				});
			});
			ui.add_space(5.0);
		});

		// 如果还有在下载的东西，那就刷新页面
		if !all_finished {
			ctx.request_repaint();
		} else {
			ctx.request_repaint_after(Duration::from_secs(1));
		}
	}

	fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
		println!("Quit");
	}
}
