use anyhow::{Context, Result};
use std::path::Path;
use crate::database;
use crate::track;
use crate::import;

pub enum Filter {
    And,
    Or,
    None,
}

pub struct Stage {
    pub tracks: Vec<track::Track>,
    pub visible_tracks: Vec<track::Track>,
    pub tags: Vec<String>,
    pub selected_tags: Vec<String>,
    pub filter: Filter,
}

pub fn initialize_stage(db_name: &str) -> Result<Stage> {
    let db_path = Path::new(db_name);
    if !db_path.exists() {
        import::import_tracks_button(db_name)?;
    }

    let tracks = database::get_tracks(db_name)
        .context("Failed to get tracks from database")?;
    
    let tags: Vec<String> = tracks
        .iter()
        .flat_map(|track| track.tags.split(',').map(|tag| tag.trim_start().to_string()))
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let stage = Stage {
        tracks,
        visible_tracks: Vec::new(),
        tags,
        selected_tags: Vec::new(),
        filter: Filter::None,
    };

    Ok(stage)
}

impl Stage {
    pub fn update_visible_tracks(&mut self) {
        self.visible_tracks.clear();

        if self.selected_tags.is_empty() || matches!(self.filter, Filter::None) {
            self.visible_tracks = self.tracks.clone();
            return;
        }

        for track in &self.tracks {
            let track_tags: Vec<&str> = track.tags.split(',').map(str::trim_start).collect();
            let matches = match self.filter {
                Filter::And => self.selected_tags.iter().all(|tag| track_tags.contains(&tag.as_str())),
                Filter::Or => self.selected_tags.iter().any(|tag| track_tags.contains(&tag.as_str())),
                Filter::None => true, 
            };

            if matches {
                self.visible_tracks.push(track.clone());
            }
        }
    }

    pub fn update_selected_tags(&mut self, gui_tags: Vec<String>) {
        self.selected_tags = gui_tags;
        self.update_visible_tracks();
    }

    pub fn update_filter(&mut self, new_filter: Filter) {
        self.filter = new_filter;
        self.update_visible_tracks();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_test_tracks() -> Vec<track::Track> {
        vec![
            track::Track {
                title: "Song 1".to_string(),
                artist: "Artist A".to_string(),
                tags: "rock,90s".to_string(),
                path: PathBuf::from("/music/song1.mp3"),
            },
            track::Track {
                title: "Song 2".to_string(),
                artist: "Artist B".to_string(),
                tags: "pop,80s".to_string(),
                path: PathBuf::from("/music/song2.mp3"),
            },
            track::Track {
                title: "Song 3".to_string(),
                artist: "Artist C".to_string(),
                tags: "rock,80s".to_string(),
                path: PathBuf::from("/music/song3.mp3"),
            },
        ]
    }

    #[test]
    fn test_update_visible_tracks_no_filter() {
        let mut stage = Stage {
            tracks: create_test_tracks(),
            visible_tracks: Vec::new(),
            tags: vec!["rock".to_string(), "pop".to_string(), "80s".to_string(), "90s".to_string()],
            selected_tags: Vec::new(),
            filter: Filter::None,
        };

        stage.update_visible_tracks();
        assert_eq!(stage.visible_tracks.len(), 3);
    }

    #[test]
    fn test_update_visible_tracks_and_filter() {
        let mut stage = Stage {
            tracks: create_test_tracks(),
            visible_tracks: Vec::new(),
            tags: vec!["rock".to_string(), "pop".to_string(), "80s".to_string(), "90s".to_string()],
            selected_tags: vec!["rock".to_string(), "80s".to_string()],
            filter: Filter::And,
        };

        stage.update_visible_tracks();
        assert_eq!(stage.visible_tracks.len(), 1);
        assert_eq!(stage.visible_tracks[0].title, "Song 3");
    }

    #[test]
    fn test_update_visible_tracks_or_filter() {
        let mut stage = Stage {
            tracks: create_test_tracks(),
            visible_tracks: Vec::new(),
            tags: vec!["rock".to_string(), "pop".to_string(), "80s".to_string(), "90s".to_string()],
            selected_tags: vec!["rock".to_string(), "pop".to_string()],
            filter: Filter::Or,
        };

        stage.update_visible_tracks();
        assert_eq!(stage.visible_tracks.len(), 3);
    }

    #[test]
    fn test_update_selected_tags() {
        let mut stage = Stage {
            tracks: create_test_tracks(),
            visible_tracks: Vec::new(),
            tags: vec!["rock".to_string(), "pop".to_string(), "80s".to_string(), "90s".to_string()],
            selected_tags: Vec::new(),
            filter: Filter::And,
        };

        stage.update_selected_tags(vec!["rock".to_string()]);
        assert_eq!(stage.selected_tags, vec!["rock".to_string()]);
        assert_eq!(stage.visible_tracks.len(), 2);
    }

    #[test]
    fn test_update_filter() {
        let mut stage = Stage {
            tracks: create_test_tracks(),
            visible_tracks: Vec::new(),
            tags: vec!["rock".to_string(), "pop".to_string(), "80s".to_string(), "90s".to_string()],
            selected_tags: vec!["rock".to_string(), "80s".to_string()],
            filter: Filter::And,
        };

        stage.update_filter(Filter::Or);
        assert!(matches!(stage.filter, Filter::Or));
        assert_eq!(stage.visible_tracks.len(), 3);
    }

}