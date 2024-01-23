use std::thread;

use data::listen_theme_change;
use eframe::{NativeOptions, epaint::vec2, run_native, IconData};
use image::DynamicImage;

mod app;
mod session;
mod data;
mod server;
mod settings;
mod aria2c;
mod history;
mod widgets;

#[tokio::main]
async fn main() {
    // 启用监听服务
    tokio::spawn(server::listen());
    // 监听系统主题变化
    thread::spawn(|| listen_theme_change());
    // 应用设置
    // 图标
    let icon_source: Option<DynamicImage> = match image::open("assets/icon.iconset/icon_512x512.png") {
        Ok(icon) => Some(icon),
        Err(_) => None,
    };
    let icon_data = match icon_source {
        Some(icon) => Some(IconData {
            rgba: icon.to_rgba8().into_raw(),
            width: icon.width(),
            height: icon.height(),
        }),
        None => None,
    };

    let options = NativeOptions {
        initial_window_size: Some(vec2(650.0, 400.0)),
        icon_data,
        follow_system_theme: true,
        ..Default::default()
    };

    // 运行应用
    run_native(
        "Aria Download Manager",
        options,
        Box::new(|cc|
            Box::<app::DownloadManager>::new(
                app::DownloadManager::new(cc)
            )
        )
    ).unwrap();
}
