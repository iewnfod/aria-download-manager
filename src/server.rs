use actix_web::{HttpServer, App, web};
use serde::{Serialize, Deserialize};

use crate::data::{add_wait_to_start, set_focus_request, set_quit_request};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
	pub download_id: usize,
	pub size: usize,
	pub webpage_url: String,
	pub download_url: String,
	pub resume_state: bool,
	pub download_cookie: Vec<Cookie>,
	pub download_referer: String,
}

impl Info {
	pub fn with_download_url(url: String) -> Self {
		Self {
			download_id: 0,
			size: 0,
			webpage_url: "".to_string(),
			download_url: url,
			resume_state: false,
			download_cookie: vec![],
			download_referer: "".to_string(),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
	domain: String,
	host_only: bool,
	http_only: bool,
	name: String,
	path: String,
	same_site: String,
	secure: bool,
	session: bool,
	store_id: String,
	value: String,
}

impl ToString for Cookie {
	fn to_string(&self) -> String {
		format!("{}={}", self.name, self.value)
	}
}

async fn index(info: web::Json<Info>) -> actix_web::Result<String> {
	println!("{:?}", &info);
	add_wait_to_start(info.clone());
	Ok("{}".to_string())
}

async fn state() -> actix_web::Result<String> {
    Ok("{\"status\": 0}".to_string())
}

async fn quit_handler() -> actix_web::Result<String> {
	println!("Request Quit");
	set_quit_request(true);
    Ok("{\"status\": 0}".to_string())
}

async fn focus() -> actix_web::Result<String> {
	println!("Request Focus");
	set_focus_request(true);
	Ok("{\"status\": 0}".to_string())
}

pub async fn listen() {
	println!("Start Server");
	HttpServer::new(|| {
		App::new()
			.route("/api", web::post().to(index))
			.route("/state", web::get().to(state))
			.route("/focus", web::get().to(focus))
			.route("/quit", web::get().to(quit_handler))
	})
	.bind("127.0.0.1:63318").unwrap()
	.run().await
	.unwrap();
}
