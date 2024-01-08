use actix_web::{HttpServer, App, web};
use serde::{Serialize, Deserialize};

use crate::data::add_wait_to_start;

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
	Ok(String::new())
}

pub async fn listen() {
	println!("Start Server");
	HttpServer::new(|| {
		App::new().route("/", web::post().to(index))
	})
	.bind("127.0.0.1:63319").unwrap()
	.run().await
	.unwrap();
}
