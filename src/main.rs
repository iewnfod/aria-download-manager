use eframe::{NativeOptions, epaint::vec2, run_native, IconData};

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
    // 图标
    let icon = image::open("assets/icon.iconset/icon_512x512.png").unwrap().to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();

    let options = NativeOptions {
        initial_window_size: Some(vec2(600.0, 350.0)),
        icon_data: Some(IconData {
            rgba: icon.into_raw(),
            width: icon_width,
            height: icon_height,
        }),
        ..Default::default()
    };
    // 运行应用
    run_native(
        "Aria Download Manager",
        options,
        Box::new(|_cc| Box::<app::DownloadManager>::default())
    ).unwrap();
}
