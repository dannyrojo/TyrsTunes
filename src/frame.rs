use eframe::egui;
use crate::stage;
use crate::components;
use crate::playlist;
use crate::state;

pub struct AppWindow {
    pub stage: stage::Stage,
    pub playlist_1: playlist::Playlist,
    pub playlist_2: playlist::Playlist,
    pub state_1: state::State,
    pub state_2: state::State,
}

impl AppWindow {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        
        let stage = stage::initialize_stage("tyrstunes.db")
            .expect("Failed to initialize stage");
        
        let (playlist_1, playlist_2) = playlist::initialize_playlists();

        let (state_1, state_2) = state::initialize_state();
        
        Self { stage, playlist_1, playlist_2, state_1, state_2 } //TODO: add playback controller
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

                    components::tracks_panel
                    (
                        ui, 
                        panel_width, panel_height, 
                        &mut self.stage, 
                        &mut self.playlist_1, 
                        &mut self.playlist_2
                    );

                    components::filter_panel
                    (
                        ui, 
                        panel_width, panel_height,
                        &mut self.stage, 
                    );
                });
            });

        // Bottom Panels
        egui::TopBottomPanel::bottom("bottom_panel")
            .min_height(panel_height)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    components::playlist_panel
                    (
                        ui, 
                        panel_width, panel_height, 
                        "Playlist 1", 
                        &mut self.playlist_1,
                        &mut self.state_1,
                    );

                    components::playlist_panel
                    (
                        ui, 
                        panel_width, panel_height, 
                        "Playlist 2", 
                        &mut self.playlist_2,
                        &mut self.state_2,
                    );
                });
            });
    }
}
