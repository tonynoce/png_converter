#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::egui::{Id};
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
        Box::new(|_cc| Box::new(PngApp::default())),
    )
}

#[derive(Default)]
struct PngApp {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
}

impl eframe::App for PngApp {
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
            

            let mut window_text = egui::Window::new("Error en la conversión")
                .open(&mut false)
                .default_open(false)
                .resizable(true)
                .id(Id::from("window_text"))
                .show(ctx, |ui| {
                    ui.label("some text");
                });

            if let Some(picked_path) = &self.picked_path {
                let response = ui.button("Convert selected file");
                let mut error_text = String::new();
                let popup_id = ui.make_persistent_id("error_window");
                
                let below = egui::AboveOrBelow::Below;
                let mut memory = egui::Memory::default();
                //let mut open = true;
                egui::popup::popup_above_or_below_widget(ui, popup_id, &response, below, |ui| {
                    ui.label("Some more info, or things you can select:");
                    ui.label(error_text);
                });
                

                if response.clicked() {
                    println!("s2magen");

                    ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                    
                    egui::Memory::open_popup(&mut memory, Id::from("error_window"));
                    match convert_to_png(picked_path.trim().to_string()) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("picked imagen error");

                            //let 
                            error_text = e.to_string();
                            println!("{}", e);
                            //open = true;
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

                    for file in &self.dropped_files {
                        //info = file.path.as_ref().unwrap().display().to_string(); //unsafe way of doing it
                        //&file.path.display().to_string();
                        
                        info = if let Some(path) = &file.path {
                            path.display().to_string()
                        } else {
                            panic!("Scheiise");
                        };

                        //println!("{:?}", info);
                        match convert_to_png(info) {
                            Ok(_) => (),
                            Err(e) => {
                                //ui.monospace(e.to_string())                      
                                let error_text = e.to_string();
                                //display_error(ctx, text);


                                println!("Error de imagen {:?}", e);

                                
                            } 
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
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(100));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}

 
 /*
 fn display_error(ctx: &egui::Context, e:String) -> egui::Window {
     let error_text = e.to_string();
     let error_window =    egui::Window::new("Error en la conversión")
     .resizable(true)
     .show(ctx, |ui| {
         ui.label("error_text");
         if ui.button("Close").clicked() {
             eprintln!("Pressed Close button");
             //open = false
             //_frame.close();
            }
        });
        error_window
    }
*/


#[derive(Default)]
struct MyError {
    show_confirmation_dialog: bool,
}

impl eframe::App for MyError {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Close").clicked() {
                eprintln!("Pressed Close button");
                frame.close();
            }
        });
    }
}