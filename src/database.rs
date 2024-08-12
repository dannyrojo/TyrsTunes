use rusqlite::Connection;
use id3::{Tag, TagLike};
use std::path::Path;
use anyhow::{Result, Context};

#[derive(Clone)]
pub struct Track {
    pub id: i64,
    pub file_path: String,
    pub title: String,
    pub artist: String,
    pub length: i32,
    pub tags: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path).context("Failed to open database connection")?;
        let db = Database { conn };
        db.create_table()?;
        Ok(db)
    }

    fn create_table(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tracks (
                id INTEGER PRIMARY KEY,
                file_path TEXT NOT NULL UNIQUE,
                title TEXT,
                artist TEXT,
                length INTEGER,
                tags TEXT
            )",
            [],
        ).context("Failed to create table")?;
        Ok(())
    }

    pub fn insert_track(&self, track: &Track) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO tracks (file_path, title, artist, length, tags) VALUES (?1, ?2, ?3, ?4, ?5)",
            (&track.file_path, &track.title, &track.artist, &track.length, &track.tags),
        ).context("Failed to insert track")?;
        Ok(())
    }

    pub fn get_all_tracks(&self) -> Result<Vec<Track>> {
        let mut stmt = self.conn.prepare("SELECT id, file_path, title, artist, length, tags FROM tracks")
            .context("Failed to prepare statement")?;
        let tracks_iter = stmt.query_map([], |row| {
            Ok(Track {
                id: row.get(0)?,
                file_path: row.get(1)?,
                title: row.get(2)?,
                artist: row.get(3)?,
                length: row.get(4)?,
                tags: row.get(5)?,
            })
        }).context("Failed to query tracks")?;

        let mut tracks = Vec::new();
        for track in tracks_iter {
            tracks.push(track.context("Failed to process track")?);
        }
        Ok(tracks)
    }
}

pub fn extract_metadata(file_path: &Path) -> Result<Track> {
    let tag = Tag::read_from_path(file_path).context("Failed to read ID3 tag")?;
    
    let tags = tag.extended_texts()
        .next()
        .map(|t| t.value.clone())
        .unwrap_or_default();

    Ok(Track {
        id: 0,
        file_path: file_path.to_str().unwrap().to_string(),
        title: tag.title().unwrap_or("Unknown").to_string(),
        artist: tag.artist().unwrap_or("Unknown").to_string(),
        length: tag.duration().unwrap_or(0) as i32,
        tags,
    })
}