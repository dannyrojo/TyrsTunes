use eframe::egui;
use crate::stage;
use crate::playlist;
use crate::state::State;
use rodio::{Sink, Decoder, source::Source};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

pub fn playlist_panel
(
    ui: &mut egui::Ui, 
    width: f32, height: f32,
    title: &str,
    playlist: &mut playlist::Playlist,
    state: &mut State,
    stream_handle: &mut rodio::OutputStreamHandle,
    stream: &mut rodio::OutputStream,
)
{
    ui.vertical(|ui| {
        ui.set_width(width);
        ui.set_height(height);
        ui.label(title);
        ui.separator();
        ui.push_id(title, |ui| {
            egui::ScrollArea::vertical()
                .max_height(height)
                .max_width(width)
                .auto_shrink(false)
                .show(ui, |ui| {
                    render_playlist(ui, playlist, state, stream_handle, stream);
                });
        });
    });
}

pub fn tracks_panel
(
    ui: &mut egui::Ui,
    width: f32, height: f32,
    stage: &mut stage::Stage,
    playlist_1: &mut playlist::Playlist,
    playlist_2: &mut playlist::Playlist,
)
{
    ui.vertical(|ui| {
        ui.set_width(width);
        ui.set_height(height);
        ui.label("Tracks");
        ui.separator();
        egui::ScrollArea::vertical()
            .max_height(height)
            .max_width(width)
            .auto_shrink(false)
            .show(ui, |ui| {
                render_tracks_list(ui, stage, playlist_1, playlist_2); //tracks list
            });
    });
}

pub fn filter_panel
(
    ui: &mut egui::Ui, 
    width: f32, height: f32,
    stage: &mut stage::Stage, 
) 
{
    ui.vertical(|ui| {
        ui.set_width(width);
        ui.label("Tags");
        ui.separator();
        render_filter_menu(ui, stage); //filter menu
        ui.separator();
        render_tags_list(ui, stage); //tags list
    });
}

fn render_playlist
(
    ui: &mut egui::Ui, 
    playlist: &mut playlist::Playlist,
    state: &mut State,
    stream_handle: &mut rodio::OutputStreamHandle,
    stream: &mut rodio::OutputStream,
)
{
    let mut track_to_move: Option<(usize, usize)> = None;

    ui.horizontal(|ui| {
        if ui.button("Remove").clicked() && state.selected_track.is_some() {
            if let Some(index) = state.selected_track {
                playlist.playlist.remove(index);
                state.selected_track = None;
            }
        }
        if ui.button("Play").clicked() && state.selected_track.is_some() {
            if let Some(index) = state.selected_track {
                let track = &playlist.playlist[index];
                let track_path = track.path.to_str().unwrap();  
                let file = File::open(track_path).unwrap();
                let source = Decoder::new(BufReader::new(file)).unwrap();
                stream_handle.play_raw(source.convert_samples());
            }
        }
    });
    
    for (index, track) in playlist.playlist.iter().enumerate() {
        ui.horizontal(|ui| {
            if ui.button("U").clicked() && index > 0 {
                track_to_move = Some((index, index - 1));
            }
            if ui.button("D").clicked() && index < playlist.playlist.len() - 1 {
                track_to_move = Some((index, index + 1));
            }
            if ui.selectable_label(state.selected_track == Some(index), format!("{} - {}", track.title, track.artist)).clicked() {
                state.selected_track = Some(index);
            }
        });
    }
    
    if let Some((from_index, to_index)) = track_to_move {
        playlist.playlist.swap(from_index, to_index);
        if let Some(selected) = state.selected_track {
            if selected == from_index {
                state.selected_track = Some(to_index);
            } else if selected == to_index {
                state.selected_track = Some(from_index);
            }
        }
    }
}

fn render_tracks_list
(
    ui: &mut egui::Ui, 
    stage: &mut stage::Stage, 
    playlist_1: &mut playlist::Playlist, 
    playlist_2: &mut playlist::Playlist
) 
{
    stage.update_visible_tracks();
    for track in &stage.visible_tracks {
        ui.horizontal(|ui| {
            if ui.button("L").clicked() {
                playlist_1.playlist.push(track.clone());
            }
            if ui.button("R").clicked() {
                playlist_2.playlist.push(track.clone());
            }
            ui.label(format!("{} - {}", track.title, track.artist));
        });
    }
}

fn render_filter_menu
(
    ui: &mut egui::Ui, 
    stage: &mut stage::Stage
) 
{
    let filter_options = ["AND", "OR", "----"];
    let current_filter = match stage.filter {
        stage::Filter::And => "AND",
        stage::Filter::Or => "OR",
        stage::Filter::None => "----",
    };
    ui.menu_button(current_filter, |ui| {
        for option in filter_options {
            if ui.button(option).clicked() {
                let new_filter = match option {
                    "AND" => stage::Filter::And,
                    "OR" => stage::Filter::Or,
                    "----" => stage::Filter::None,
                    _ => unreachable!(),
                };
                ui.close_menu();
                stage.update_filter(new_filter);
            }
        }
    });
}

fn render_tags_list
(
    ui: &mut egui::Ui, 
    stage: &mut stage::Stage
) 
{
    let mut updated_tags = Vec::new();
    for tag in &stage.tags {
        let mut is_selected = stage.selected_tags.contains(tag);
        if ui.checkbox(&mut is_selected, tag).changed() {
            if is_selected {
                updated_tags.push(tag.clone());
            }
        } else if is_selected {
            updated_tags.push(tag.clone());
        }
    }
    stage.update_selected_tags(updated_tags);
}