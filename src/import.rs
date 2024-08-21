use rfd::FileDialog;
use walkdir::WalkDir;
use id3::{Tag, TagLike};
use anyhow::{Result, anyhow};
use std::path::PathBuf;
use crate::track;
use crate::database;

use nfd::Response;


pub fn select_directory() -> Result<PathBuf> {
    let result = nfd::open_pick_folder(None)
        .map_err(|e| anyhow!("Failed to open directory dialog: {}", e))?;

    match result {
        Response::Okay(directory_path) => Ok(PathBuf::from(directory_path)),
        Response::Cancel => Err(anyhow!("No directory selected")),
        _ => Err(anyhow!("Unexpected response")),
    }
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