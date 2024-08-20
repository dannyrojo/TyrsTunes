use rfd::FileDialog;
use walkdir::WalkDir;
use id3::{Tag, TagLike};
use anyhow::{Result, anyhow};
use std::path::PathBuf;
use crate::track;
use crate::database;

pub fn select_directory() -> Result<PathBuf> {
    FileDialog::new()
        .set_title("Select a directory with mp3 files")
        .pick_folder()
        .ok_or_else(|| anyhow!("No directory selected"))
}

pub fn import_tracks_from_directory(target_db: &str, directory: &PathBuf) -> Result<()> {
    for entry in WalkDir::new(directory) {
        let entry = entry?;
        if entry.file_type().is_file() && entry.path().extension().map(|ext| ext == "mp3").unwrap_or(false) {
            let metadata = Tag::read_from_path(entry.path())?;
            let track = track::Track {
                title: metadata.title().unwrap_or("Unknown Title").to_string(),
                artist: metadata.artist().unwrap_or("Unknown Artist").to_string(),
                tags: vec![metadata.genre().unwrap_or("Unknown Genre").to_string()],
                path: entry.path().to_path_buf(),
            };
            database::add_track(target_db, &track)?;
        }
    }
    Ok(())
}