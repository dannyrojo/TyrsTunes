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
mod audio;

fn main() {

    let tx1 = audio::spawn_audio_thread();
    let tx2 = audio::spawn_audio_thread();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Tyrs Tunes",
        native_options,
        Box::new(|cc| Ok(Box::new(frame::AppWindow::new(cc, tx1, tx2))))
    );
}