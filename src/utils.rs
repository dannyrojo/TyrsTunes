use std::path::{Path, PathBuf};

pub trait ToPathBuf {
    fn to_pathbuf(&self) -> PathBuf;
}

pub trait ToStringPath {
    fn to_string_path(&self) -> String;
}

impl ToPathBuf for str {
    fn to_pathbuf(&self) -> PathBuf {
        PathBuf::from(self)
    }
}

impl ToPathBuf for String {
    fn to_pathbuf(&self) -> PathBuf {
        PathBuf::from(self)
    }
}

impl ToStringPath for Path {
    fn to_string_path(&self) -> String {
        self.to_string_lossy().into_owned()
    }
}

impl ToStringPath for PathBuf {
    fn to_string_path(&self) -> String {
        self.to_string_lossy().into_owned()
    }
}
