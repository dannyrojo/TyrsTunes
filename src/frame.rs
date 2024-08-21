use eframe::egui;
use crate::stage;
use crate::components;

pub struct AppWindow {
    pub stage: stage::Stage,
}

impl AppWindow {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let stage = stage::initialize_stage("tyrstunes.db")
            .expect("Failed to initialize stage");
        Self { stage }
    }
}

impl eframe::App for AppWindow {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let panel_height = ctx.available_rect().height() / 2.0;
        let panel_width = ctx.available_rect().width() / 2.0;

        // Top Panels
        egui::TopBottomPanel::top("top_panel")
            .min_height(panel_height)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    components::render_tracks_panel(ui, &mut self.stage, panel_width, panel_height);
                    components::render_filter_panel(ui, &mut self.stage, panel_width, panel_height);
                });
            });

        // Bottom Panels
        egui::TopBottomPanel::bottom("bottom_panel")
            .min_height(panel_height)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    components::render_playlist_panel(ui, panel_width, "Playlist 1");
                    components::render_playlist_panel(ui, panel_width, "Playlist 2");
                });
            });
    }
}
