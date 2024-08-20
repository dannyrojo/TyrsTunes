use rusqlite::Connection;
use anyhow::{Result, Context};
use crate::track;

pub fn initialize_database(db_name: &str) -> Result<()> {
    let conn = Connection::open(db_name)
        .context("Failed to open database connection")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tracks (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            artist TEXT NOT NULL,
            tags TEXT,
            path TEXT NOT NULL UNIQUE
        )",
        [],
    ).context("Failed to create tracks table")?;
    Ok(())
}

pub fn add_track(db_name: &str, track: &track::Track) -> Result<()> {
    let conn = Connection::open(db_name)
        .context("Failed to open database connection")?;
    conn.execute(
        "INSERT INTO tracks (title, artist, tags, path) VALUES (?1, ?2, ?3, ?4)",
        [&track.title, &track.artist, &track.tags.join(","), &track.path],
    ).context("Failed to insert track into database")?;
    Ok(())
}

pub fn remove_track(db_name: &str, path: &str) -> Result<()> {
    let conn = Connection::open(db_name)
        .context("Failed to open database connection")?;
    conn.execute(
        "DELETE FROM tracks WHERE path = ?1",
        [path],
    ).context("Failed to remove track from database")?;
    Ok(())
}

pub fn get_tracks(db_name: &str) -> Result<Vec<track::Track>> {
    let conn = Connection::open(db_name)
        .context("Failed to open database connection")?;
    let mut stmt = conn.prepare("SELECT * FROM tracks")?;
    let tracks = stmt.query_map([], |row| {
        let tags_string: String = row.get(3)?;
        let tags = tags_string.split(',').map(|s| s.trim().to_string()).collect();
        Ok(track::Track {
            title: row.get(1)?,
            artist: row.get(2)?,
            tags,
            path: row.get(4)?,
        })
    })?;
    Ok(tracks.collect::<Result<Vec<_>, _>>()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_database_operations() -> Result<()> {
        let db_name = "test_tyrstunes.db";

        let _ = fs::remove_file(db_name);

        initialize_database(db_name)?;

        let test_track = track::Track {
            title: "Test Song".to_string(),
            artist: "Test Artist".to_string(),
            tags: vec!["rock".to_string(), "indie".to_string()],
            path: "/path/to/test/song.mp3".to_string(),
        };

        add_track(db_name, &test_track)?;

        let tracks = get_tracks(db_name)?;
        assert_eq!(tracks.len(), 1);
        assert_eq!(tracks[0].title, "Test Song");
        assert_eq!(tracks[0].artist, "Test Artist");
        assert_eq!(tracks[0].tags, vec!["rock", "indie"]);
        assert_eq!(tracks[0].path, "/path/to/test/song.mp3");

        remove_track(db_name, &test_track.path)?;

        let tracks_after_remove = get_tracks(db_name)?;
        assert_eq!(tracks_after_remove.len(), 0);

        fs::remove_file(db_name)?;

        Ok(())
    }
}