use eframe::egui;
use crate::audio_backend::AudioPlayer;
use anyhow::Result;

pub struct MyApp {
    audio_player: AudioPlayer,
    new_track_input: String,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Result<Self> {
        Ok(Self {
            audio_player: AudioPlayer::new()?,
            new_track_input: String::new(),
        })
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Audio Player Controls");

            ui.horizontal(|ui| {
                // Play/Pause button
                if ui.button(if self.audio_player.is_playing(1) { "⏸ Pause" } else { "▶ Play" }).clicked() {
                    if self.audio_player.is_playing(1) {
                        self.audio_player.stop(1);
                    } else if let Err(e) = self.audio_player.play_or_resume(1) {
                        eprintln!("Error playing/resuming playlist 1: {}", e);
                    }
                }

                // Stop button
                if ui.button("⏹ Stop").clicked() {
                    self.audio_player.stop(1);
                }

                // Skip button
                if ui.button("⏭ Skip").clicked() {
                    if let Err(e) = self.audio_player.skip(1) {
                        eprintln!("Error skipping track: {}", e);
                    }
                }

                // Loop button
                let loop_button = egui::Button::new("🔁 Loop")
                    .fill(if self.audio_player.is_looping(1) {
                        egui::Color32::from_rgb(0, 125, 255) // Bright blue when active
                    } else {
                        ui.visuals().widgets.inactive.bg_fill
                    });
                
                if ui.add(loop_button).clicked() {
                    if let Err(e) = self.audio_player.toggle_loop(1) {
                        eprintln!("Error toggling loop: {}", e);
                    }
                }
            });

            // Volume slider
            let mut volume = self.audio_player.get_volume(1);
            ui.add(egui::Slider::new(&mut volume, 0.0..=1.0).text("Volume"));
            self.audio_player.set_volume(1, volume);

            // Add to playlist
            ui.horizontal(|ui| {
                ui.label("Add track:");
                let response = ui.text_edit_singleline(&mut self.new_track_input);
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if let Err(e) = self.audio_player.add_to_playlist(1, self.new_track_input.clone()) {
                        eprintln!("Error adding track to playlist: {}", e);
                    }
                    self.new_track_input.clear();
                }
                if ui.button("Add").clicked() {
                    if let Err(e) = self.audio_player.add_to_playlist(1, self.new_track_input.clone()) {
                        eprintln!("Error adding track to playlist: {}", e);
                    }
                    self.new_track_input.clear();
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