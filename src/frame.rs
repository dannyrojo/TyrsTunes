use eframe::egui;
use crate::stage;
use crate::components;
use crate::playlist;
use crate::state;
use crate::audio;
use std::sync::mpsc;

pub struct AppWindow {
    pub stage: stage::Stage,
    pub playlist_1: playlist::Playlist,
    pub playlist_2: playlist::Playlist,
    pub state_1: state::State,
    pub state_2: state::State,
    pub tx1: mpsc::Sender<audio::AudioCommand>,
    pub tx2: mpsc::Sender<audio::AudioCommand>,
}

impl AppWindow {
    pub fn new(cc: &eframe::CreationContext<'_>, tx1: mpsc::Sender<audio::AudioCommand>, tx2: mpsc::Sender<audio::AudioCommand>) -> Self {
        
        let stage = stage::initialize_stage("tyrstunes.db")
            .expect("Failed to initialize stage");
        
        let (playlist_1, playlist_2) = playlist::initialize_playlists();

        let (state_1, state_2) = state::initialize_state();
        
        

        Self { stage, playlist_1, playlist_2, state_1, state_2, tx1, tx2 }
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
                    ui.vertical(|ui| {
                        ui.set_width(panel_width);
                        ui.set_height(panel_height);
                        ui.label("Tracks");
                        ui.separator();
                        egui::ScrollArea::vertical()
                            .max_height(panel_height)
                            .max_width(panel_width)
                            .auto_shrink(false)
                            .show(ui, |ui| {
                                components::render_tracks_list( //tracks list
                                    ui, 
                                    &mut self.stage, 
                                    &mut self.playlist_1, 
                                    &mut self.playlist_2
                                );
                            });
                    });
                    
                    ui.vertical(|ui| {
                        ui.set_width(panel_width);
                        ui.label("Tags");
                        ui.separator();
                        components::render_filter_menu(
                            ui, 
                            &mut self.stage
                        ); 
                        ui.separator();
                        components::render_tags_list(
                            ui, 
                            &mut self.stage
                        ); 
                    });
                });
            });

        // Bottom Panels
        egui::TopBottomPanel::bottom("bottom_panel")
            .min_height(panel_height)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.set_width(panel_width);
                        ui.set_height(panel_height);
                        ui.label("Playlist 1");
                        ui.separator();
                        ui.push_id("playlist_1", |ui| {
                            egui::ScrollArea::vertical()
                                .max_height(panel_height)
                                .max_width(panel_width)
                                .auto_shrink(false)
                                .show(ui, |ui| {
                                    components::render_playlist( //playlist
                                        ui, 
                                        &mut self.playlist_1, 
                                        &mut self.state_1, 
                                        &self.tx1
                                    ); 
                                });
                        });
                    });
                    ui.vertical(|ui| {
                        ui.set_width(panel_width);
                        ui.set_height(panel_height);
                        ui.label("Playlist 2");
                        ui.separator();
                        ui.push_id("playlist_2", |ui| {
                            egui::ScrollArea::vertical()
                                .max_height(panel_height)
                                .max_width(panel_width)
                                .auto_shrink(false)
                                .show(ui, |ui| {
                                    components::render_playlist( //playlist
                                        ui, 
                                        &mut self.playlist_2, 
                                        &mut self.state_2, 
                                        &self.tx2
                                    ); 
                                });
                        });
                    });
                });
            });
    }
}