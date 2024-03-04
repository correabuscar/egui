#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::*;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Keyboard events",
        options,
        Box::new(|_cc| Box::<Content>::default()),
    )
}

#[derive(Default)]
struct Content {
    text: String,
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Press/Hold/Release example. Press A to test.");
            if ui.button("Clear").clicked() {
                self.text.clear();
            }
            ScrollArea::vertical()
                .auto_shrink(false)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.label(&self.text);
                });

            let pressed:bool;
            let human_pressed:bool;
            let held:bool;
            let released:bool;


            if ctx.input(|i| i.key_pressed(Key::A)) {
                pressed=true;
            } else {
                pressed=false;
            }
            if ctx.input(|i| i.key_down(Key::A)) {
                held=true;
            } else {
                held=false;
            }
            if pressed && !held  {
                human_pressed=true;
            } else {
                human_pressed=false;
            }
            if ctx.input(|i| i.key_released(Key::A)) {
                released=true;
            } else {
                released=false;
            }
            //self.text=String::from(format!("{},{},{},{}",human_pressed as i8,pressed as i8,held as i8,released as i8));
            self.text.push_str(&format!("HP:{},P:{},H:{},R:{}\n",human_pressed as i8,pressed as i8,held as i8,released as i8)[..]);
            if held {
                ui.ctx().request_repaint(); // make sure we note the holding.
            }
        });
    }
}
