use eframe::egui;
use crate::audio_backend::AudioPlayer;
use anyhow::Result;

pub struct MyApp {
    audio_player: AudioPlayer,
    // Add other UI state here
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Result<Self> {
        Ok(Self {
            audio_player: AudioPlayer::new()?,
            // Initialize other UI state
        })
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Audio Player Controls");

            // Play/Resume and Stop buttons
            ui.horizontal(|ui| {
                if ui.button("Play/Resume Playlist 1").clicked() {
                    if let Err(e) = self.audio_player.play_or_resume(1) {
                        eprintln!("Error playing/resuming playlist 1: {}", e);
                    }
                }
                if ui.button("Stop Playlist 1").clicked() {
                    self.audio_player.stop(1);
                }
            });

            // Volume slider
            let mut volume = self.audio_player.get_volume(1);
            ui.add(egui::Slider::new(&mut volume, 0.0..=1.0).text("Volume"));
            if ui.button("Set Volume").clicked() {
                self.audio_player.set_volume(1, volume);
            }

            // Toggle loop button
            if ui.button("Toggle Loop").clicked() {
                if let Err(e) = self.audio_player.toggle_loop(1) {
                    eprintln!("Error toggling loop: {}", e);
                }
            }

            // Skip button
            if ui.button("Skip").clicked() {
                if let Err(e) = self.audio_player.skip(1) {
                    eprintln!("Error skipping track: {}", e);
                }
            }

            // Add to playlist
            ui.horizontal(|ui| {
                let mut new_track = String::new();
                ui.text_edit_singleline(&mut new_track);
                if ui.button("Add to Playlist").clicked() {
                    if let Err(e) = self.audio_player.add_to_playlist(1, new_track.clone()) {
                        eprintln!("Error adding track to playlist: {}", e);
                    }
                }
            });
        });

        // Update audio backend
        if let Err(e) = self.audio_player.update() {
            eprintln!("Error updating audio player: {}", e);
        }

        // Request continuous redraw
        ctx.request_repaint();
    }
}