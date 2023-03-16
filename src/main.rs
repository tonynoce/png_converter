#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use rfd;

use png_converter::convert_to_png;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(640.0, 320.0)),
        ..Default::default()
    };
    eframe::run_native(
        "JPG to PNG",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

#[derive(Default)]
struct MyApp {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Convert your jpg to png");
            ui.label("You can drop a file too !");
            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }
            
            // Convert appears after file is selected
            if let Some(picked_path) = &self.picked_path {
                if ui.button("Convert selected file").clicked() {
                    match convert_to_png(picked_path.trim().to_string()) {
                        Ok(_) => (),
                        Err(e) => {
                            //                            ui.monospace(e.to_string())                      
                            println!("Error de imagen {:?}", e);
                            return 
                        }
                    };
                }
            }
            // Show dropped files (if any):
            if !self.dropped_files.is_empty() {
                let mut info = String::new();

                ui.group(|ui| {
                    ui.label("Dropped files:");

                    for file in &self.dropped_files {
                        info = if let Some(path) = &file.path {
                            path.display().to_string()
                        } else if !file.name.is_empty() {
                            file.name.clone()
                        } else {
                            "???".to_owned()
                        };
                        if let Some(bytes) = &file.bytes {
                            use std::fmt::Write as _;
                            write!(info, " ({} bytes)", bytes.len()).ok();
                        }
                        ui.label(&info);
                    }
                });

                // Convert droppped file
                if ui.button("Convert dropped file").clicked() {
                    
                    match convert_to_png(info) {
                        Ok(_) => (),
                        Err(e) => {
                            //                            ui.monospace(e.to_string())                      
                            println!("Error de imagen {:?}", e);
                            return 
                        }
                    };
            }
        }

        });

        

        preview_files_being_dropped(ctx);

        // Collect dropped files:
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.dropped_files = i.raw.dropped_files.clone();
            }
        });
    }
}

/// Preview hovering files:
fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}