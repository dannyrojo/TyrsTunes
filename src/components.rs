use eframe::egui;
use crate::stage;
use crate::playlist;

pub fn render_playlist_panel
(
    ui: &mut egui::Ui, 
    width: f32, 
    title: &str,
    playlist: &mut playlist::Playlist,
)

{
    ui.vertical(|ui| {
        ui.set_width(width);
        ui.label(title);
        ui.separator();
        
        let mut track_to_remove: Option<usize> = None;
        
        for (index, track) in playlist.playlist.iter().enumerate() {
            ui.horizontal(|ui| {
                if ui.button("-").clicked() {
                    track_to_remove = Some(index);
                }
                ui.label(format!("{} - {}", track.title, track.artist));
            });
        }
        
        if let Some(index) = track_to_remove {
            playlist.playlist.remove(index);
        }
    });
}

pub fn render_tracks_panel
(
    ui: &mut egui::Ui,
    stage: &mut stage::Stage,
    width: f32, height: f32,
    playlist_1: &mut playlist::Playlist,
    playlist_2: &mut playlist::Playlist,
)

{
    ui.vertical(|ui| {
        ui.set_width(width);
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

pub fn render_filter_panel
(
    ui: &mut egui::Ui, 
    stage: &mut stage::Stage, 
    width: f32, height: f32,
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

fn render_filter_menu(ui: &mut egui::Ui, stage: &mut stage::Stage) {
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

fn render_tags_list(ui: &mut egui::Ui, stage: &mut stage::Stage) {
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