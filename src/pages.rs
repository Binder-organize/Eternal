use crate::function;
use eframe::egui::{self};
use function::MyEguiApp;
use function::Page;

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.page {
            Page::InitialFeature => {
                self.add_image(
                    true,
                    "Example1",
                    [600_f32, 600_f32],
                    [50_f32, 50_f32],
                    [0, 1],
                    [1, 2],
                    [false, true, true, false],
                );
                self.add_image(
                    false,
                    "Example2",
                    [400_f32, 400_f32],
                    [70_f32, 70_f32],
                    [1, 2],
                    [1, 2],
                    [true, true, true, true],
                );
                self.add_custom_text(
                    true,
                    "Example3",
                    [0.0, 0.0],
                    24.0,
                    [0, 0, 0, 255],
                    "这是一段以左上角排版，圆润黄色背景，黑色文本与限定长度的RustConstructor自定义文本！",
                    [true, true, false, false],
                    100.0,
                    true,
                    [255, 255, 0],
                    10.0,
                    [0, 1],
                    [0, 1],
                );
                self.page = Page::InitialHome;
            }
            Page::InitialHome => {
                self.add_custom_text(
                    true,
                    "Title",
                    [0.0, 0.0],
                    48.0,
                    [127, 127, 127, 255],
                    "Rust Constructor v0.1.0",
                    [false, true, true, false],
                    1000.0,
                    false,
                    [255, 255, 0],
                    10.0,
                    [1, 2],
                    [1, 4],
                );
                self.page = Page::Home;
            }
            Page::Feature => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.update_screen_size(ctx);
                    self.load_image(
                        ctx,
                        include_bytes!("../assets/images/SampleImage.png"),
                        1,
                        ui,
                    );
                    self.load_image(
                        ctx,
                        include_bytes!("../assets/images/SampleImage.png"),
                        2,
                        ui,
                    );
                    self.custom_text(ui, 1);
                });
            }

            Page::Home => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.update_screen_size(ctx);
                    self.custom_text(ui, 2);
                });
                egui::SidePanel::right("更新内容").show(ctx, |ui| {
                    ui.heading("Rust Constructor v0.1.0更新内容：");
                    ui.label("1.构建最基础的框架\n2.加入图片：包含网格式定位，修改坐标对应显示位置\n3.加入自定义文本：可修改颜色，包含网格式定位，纯色背景（可添加圆滑边&修改颜色），修改坐标对应显示位置等功能\n4.网格式定位：以[x, y]形式表示，将长/宽除以y并乘x，可以在任何窗口尺寸下完美适配\n5.其他功能：wav播放器，创建文本/图片成功播报器等");
                });
            } // _ => {
              // }
        }
        egui::TopBottomPanel::bottom("Console").show(ctx, |ui| {
            if ui.button("主页").clicked() {
                self.page = Page::Home;
            }
            if ui.button("新特性").clicked() {
                self.page = Page::Feature;
            }
        });
    }
}
