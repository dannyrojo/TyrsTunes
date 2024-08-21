use nfd::Response;
use walkdir::WalkDir;
use id3::{Tag, TagLike};
use anyhow::{Result, Context};
use std::path::PathBuf;
use crate::track;
use crate::database;


fn select_directory() -> Result<PathBuf> {
    let result = nfd::open_pick_folder(None)
        .context("Failed to open directory dialog")?;

    match result {
        Response::Okay(directory_path) => Ok(PathBuf::from(directory_path)),
        Response::Cancel => Err(anyhow::Error::msg("No directory selected")),
        _ => Err(anyhow::Error::msg("Unexpected response")),
    }
}

fn import_tracks_from_directory(target_db: &str, directory: &PathBuf) -> Result<()> {
    for entry in WalkDir::new(directory) {
        let entry = entry?;
        if entry.file_type().is_file() && entry.path().extension().map(|ext| ext == "mp3").unwrap_or(false) {
            let metadata = Tag::read_from_path(entry.path())?;
            let comments = metadata.comments()
                .next()
                .map(|comment| comment.text.clone())
                .unwrap_or_else(|| "None".to_string());
            let track = track::Track {
                title: metadata.title().unwrap_or("Unknown Title").to_string(),
                artist: metadata.artist().unwrap_or("Unknown Artist").to_string(),
                tags: comments,
                path: entry.path().to_path_buf(),
            };
            database::add_track(target_db, &track)?;
        }
    }
    
    Ok(())
}

pub fn import_tracks_button(db_name: &str) -> Result<()> {
    if let Ok(directory) = select_directory() {
        database::delete_table(db_name).context("Failed to delete table")?;
        database::create_table(db_name).context("Failed to create table")?;
        import_tracks_from_directory(db_name, &directory).context("Failed to import tracks")?;
        Ok(())
    } else {
        println!("No directory selected. Exiting import process.");
        Ok(())
    }
}