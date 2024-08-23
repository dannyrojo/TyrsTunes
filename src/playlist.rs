use crate::track::Track;

pub struct Playlist {
    pub name: String,
    pub playlist: Vec<Track>,
}

impl Playlist {
    pub fn new(name: String) -> Self {
        Self {
            name,
            playlist: Vec::new(),
        }
    }
    
}

pub fn initialize_playlists() -> (Playlist, Playlist) {
    let playlist_1 = Playlist::new(String::from("Playlist 1"));
    let playlist_2 = Playlist::new(String::from("Playlist 2"));
    (playlist_1, playlist_2)
}