use eframe::{NativeOptions, epaint::vec2, run_native};

mod app;
mod session;
mod data;
mod server;
mod settings;
mod aria2c;

#[tokio::main]
async fn main() {
    // 启用监听服务
    tokio::spawn(server::listen());
    // 应用设置
    let options = NativeOptions {
        initial_window_size: Some(vec2(600.0, 350.0)),
        ..Default::default()
    };
    // 运行应用
    run_native(
        "Aria Download Manager",
        options,
        Box::new(|_cc| Box::<app::DownloadManager>::default())
    ).unwrap();
}
