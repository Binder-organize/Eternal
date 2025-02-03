use egui::IconData;
use std::sync::Arc;
mod function;
mod pages;
mod zsh;

use function::player;
use function::MyEguiApp;
use function::Size;

fn main() {

    // // 创建 MyEguiApp 实例
    let mut screen_size = Size::new();
    screen_size.calculate();
    // 访问 screen_size
    let _ = std::thread::spawn(|| {
        let _ = player("assets/sounds/Launch.wav".to_string());
    });
    let mut options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([screen_size.width as f32, screen_size.width as f32]),
        ..Default::default()
    };
    let img = image::load_from_memory_with_format(
        include_bytes!("../assets/images/icon.png"),
        image::ImageFormat::Png,
    )
    .unwrap();
    let rgba_data = img.into_rgba8();
    let (w, h) = (rgba_data.width(), rgba_data.height());
    let raw_data: Vec<u8> = rgba_data.into_raw();
    options.viewport.icon = Some(Arc::<IconData>::new(IconData {
        rgba: raw_data,
        width: w,
        height: h,
    }));
    let _ = eframe::run_native(
        "Eternal",
        options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    );
}
