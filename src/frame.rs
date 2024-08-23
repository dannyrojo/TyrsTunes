use eframe::egui;
use crate::stage;
use crate::components;
use crate::playlist;

pub struct AppWindow {
    pub stage: stage::Stage,
    pub playlist_1: playlist::Playlist,
    pub playlist_2: playlist::Playlist,
}

impl AppWindow {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        
        let stage = stage::initialize_stage("tyrstunes.db")
            .expect("Failed to initialize stage");
        
        let (playlist_1, playlist_2) = playlist::initialize_playlists();
        
        Self { stage, playlist_1, playlist_2 } //TODO: add playback controller
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

                    components::render_tracks_panel(
                        ui, 
                        &mut self.stage, 
                        panel_width, panel_height, 
                        &mut self.playlist_1, 
                        &mut self.playlist_2);

                    components::render_filter_panel(
                        ui, 
                        &mut self.stage, 
                        panel_width, panel_height);
                });
            });

        // Bottom Panels
        egui::TopBottomPanel::bottom("bottom_panel")
            .min_height(panel_height)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    components::render_playlist_panel_1(
                        ui, 
                        panel_width, 
                        "Playlist 1", 
                        &mut self.playlist_1);

                    components::render_playlist_panel_2(
                        ui, 
                        panel_width, 
                        "Playlist 2", 
                        &mut self.playlist_2);
                });
            });
    }
}
