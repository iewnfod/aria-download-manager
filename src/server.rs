use actix_web::{HttpServer, App, web};
use serde::{Serialize, Deserialize};

use crate::data::{add_wait_to_start, set_quit_request};

#[derive(Debug, Serialize, Deserialize)]
struct Info {
	download_id: usize,
	size: usize,
	webpage_url: String,
	download_url: String,
	resume_state: bool,
}

async fn index(info: web::Json<Info>) -> actix_web::Result<String> {
	println!("{:?}", &info);
	add_wait_to_start(info.download_url.clone());
	Ok("{}".to_string())
}

async fn state() -> actix_web::Result<String> {
    Ok("{\"status\": 0}".to_string())
}

async fn quit_handler() -> actix_web::Result<String> {
	set_quit_request(true);
    Ok("{\"status\": 0}".to_string())
}

pub async fn listen() {
	println!("Start Server");
	HttpServer::new(|| {
		App::new()
			.route("/api", web::post().to(index))
			.route("/state", web::get().to(state))
			.route("/quit", web::get().to(quit_handler))
	})
	.bind("127.0.0.1:63319").unwrap()
	.run().await
	.unwrap();
}
