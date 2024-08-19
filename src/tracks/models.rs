use std::path::{Path, PathBuf};

pub struct Track {
    pub title: String,
    pub artist: String,
    pub comments_vec: Vec<String>,
    path: PathBuf,
}

impl Track {
    pub(crate) fn new(title: String, artist: String, comments_vec: Vec<String>, path: PathBuf) -> Self {
        Self { title, artist, comments_vec, path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
