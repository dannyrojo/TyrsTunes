use std::path::PathBuf;

#[derive(Clone)]
pub struct Track {
    pub title: String,
    pub artist: String,
    pub tags: String,
    pub path: PathBuf,
}