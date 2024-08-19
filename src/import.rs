use id3::{Tag, TagLike};
use walkdir::WalkDir;

pub struct Track {
    pub title: String,
    pub artist: String,
    pub comments_vec: Vec<String>,
}

pub fn import_track(track_path: &str) -> Track {
    let tag = Tag::read_from_path(track_path).unwrap(); 
    let title = tag.title().unwrap_or_default().to_string();
    let artist = tag.artist().unwrap_or_default().to_string();
    let comment_frame = tag.comments();
    let comments_vec: Vec<String> = comment_frame
        .flat_map(|comment| comment.text.split(','))
        .map(|s| s.trim().to_string())
        .collect();
    Track {
        title,
        artist,
        comments_vec,
    }
}

fn import_directory(dir_path: &str) -> Vec<Track> {
    let mut tracks = vec![];
    for entry in WalkDir::new(dir_path) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "mp3" {
            tracks.push(import_track(&path.to_str().unwrap()));
        }
    }
    tracks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_track() {
        let track = import_track("/home/eggbert/songs/Divine1.mp3");
        assert_eq!(track.title, "Divine1");
        assert_eq!(track.artist, "Gar");
        assert_eq!(track.comments_vec, vec!["Moody", "Atmospheric"]);
    }
    #[test]
    fn test_import_directory() {
        let tracks = import_directory("/home/eggbert/songs");
        assert_eq!(tracks.len(), 3);
    }
}