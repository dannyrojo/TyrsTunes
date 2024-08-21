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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let panel_height = ctx.available_rect().height() / 2.0;
        let panel_width = ctx.available_rect().width() / 2.0;

        // Top Panels
        egui::TopBottomPanel::top("top_panel")
            .min_height(panel_height)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Top Left Panel
                    ui.vertical(|ui| {
                        ui.set_width(panel_width);
                        ui.label("Tracks");
                    });

                    // Top Right Panel
                    ui.vertical(|ui| {
                        ui.set_width(panel_width);
                        ui.label("Filter");
                    });
                });
            });

        // Bottom Panels
        egui::TopBottomPanel::bottom("bottom_panel")
            .min_height(panel_height)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Bottom Left Panel
                    ui.vertical(|ui| {
                        ui.set_width(panel_width);
                        ui.label("Playlist 1");
                    });

                    // Bottom Right Panel
                    ui.vertical(|ui| {
                        ui.set_width(panel_width);
                        ui.label("Playlist 2");
                    });
                });
            });
    }
}
