use std::{collections::HashMap, time::{Duration, Instant}};

use aria2_ws::Client;
use eframe::{App, egui::{CentralPanel, CollapsingHeader, DragValue, Grid, Id, ProgressBar, ScrollArea, TextEdit, TopBottomPanel}};
use futures::executor::block_on;
use crate::{aria2c::{self, SERVER_URL}, data::{clear_wait_to_start, get_focus_request, get_global_fonts, get_global_style, get_quit_request, get_settings, get_settings_update, get_status_info, get_visual_dark, get_wait_to_start, set_focus_request, set_settings, set_settings_update, set_status_info, set_visual_dark}, history::History, server::Info, session::Session, settings::Settings};

pub struct DownloadManager {
	sessions: HashMap<String, Session>,
	url_input: String,
	info: String,
	wait_to_remove: Vec<Session>,
	settings: Settings,
	show_history: bool,
	history_sessions: History,
	settings_changed: bool,
	client: Option<Client>,
	tell_active_time: Instant,
}

impl DownloadManager {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		// 创建实例
		let mut obj = Self::default();
		// 加载字体
		cc.egui_ctx.set_fonts(get_global_fonts());
		// 加载样式
		cc.egui_ctx.set_style(get_global_style());
		// 更新连接
		obj.update_client();
		// 返回
		obj
	}

	fn new_session(&mut self, data: Info) {
		let url = data.download_url.clone().trim().to_string();
		if !url.is_empty() {
			let mut session = match Session::new(url.clone(), self.client.clone()) {
				Ok(s) => s,
				Err(_) => return,
			};
			session.set_cookie(data.download_cookie);
			session.set_webpage(data.webpage_url);
			session.start();
			let name = session.get_name();
			self.sessions.insert(session.get_uid(), session);
			set_status_info(format!("New session to `{}`", name));
		} else {
			set_status_info("Target url cannot be empty".to_string());
		}
	}

	fn apply_settings(&mut self) {
		// 同步设置
		set_settings(self.settings.clone());
		// 保存设置
		self.settings.save();
		// 标记改变
		self.settings_changed = true;
		// 提示信息
		set_status_info("Apply Settings".to_string());
	}

	fn remove_all(&mut self) {
		for (_uid, session) in self.sessions.iter_mut() {
			session.remove();
		}
	}

	fn update_session_client(&mut self) {
		for (_uid, session) in self.sessions.iter_mut() {
			session.set_client(self.client.clone());
		}
	}

	fn update_client(&mut self) {
		if let Some(client) = &self.client {
			let status = match block_on(client.get_global_stat()) {
				Ok(_) => true,
				Err(_) => false,
			};
			if status {
				set_status_info("Connect to aria2 successfully".to_string());
				self.update_session_client();
				return;
			}
		}
		// 获取 client
		self.client = match block_on(
			Client::connect(SERVER_URL, None)
		) {
			Ok(c) => {
				set_status_info("Connect to aria2 successfully".to_string());
				Some(c)
			},
			Err(e) => {
				set_status_info(format!("Connection Error: {:?}", e.to_string()));
				None
			}
		};
		// 更新所有 session 的 client
		self.update_session_client();
	}
}

impl Default for DownloadManager {
	fn default() -> Self {
		Self {
			sessions: HashMap::new(),
			url_input: String::new(),
			info: String::new(),
			wait_to_remove: vec![],
			settings: Settings::new(),
			show_history: false,
			history_sessions: History::new(),
			settings_changed: false,
			client: None,
			tell_active_time: Instant::now(),
		}
	}
}

impl App for DownloadManager {
	fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
		// 如果设置修改了，那就重新设置字体以及主题
		// 或者 data 中请求更新了，那就需要覆盖设置并重载主题
		if self.settings_changed || get_settings_update() {
			ctx.set_fonts(get_global_fonts());
			ctx.set_style(get_global_style());
			self.settings = get_settings();
			self.settings.save();
			self.settings_changed = false;
			// 完成更新请求
			set_settings_update(false);
		}
		let visual_dark = ctx.style().visuals.dark_mode;
		if visual_dark != get_visual_dark() {
			set_visual_dark(visual_dark);
		}
		// 更新 sessions
		if self.tell_active_time.elapsed().as_secs() > 1 {
			aria2c::get_active(&self.client, &mut self.sessions);
			self.tell_active_time = Instant::now();
		}
		// 判断是否需要退出
		if get_quit_request() {
			println!("Quit");
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
			self.new_session(u.clone());
		}
		clear_wait_to_start();
		// 获取状态栏数据
		self.info = get_status_info();
		// 判断是否需要刷新
		let mut all_finished = true;

