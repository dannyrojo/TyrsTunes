use eframe::egui;
use crate::audio_backend::AudioPlayer;
use crate::database::{Database, Track, extract_metadata};
use anyhow::Result;
use walkdir::WalkDir;
use rfd::FileDialog;

pub struct MyApp {
    audio_player: AudioPlayer,
    database: Database,
    staged_tracks: Vec<Track>,
    playlist: Vec<Track>,
    current_track_index: Option<usize>,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Result<Self> {
        Ok(Self {
            audio_player: AudioPlayer::new()?,
            database: Database::new("tracks.db")?,
            staged_tracks: Vec::new(),
            playlist: Vec::new(),
            current_track_index: None,
        })
    }

    fn load_directory(&mut self, path: &str) -> Result<()> {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Ok(track) = extract_metadata(entry.path()) {
                    self.database.insert_track(&track)?;
                    self.staged_tracks.push(track);
                }
            }
        }
        Ok(())
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TyrsTunes");

            // Add directory button
            if ui.button("Add Directory").clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    if let Err(e) = self.load_directory(path.to_str().unwrap()) {
                        eprintln!("Error loading directory: {}", e);
                    }
                }
            }

            ui.separator();

            // Staged tracks
            ui.push_id("staged_tracks", |ui| {
                ui.heading("Staged Tracks");
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (index, track) in self.staged_tracks.iter().enumerate() {
                        ui.push_id(index, |ui| {
                            ui.horizontal(|ui| {
                                if ui.button("+").clicked() {
                                    self.playlist.push(track.clone());
                                }
                                ui.label(&track.title);
                            });
                        });
                    }
                });
            });

            ui.separator();

            // Playlist
            ui.push_id("playlist", |ui| {
                ui.heading("Playlist");
                let mut tracks_to_remove = Vec::new();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (index, track) in self.playlist.iter().enumerate() {
                        ui.push_id(index, |ui| {
                            ui.horizontal(|ui| {
                                if ui.button("-").clicked() {
                                    tracks_to_remove.push(index);
                                }
                                ui.label(&track.title);
                            });
                        });
                    }
                });
                // Remove tracks in reverse order to maintain correct indices
                for &index in tracks_to_remove.iter().rev() {
                    self.playlist.remove(index);
                }
            });

            ui.separator();

            // Playback controls
            ui.horizontal(|ui| {
                if ui.button(if self.audio_player.is_playing(1) { "⏸ Pause" } else { "▶ Play" }).clicked() {
                    if self.audio_player.is_playing(1) {
                        self.audio_player.stop(1);
                    } else if let Err(e) = self.audio_player.play_or_resume(1) {
                        eprintln!("Error playing/resuming: {}", e);
                    }
                }

                if ui.button("⏹ Stop").clicked() {
                    self.audio_player.stop(1);
                }

                if ui.button("⏭ Skip").clicked() {
                    if let Err(e) = self.audio_player.skip(1) {
                        eprintln!("Error skipping track: {}", e);
                    }
                }

                let mut volume = self.audio_player.get_volume(1);
                ui.add(egui::Slider::new(&mut volume, 0.0..=1.0).text("Volume"));
                self.audio_player.set_volume(1, volume);
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