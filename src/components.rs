use eframe::egui;
use crate::stage;
use crate::playlist;
use crate::state::State;
use crate::audio;
use std::sync::mpsc;
use crate::utils::ToStringPath;

pub fn render_playlist
(
    ui: &mut egui::Ui, 
    playlist: &mut playlist::Playlist,
    state: &mut State,
    tx: &mpsc::Sender<audio::AudioCommand>,
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
                let track_path = track.path.to_string_path();
                tx.send(audio::AudioCommand::Play(track_path));
            }
        }
        if ui.button("Pause").clicked() {
            tx.send(audio::AudioCommand::Pause);
        }
        if ui.button("Resume").clicked() {
            tx.send(audio::AudioCommand::Resume);
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

pub fn render_tracks_list
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

pub fn render_filter_menu
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

pub fn render_tags_list
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