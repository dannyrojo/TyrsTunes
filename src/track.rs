use std::path::PathBuf;

pub struct Track {
    pub title: String,
    pub artist: String,
    pub tags: Vec<String>,
    pub path: PathBuf,
}

