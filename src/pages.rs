use crate::function;
use eframe::egui::{self};
use egui::Key;
use function::MyEguiApp;
use std::env;

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 使用自定义样式显示输入框
            egui::ScrollArea::vertical()
                .enable_scrolling(true)
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width()); // 设置宽度为可用宽度
                    ui.set_height(ui.available_height()); // 设置高度为可用高度
                    ui.vertical(|ui| {
                        ui.colored_label(
                            egui::Color32::from_rgba_premultiplied(0, 0, 0, 255),
                            &*self.strs,
                        );
                        ui.add_space(-20.0);
                        ui.horizontal(|ui| {
                            let mut prompt_text = "[".to_string();
                            let current_dir = env::current_dir().unwrap_or_else(|e| {
                                eprintln!("Error getting current directory: {}", e);
                                std::path::PathBuf::from("")
                            });

                            let current_path = self.format_path(&current_dir);
                            prompt_text += current_path.as_str();
                            prompt_text += "]> ";
                            ui.colored_label(
                                egui::Color32::from_rgba_premultiplied(0, 0, 0, 255),
                                &prompt_text,
                            );
                            ui.add_space(-10.0);
                            ui.add(
                                egui::TextEdit::multiline(&mut self.input_strs)
                                    .frame(false)
                                    .desired_width(f32::INFINITY)
                                    .text_color(egui::Color32::from_rgba_premultiplied(0, 0, 0, 255))
                                    .hint_text("Get started..."), // 提示文本
                            );
                            if ui.input(|i| i.key_pressed(Key::Enter)) {
                                if !self.input_strs.is_empty() {
                                    self.strs += &*prompt_text;
                                    self.strs += &self.input_strs;
                                    self.shell(self.input_strs.to_string());
                                    self.input_strs.clear();
                                }
                            }
                        });
                    });
                });
        });
    }
}
