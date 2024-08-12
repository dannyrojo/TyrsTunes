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
    pub comments: Vec<String>,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path).context("Failed to open database connection")?;
        let db = Database { conn };
        db.create_tables()?;
        Ok(db)
    }

    fn create_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tracks (
                id INTEGER PRIMARY KEY,
                file_path TEXT NOT NULL UNIQUE,
                title TEXT,
                artist TEXT,
                length INTEGER
            )",
            [],
        ).context("Failed to create tracks table")?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
            )",
            [],
        ).context("Failed to create tags table")?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS track_tags (
                track_id INTEGER,
                tag_id INTEGER,
                PRIMARY KEY (track_id, tag_id),
                FOREIGN KEY (track_id) REFERENCES tracks(id),
                FOREIGN KEY (tag_id) REFERENCES tags(id)
            )",
            [],
        ).context("Failed to create track_tags table")?;

        Ok(())
    }

    pub fn insert_track(&mut self, track: &Track) -> Result<()> {
        let tx = self.conn.transaction()?;

        tx.execute(
            "INSERT OR REPLACE INTO tracks (file_path, title, artist, length) VALUES (?1, ?2, ?3, ?4)",
            (&track.file_path, &track.title, &track.artist, &track.length),
        ).context("Failed to insert track")?;

        let track_id = tx.last_insert_rowid();

        for tag in &track.comments {
            tx.execute(
                "INSERT OR IGNORE INTO tags (name) VALUES (?1)",
                [tag],
            ).context("Failed to insert tag")?;

            let tag_id: i64 = tx.query_row(
                "SELECT id FROM tags WHERE name = ?1",
                [tag],
                |row| row.get(0),
            ).context("Failed to get tag id")?;

            tx.execute(
                "INSERT OR IGNORE INTO track_tags (track_id, tag_id) VALUES (?1, ?2)",
                (track_id, tag_id),
            ).context("Failed to insert track_tag relation")?;
        }

        tx.commit()?;
        Ok(())
    }

    pub fn get_all_tracks(&self) -> Result<Vec<Track>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.id, t.file_path, t.title, t.artist, t.length, GROUP_CONCAT(tg.name, ',')
             FROM tracks t
             LEFT JOIN track_tags tt ON t.id = tt.track_id
             LEFT JOIN tags tg ON tt.tag_id = tg.id
             GROUP BY t.id"
        ).context("Failed to prepare statement")?;

        let tracks_iter = stmt.query_map([], |row| {
            let comments_str: Option<String> = row.get(5)?;
            let comments = comments_str
                .map(|s| s.split(',').map(|t| t.trim().to_string()).collect())
                .unwrap_or_default();

            Ok(Track {
                id: row.get(0)?,
                file_path: row.get(1)?,
                title: row.get(2)?,
                artist: row.get(3)?,
                length: row.get(4)?,
                comments,
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
    
    let comments = tag.comments()
        .next()
        .map(|c| c.text.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    Ok(Track {
        id: 0,
        file_path: file_path.to_str().unwrap().to_string(),
        title: tag.title().unwrap_or("Unknown").to_string(),
        artist: tag.artist().unwrap_or("Unknown").to_string(),
        length: tag.duration().unwrap_or(0) as i32,
        comments,
    })
}