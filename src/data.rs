use std::process::{Command, Stdio};

use eframe::{egui::{FontData, FontDefinitions, Style, TextStyle, Visuals}, epaint::{FontFamily, FontId}};

use crate::{settings::Settings, server::Info};

static mut STATUS_INFO: String = String::new();
static mut WAIT_TO_START: Vec<Info> = vec![];
static mut QUIT_REQUEST: bool = false;
static mut FOCUS_REQUEST: bool = false;
static mut SETTINGS_UPDATE: bool = false;
static mut VISUAL_DARK: bool = false;

static mut SETTINGS: Option<Settings> = None;

pub const CLICK_TO_COPY_PROMPT: &str = "Click to Copy";

pub fn set_settings(new_settings: Settings) {
	unsafe {
		SETTINGS = Some(new_settings);
	}
}

pub fn get_settings() -> Settings {
	unsafe {
		if SETTINGS.is_none() {
			let settings = Settings::new();
			set_settings(settings.clone());
			settings
		} else {
			SETTINGS.clone().unwrap()
		}
	}
}

pub fn set_status_info(info: String) {
	unsafe {
		STATUS_INFO = info;
	}
}

pub fn get_status_info() -> String {
	unsafe {
		STATUS_INFO.clone()
	}
}

pub fn add_wait_to_start(data: Info) {
	unsafe {
		WAIT_TO_START.push(data);
	}
}

pub fn get_wait_to_start() -> Vec<Info> {
	unsafe {
		WAIT_TO_START.clone()
	}
}

pub fn clear_wait_to_start() {
	unsafe {
		WAIT_TO_START = vec![];
	}
}

pub fn get_quit_request() -> bool {
	unsafe {
		QUIT_REQUEST
	}
}

pub fn set_quit_request(q: bool) {
	unsafe {
		QUIT_REQUEST = q;
	}
}

pub fn get_focus_request() -> bool {
	unsafe {
		FOCUS_REQUEST
	}
}

pub fn set_focus_request(f: bool) {
	unsafe {
		FOCUS_REQUEST = f;
	}
}

pub fn get_settings_update() -> bool {
	unsafe {
		SETTINGS_UPDATE
	}
}

pub fn set_settings_update(s: bool) {
	unsafe {
		SETTINGS_UPDATE = s;
	}
}

pub fn get_visual_dark() -> bool {
	unsafe {
		VISUAL_DARK
	}
}

pub fn set_visual_dark(v: bool) {
	unsafe {
		VISUAL_DARK = v;
	}
}

pub fn get_global_fonts() -> FontDefinitions {
	let mut fonts = FontDefinitions::default();
	let font_name = "LXGW".to_string();

	fonts.font_data.insert(
		font_name.clone(),
		FontData::from_static(include_bytes!("../assets/LXGWWenKaiGBFusion-Regular.ttf"))
	);

	fonts.families.get_mut(&FontFamily::Monospace).unwrap()
	.insert(0, font_name.clone());

	fonts
}

pub fn get_global_style() -> Style {
	let mut style = Style::default();

	// 设置字体
	let small_font_id = FontId::new(9.0, FontFamily::Monospace);
	let middle_font_id = FontId::new(12.5, FontFamily::Monospace);
	let large_font_id = FontId::new(18.0, FontFamily::Monospace);

	style.text_styles.insert(TextStyle::Small, small_font_id.clone());
	style.text_styles.insert(TextStyle::Body, middle_font_id.clone());
	style.text_styles.insert(TextStyle::Button, middle_font_id.clone());
	style.text_styles.insert(TextStyle::Heading, large_font_id.clone());

	// 设置亮暗色主题
	if get_settings().dark_mode {
		style.visuals = Visuals::dark();
	} else {
		style.visuals = Visuals::light();
	}

	style
}

fn get_dark_mode() -> bool {
	let mut command = Command::new("osascript");
	command.arg("-e").arg("tell application \"System Events\" to tell appearance preferences to return dark mode");
	let output = command.stdout(Stdio::piped()).output().unwrap();
	let str_data = String::from_utf8_lossy(&output.stdout);
	if str_data.trim() == "true" {
		true
	} else {
		false
	}
}

pub fn listen_theme_change() {
	loop {
		let mut settings = get_settings();
		let dark_mode = get_dark_mode();
		// 如果发生了改变
		if settings.dark_mode != dark_mode {
			// 如果不自定义主题，那就修改
			if !settings.custom_theme {
				settings.dark_mode = dark_mode;
				set_settings(settings);
				set_settings_update(true);
			} else {
				// 如果自定义了主题，并且显示的还和设置的不一样的话，请求刷新
				if settings.dark_mode != get_visual_dark() {
					set_settings_update(true);
				}
			}
		}
		// 每隔一秒刷新一次
		std::thread::sleep(std::time::Duration::from_secs(1));
	}
}
