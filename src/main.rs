use anyhow::Result;
use eframe::egui;

mod database;
mod track;
mod import;
mod utils;
mod stage;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Tyrs Tunes", native_options, Box::new(|cc| Ok(Box::new(AppWindow::new(cc)))));
}

struct AppWindow {
    stage: stage::Stage,
}

impl AppWindow {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let stage = stage::initialize_stage("tyrstunes.db")
            .expect("Failed to initialize stage");
        Self { stage }
    }   
}

impl eframe::App for AppWindow {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tyrs Tunes");
        });
    }
}