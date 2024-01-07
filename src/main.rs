use eframe::{egui::ViewportBuilder, NativeOptions, epaint::vec2, run_native};

mod app;
mod session;
mod aria2c;

#[tokio::main]
async fn main() {
    // 启动 aria2c 服务
    aria2c::startup();
    // 应用设置
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size(vec2(600.0, 350.0)),
        ..Default::default()
    };
    // 运行应用
    run_native(
        "Aria Download Manager",
        options,
        Box::new(|_cc| Box::<app::DownloadManager>::default())
    ).unwrap();
}