		// 绘制 ui
		TopBottomPanel::top(Id::new("top")).show(ctx, |ui| {
			ui.add_space(5.0);
			ScrollArea::horizontal().show(ui, |ui| {
				ui.horizontal(|ui| {
					ui.add(TextEdit::singleline(&mut self.url_input).hint_text("Target Url"));
					if ui.button("New Session").clicked() {
						self.new_session(Info::with_download_url(self.url_input.clone()));
					}
					ui.checkbox(&mut self.show_history, "Show History");
					if ui.button("Reconnect Aria2").clicked() {
						self.update_client();
					}
				});
			});
			ui.add_space(5.0);
		});

		CentralPanel::default().show(ctx, |ui| {
			ScrollArea::vertical().show(ui, |ui| {
				for (uid, session) in self.sessions.iter_mut() {
					session.update_status();
					self.history_sessions.add_session(session.clone());
					if !session.is_completed() {
						all_finished = false;
					}
					ScrollArea::horizontal().id_source(uid).show(ui, |ui| {
						ui.horizontal(|ui| {
							ui.label(session.get_name());
							if ui.button("Remove").clicked() {
								self.wait_to_remove.push(session.clone());
							}
							if ui.button("Open").clicked() {
								session.open();
							}
							if ui.button("Open Folder").clicked() {
								session.open_folder();
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
						CollapsingHeader::new("Detailed Information")
						.id_source(uid.to_string() + "detail")
						.show(ui, |ui| {
							Grid::new(session.get_uid() + "grid")
							.num_columns(2)
							.show(ui, |ui| {
								ui.label("Gid");
								ui.label(session.get_gid());
								ui.end_row();

								ui.label("Download Url");
								ui.label(session.get_url());
								ui.end_row();

								ui.label("Webpage url");
								ui.label(session.get_webpage());
								ui.end_row();

								ui.label("File");
								ui.label(session.get_file());
								ui.end_row();

								ui.label("Completed");
								ui.label(session.get_complete_data());
								ui.end_row();

								ui.label("Verified");
								ui.label(session.get_verified_data());
								ui.end_row();

								ui.label("Connection Number");
								ui.label(session.get_connections_num().to_string());
								ui.end_row();

								ui.label("Pieces");
								ui.label(format!("{}B * {}", session.get_pieces_length(), session.get_pieces_num()));
								ui.end_row();

								if session.is_error() {
									ui.label("Error Code");
									ui.label(session.get_error_code());
									ui.end_row();

									ui.label("Error Message");
									ui.label(session.get_error_msg());
									ui.end_row();
								}
							});
						});
					});
					ui.separator();
				}
				// 历史记录
				if self.show_history {
					for (uid, session) in self.history_sessions.get_sessions() {
						if self.sessions.contains_key(&uid) {
							continue;
						}
						ScrollArea::horizontal().id_source(format!("{}scroll", &uid))
						.show(ui, |ui| {
							ui.horizontal(|ui| {
								ui.label(session.get_name());
								if ui.button("Open in Browser").clicked() {
									session.open_webpage();
								}
								if ui.button("Resume").clicked() {
									session.resume(&mut self.sessions, self.client.clone());
								}
								if ui.button("Remove").clicked() {
									self.history_sessions.remove(&uid.clone());
								}
							});
							CollapsingHeader::new("Detailed Information")
							.id_source(format!("{}history", &uid))
							.show(ui, |ui| {
								Grid::new(format!("{}grid", &uid))
								.num_columns(2)
								.show(ui, |ui| {
									ui.label("File");
									ui.label(session.get_file());
									ui.end_row();

									ui.label("Download Url");
									ui.label(session.get_url());
									ui.end_row();

									ui.label("Webpage Url");
									ui.label(session.get_webpage());
									ui.end_row();

									ui.label("Start Time");
									ui.label(session.get_time());
									ui.end_row();
								});
							});
						});
					}
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

						ui.label("User Agent");
						ui.text_edit_singleline(&mut self.settings.user_agent);
						ui.end_row();

						ui.label("Custom Theme");
						ui.checkbox(&mut self.settings.custom_theme, "Enable");
						ui.end_row();

						if self.settings.custom_theme {
							ui.label("Dark Mode");
							ui.checkbox(&mut self.settings.dark_mode, "Enable");
							ui.end_row();
						}
					});
					if ui.button("Apply").clicked() {
						self.apply_settings();
					}
				});
			});
			ui.add_space(5.0);
		});

		// 查看聚焦请求
		if get_focus_request() {
			println!("Do Focus Request");
			frame.focus();
			set_focus_request(false);
		}

		// 如果还有在下载的东西，那就刷新页面
		if !all_finished {
			ctx.request_repaint();
		} else {
			ctx.request_repaint_after(Duration::from_secs(1));
		}
	}
}
