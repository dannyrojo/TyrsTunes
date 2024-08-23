use anyhow::Result;
use eframe::egui;

mod database;
mod track;
mod import;
mod utils;
mod stage;
mod frame;
mod components;
mod playlist;
mod state;
mod playback;

fn main() {

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Tyrs Tunes",
        native_options,
        Box::new(|cc| Ok(Box::new(frame::AppWindow::new(cc))))
    );
}