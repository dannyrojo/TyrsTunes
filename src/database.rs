use rusqlite::Connection;
use anyhow::{Result, Context};
use crate::track;
use crate::utils::{ToStringPath, ToPathBuf};

pub fn create_table(db_name: &str) -> Result<()> {
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

pub fn delete_table(db_name: &str) -> Result<()> {
    let conn = Connection::open(db_name)
        .context("Failed to open database connection")?;
    conn.execute(
        "DROP TABLE IF EXISTS tracks",
        [],
    ).context("Failed to delete tracks table")?;
    Ok(())
}

pub fn add_track(db_name: &str, track: &track::Track) -> Result<()> {
    let conn = Connection::open(db_name)
        .context("Failed to open database connection")?;
    conn.execute(
        "INSERT INTO tracks (title, artist, tags, path) VALUES (?1, ?2, ?3, ?4)",
        [
            &track.title,
            &track.artist,
            &track.tags,
            &track.path.to_string_path(),
        ],
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

// check if track exists by path, if it does, update it
pub fn update_track(db_name: &str, track: &track::Track) -> Result<()> {
    let conn = Connection::open(db_name)
        .context("Failed to open database connection")?;
    conn.execute(
        "UPDATE tracks SET title = ?1, artist = ?2, tags = ?3, path = ?4",
        [
            &track.title,
            &track.artist,
            &track.tags,
            &track.path.to_string_path(),
        ],
    ).context("Failed to update track in database")?;
    Ok(())
}

pub fn get_tracks(db_name: &str) -> Result<Vec<track::Track>> {
    let conn = Connection::open(db_name)
        .context("Failed to open database connection")?;
    let mut stmt = conn.prepare("SELECT * FROM tracks")?;
    let tracks = stmt.query_map([], |row| {
        let path_string: String = row.get(4)?;
        Ok(track::Track {
            title: row.get(1)?,
            artist: row.get(2)?,
            tags: row.get(3)?,
            path: path_string.to_pathbuf(),
        })
    })?;
    Ok(tracks.collect::<Result<Vec<_>, _>>()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_database_operations() -> Result<()> {
        let db_name = "test_tyrstunes.db";

        let _ = fs::remove_file(db_name);

        create_table(db_name)?;

        let test_track = track::Track {
            title: "Test Song".to_string(),
            artist: "Test Artist".to_string(),
            tags: "rock,indie".to_string(),
            path: PathBuf::from("/path/to/test/song.mp3"),
        };

        add_track(db_name, &test_track)?;

        let tracks = get_tracks(db_name)?;
        assert_eq!(tracks.len(), 1);
        assert_eq!(tracks[0].title, "Test Song");
        assert_eq!(tracks[0].artist, "Test Artist");
        assert_eq!(tracks[0].tags, "rock,indie");
        assert_eq!(tracks[0].path, PathBuf::from("/path/to/test/song.mp3"));

        remove_track(db_name, &test_track.path.to_string_path())?;

        let tracks_after_remove = get_tracks(db_name)?;
        assert_eq!(tracks_after_remove.len(), 0);

        fs::remove_file(db_name)?;

        Ok(())
    }
}