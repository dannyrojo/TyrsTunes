use rusqlite::Connection;
use id3::{Tag, TagLike};
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use rfd::FileDialog;
use std::env;

pub struct Track {
    pub title: String,
    pub artist: String,
    pub comments_vec: Vec<String>,
    path: PathBuf,
}

impl Track {
    pub fn new(title: String, artist: String, comments_vec: Vec<String>, path: PathBuf) -> Self {
        Self { title, artist, comments_vec, path }
    }
    pub fn path(&self) -> &Path { 
        &self.path
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn artist(&self) -> &str {
        &self.artist
    }
    pub fn comments(&self) -> &[String] {
        &self.comments_vec
    }
}

// returns a PathBuf
fn select_directory() -> Option<PathBuf> {
    let default_directory = if cfg!(target_os = "windows") {
        PathBuf::from("C:\\")
    } else {
        PathBuf::from("/")
    };

    FileDialog::new()
        .set_title("Select a folder for MP3 files")
        .set_directory(default_directory)
        .pick_folder()
}

// returns a vector of Track structs
fn import_tracks_from_directory(dir_path: &Path) -> Vec<Track> {
    let mut tracks = Vec::new();
    for entry in WalkDir::new(dir_path).into_iter().filter_map(|entry| entry.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("mp3") {
            if let Ok(tag) = Tag::read_from_path(path) {
               
                let comments: Vec<String> = tag.comments()
                    .flat_map(|comment| comment.text.split(','))
                    .map(|s| s.trim().to_string())
                    .collect();

                tracks.push(Track::new(
                    tag.title().unwrap_or_default().to_string(),
                    tag.artist().unwrap_or_default().to_string(),
                    comments,
                    path.to_path_buf(),
                ));
            }
        }
    }
    tracks 
}

// returns a boolean indicating if the table was created
fn initialize_database() -> Result<bool, rusqlite::Error> {
    let db_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "tracks.db".to_string());
    let conn = Connection::open(db_name)?;
    
    let table_exists: bool = conn.query_row(
        "SELECT 1 FROM sqlite_master WHERE type='table' AND name='tracks'",
        [],
        |row| row.get(0),
    ).unwrap_or(false);

    if !table_exists {
        conn.execute(
            "CREATE TABLE tracks (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                artist TEXT NOT NULL,
                comments_vec TEXT NOT NULL,
                file_path TEXT NOT NULL UNIQUE
            )",
            [],
        )?;
        Ok(true) 
    } else {
        Ok(false) 
    }
}

// returns nothing
fn store_metadata_in_database(tracks: Vec<Track>) -> Result<(), Box<dyn std::error::Error>> {
    let db_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "tracks.db".to_string());
    let conn = Connection::open(db_name)?;
    let mut stmt = conn.prepare("INSERT INTO tracks (title, artist, comments_vec, file_path) VALUES (?, ?, ?, ?)")?;

    for track in tracks {
        stmt.execute((
            &track.title,
            &track.artist,
            &track.comments_vec.join(","),
            track.path().to_str().ok_or("Invalid path")?,
        ))?;
    }
    
    Ok(())
}

// returns a vector of Track 
fn retrieve_tracks_from_database() -> Result<Vec<Track>, Box<dyn std::error::Error>> {
    let db_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "tracks.db".to_string());
    let conn = Connection::open(db_name)?;
    let mut stmt = conn.prepare("SELECT * FROM tracks")?;
    let tracks_iter = stmt.query_map([], |row| {
        Ok(Track::new(
            row.get(1)?,
            row.get(2)?,
            row.get::<_, String>(3)?.split(',').map(String::from).collect(),
            PathBuf::from(row.get::<_, String>(4)?),
        ))
    })?;
    
    let mut tracks = Vec::new();
    for track in tracks_iter {
        tracks.push(track?);
    }

    Ok(tracks)
}

// callback function for load tracks button
pub fn load_tracks_button() {
    let dir_path = select_directory().unwrap();
    let tracks = import_tracks_from_directory(&dir_path);
    store_metadata_in_database(tracks).unwrap();
}


// tests for database operations
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn create_test_track(title: &str, artist: &str, comments: Vec<&str>, path: &str) -> Track {
        Track::new(
            title.to_string(),
            artist.to_string(),
            comments.into_iter().map(String::from).collect(),
            PathBuf::from(path),
        )
    }

    #[test]
    fn test_database_operations() -> Result<(), Box<dyn std::error::Error>> {
        
        let db_name = "test_tracks.db";
        env::set_var("DATABASE_NAME", db_name);
        let table_created = initialize_database()?;

        let test_tracks = vec![
            create_test_track("Title 1", "Artist 1", vec!["Comment 1", "Comment 2"], "path1.mp3"),
            create_test_track("Title 2", "Artist 2", vec!["Comment 3", "Comment 4"], "path2.mp3"),
        ];
        
        store_metadata_in_database(test_tracks)?;
        let retrieved_tracks = retrieve_tracks_from_database()?;

        assert_eq!(retrieved_tracks.len(), 2);
        
        assert_eq!(retrieved_tracks[1].title(), "Title 2");
        assert_eq!(retrieved_tracks[1].artist(), "Artist 2");
        assert_eq!(retrieved_tracks[1].comments(), &["Comment 3", "Comment 4"]);
        assert_eq!(retrieved_tracks[1].path(), Path::new("path2.mp3"));

        // Test storing duplicate track
        let duplicate_track = create_test_track("Duplicate", "Artist", vec!["Comment"], "path1.mp3");
        let result = store_metadata_in_database(vec![duplicate_track]);
        assert!(result.is_err(), "Storing duplicate track should fail");

        fs::remove_file(db_name)?;

        Ok(())
    }
}