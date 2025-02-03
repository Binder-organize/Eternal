use anyhow::Context;
use eframe::emath::Rect;
use eframe::epaint::textures::TextureOptions;
use egui::{Color32, FontId, Pos2, Ui, Vec2};
use rodio::{Decoder, OutputStream};
use screen_size::get_primary_screen_size;
use std::fs::File;
use std::io::BufReader;

pub fn player(wav_path: String) -> anyhow::Result<f64> {
    // 打开 WAV 文件
    let reader = hound::WavReader::open(&wav_path).context("无法打开 WAV 文件")?;

    // 获取 WAV 文件的规格
    let spec = reader.spec();
    let sample_rate = spec.sample_rate as f64;
    let total_samples = reader.len() as f64;

    // 计算时长（秒）
    let duration = total_samples / sample_rate;

    // 打开文件并创建解码器
    let file = BufReader::new(File::open(&wav_path).context("无法打开文件")?);
    let source = Decoder::new(file).context("无法解码音频文件")?;

    // 获取默认物理声音设备的输出流句柄
    let (_stream, stream_handle) = OutputStream::try_default().context("无法获取默认输出流")?;

    // 创建一个新的 Sink 来管理播放
    let sink = rodio::Sink::try_new(&stream_handle).context("无法创建 Sink")?;
    sink.append(source);

    sink.sleep_until_end(); // 等待音频播放结束
    Ok(duration)
}

fn load_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/Text.ttf")).into(),
    );
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "my_font".to_owned());
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .push("my_font".to_owned());
    ctx.set_fonts(fonts);
}
pub struct Image {
    pub name: String,
    pub image_texture: Option<egui::TextureHandle>,
    pub image_position: [f32; 2],
    pub image_size: [f32; 2],
    pub x_grid: [u32; 2],
    pub y_grid: [u32; 2],
    pub center_display: [bool; 4],
}

pub struct CustomText {
    pub name: String,
    pub text_content: String,
    pub font_size: f32,
    pub rgba: [u8; 4],
    pub position: [f32; 2],
    pub center_display: [bool; 4],
    pub wrap_width: f32,
    pub write_background: bool,
    pub background_rgb: [u8; 3],
    pub rounding: f32,
    pub x_grid_locate_position: [u32; 2],
    pub y_grid_locate_position: [u32; 2],
}

pub struct MyEguiApp {
    pub page: Page,
    pub image: Vec<Image>,
    pub screen_size: Size,
    pub custom_text: Vec<CustomText>,
}

pub enum Page {
    Feature,
    InitialFeature,
    Home,
    InitialHome,
}

impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        load_fonts(&cc.egui_ctx);
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self {
            page: Page::InitialFeature,
            image: vec![Image {
                name: "Placeholder".to_string(),
                image_texture: None,
                image_position: [0.0, 0.0],
                image_size: [0.0, 0.0],
                x_grid: [0, 0],
                y_grid: [0, 0],
                center_display: [false, false, false, false],
            }],
            screen_size: Size {
                width: 1280,
                height: 720,
            },
            custom_text: vec![CustomText {
                name: "Placeholder".to_string(),
                text_content: "".to_string(),
                font_size: 0.0,
                rgba: [0, 0, 0, 0],
                position: [0.0, 0.0],
                center_display: [false, false, false, false],
                wrap_width: 0.0,
                write_background: false,
                background_rgb: [0, 0, 0],
                rounding: 0.0,
                x_grid_locate_position: [0, 0],
                y_grid_locate_position: [0, 0],
            }],
        }
    }

    pub fn success_create_message_sender(
        &mut self,
        subject: &str,
        subject_name: String,
        id: usize,
    ) {
        println!("已成功创建{}{},id为{}", subject, subject_name, id);
    }

    pub fn update_screen_size(&mut self, ctx: &egui::Context) {
        let available_rect = ctx.available_rect();
        self.screen_size.width = available_rect.width() as u64;
        self.screen_size.height = available_rect.height() as u64;
    }

    pub fn add_custom_text(
        &mut self,
        send_message: bool,
        name: &str,
        position: [f32; 2],
        font_size: f32,
        rgba: [u8; 4],
        text_content: &str,
        center_display: [bool; 4],
        wrap_width: f32,
        write_background: bool,
        background_rgb: [u8; 3],
        rounding: f32,
        x_grid_locate_position: [u32; 2],
        y_grid_locate_position: [u32; 2],
    ) {
        self.custom_text.push(CustomText {
            name: name.to_string(),
            text_content: text_content.to_string(),
            font_size,
            rgba,
            position,
            center_display,
            wrap_width,
            write_background,
            background_rgb,
            rounding,
            x_grid_locate_position,
            y_grid_locate_position,
        });
        if send_message {
            self.success_create_message_sender(
                "自定义文本",
                self.custom_text[self.custom_text.len() - 1].name.clone(),
                self.custom_text.len() - 1,
            );
        }
    }

    pub fn custom_text(&mut self, ui: &mut Ui, id: u32) {
        // 计算文本大小
        let galley = ui.fonts(|f| {
            f.layout(
                self.custom_text[id as usize].text_content.to_string(),
                FontId::proportional(self.custom_text[id as usize].font_size),
                Color32::from_rgba_premultiplied(
                    self.custom_text[id as usize].rgba[0],
                    self.custom_text[id as usize].rgba[1],
                    self.custom_text[id as usize].rgba[2],
                    self.custom_text[id as usize].rgba[3],
                ),
                self.custom_text[id as usize].wrap_width,
            )
        });
        let text_size = galley.size();
        self.custom_text[id as usize].position[0] =
            match self.custom_text[id as usize].x_grid_locate_position[1] {
                0 => self.custom_text[id as usize].position[0],
                _ => {
                    (self.screen_size.width as f64
                        / self.custom_text[id as usize].x_grid_locate_position[1] as f64
                        * self.custom_text[id as usize].x_grid_locate_position[0] as f64)
                        as f32
                }
            };
        self.custom_text[id as usize].position[1] =
            match self.custom_text[id as usize].y_grid_locate_position[1] {
                0 => self.custom_text[id as usize].position[1],
                _ => {
                    (self.screen_size.height as f64
                        / self.custom_text[id as usize].y_grid_locate_position[1] as f64
                        * self.custom_text[id as usize].y_grid_locate_position[0] as f64)
                        as f32
                }
            };
        let pos_x;
        let pos_y;
        if self.custom_text[id as usize].center_display[2] {
            pos_x = self.custom_text[id as usize].position[0] - text_size.x / 2.0;
        } else if self.custom_text[id as usize].center_display[0] {
            pos_x = self.custom_text[id as usize].position[0];
        } else {
            pos_x = self.custom_text[id as usize].position[0] - text_size.x;
        }
        if self.custom_text[id as usize].center_display[3] {
            pos_y = self.custom_text[id as usize].position[1] - text_size.y / 2.0;
        } else if self.custom_text[id as usize].center_display[1] {
            pos_y = self.custom_text[id as usize].position[1];
        } else {
            pos_y = self.custom_text[id as usize].position[1] - text_size.y;
        }
        // 使用绝对定位放置文本
        let position = Pos2::new(pos_x, pos_y);
        if self.custom_text[id as usize].write_background {
            let rect = Rect::from_min_size(position, text_size);
            // 绘制背景颜色
            ui.painter().rect_filled(
                rect,
                self.custom_text[id as usize].rounding,
                Color32::from_rgb(
                    self.custom_text[id as usize].background_rgb[0],
                    self.custom_text[id as usize].background_rgb[1],
                    self.custom_text[id as usize].background_rgb[2],
                ),
            ); // 背景色
        }
        // 绘制文本
        ui.painter().galley(position, galley, Color32::BLACK);
    }

    pub fn add_image(
        &mut self,
        send_message: bool,
        name: &str,
        position: [f32; 2],
        size: [f32; 2],
        x_grid_locate_position: [u32; 2],
        y_grid_locate_position: [u32; 2],
        center_display: [bool; 4],
    ) {
        self.image.push(Image {
            name: name.to_string(),
            image_texture: None,
            image_position: position,
            image_size: size,
            x_grid: x_grid_locate_position,
            y_grid: y_grid_locate_position,
            center_display,
        });
        if send_message {
            self.success_create_message_sender(
                "图像",
                self.image[self.image.len() - 1].name.clone(),
                self.image.len() - 1,
            )
        }
    }

    pub fn load_image(&mut self, ctx: &egui::Context, image_loader: &[u8], id: u32, ui: &mut Ui) {
        if self.image[id as usize].image_texture.is_none() {
            let img =
                image::load_from_memory_with_format(image_loader, image::ImageFormat::Png).unwrap();
            let rgba_data = img.into_rgba8();
            let (w, h) = (rgba_data.width(), rgba_data.height());
            let raw_data: Vec<u8> = rgba_data.into_raw();

            let color_image =
                egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &raw_data);
            self.image[id as usize].image_texture =
                Some(ctx.load_texture("", color_image, TextureOptions::LINEAR));
        }

        self.image[id as usize].image_position[0] = match self.image[id as usize].x_grid[1] {
            0 => self.image[id as usize].image_position[0],
            _ => {
                (self.screen_size.width as f64 / self.image[id as usize].x_grid[1] as f64
                    * self.image[id as usize].x_grid[0] as f64) as f32
            }
        };
        self.image[id as usize].image_position[1] = match self.image[id as usize].y_grid[1] {
            0 => self.image[id as usize].image_position[1],
            _ => {
                (self.screen_size.height as f64 / self.image[id as usize].y_grid[1] as f64
                    * self.image[id as usize].y_grid[0] as f64) as f32
            }
        };
        let pos_x;
        let pos_y;
        if self.image[id as usize].center_display[2] {
            pos_x = self.image[id as usize].image_position[0]
                - self.image[id as usize].image_size[0] / 2.0;
        } else if self.image[id as usize].center_display[0] {
            pos_x = self.image[id as usize].image_position[0];
        } else {
            pos_x =
                self.image[id as usize].image_position[0] - self.image[id as usize].image_size[0];
        }
        if self.image[id as usize].center_display[3] {
            pos_y = self.image[id as usize].image_position[1]
                - self.image[id as usize].image_size[1] / 2.0;
        } else if self.image[id as usize].center_display[1] {
            pos_y = self.image[id as usize].image_position[1];
        } else {
            pos_y =
                self.image[id as usize].image_position[1] - self.image[id as usize].image_size[1];
        }
        if let Some(texture) = &self.image[id as usize].image_texture {
            let rect = Rect::from_min_size(
                Pos2::new(pos_x, pos_y),
                Vec2::new(
                    self.image[id as usize].image_size[0],
                    self.image[id as usize].image_size[1],
                ),
            );
            ui.painter().image(
                texture.into(),
                rect,
                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                Color32::WHITE,
            );
        }
    }
}

pub struct Size {
    pub width: u64,
    pub height: u64,
}

impl Size {
    pub fn new() -> Self {
        Self {
            width: 1280,
            height: 720,
        }
    }

    pub fn calculate(&mut self) {
        if let Ok((try_catch_width, try_catch_height)) = get_primary_screen_size() {
            (self.width, self.height) = (try_catch_width, try_catch_height);
        }
    }
}
