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
    pub fn add_track_to_playlist(&mut self, track: Track) {
        self.playlist.push(track);
    }
    pub fn remove_track_from_playlist(&mut self, index: usize) {
        self.playlist.remove(index);
    }
    pub fn move_track_in_playlist(&mut self, from_index: usize, to_index: usize) {
        if from_index != to_index {
            let track = self.playlist.remove(from_index);
            let insert_at = if to_index > from_index { to_index - 1 } else { to_index };
            self.playlist.insert(insert_at, track);
        }
    }
    
}

pub fn initialize_playlists() -> (Playlist, Playlist) {
    let playlist_1 = Playlist::new(String::from("Playlist 1"));
    let playlist_2 = Playlist::new(String::from("Playlist 2"));
    (playlist_1, playlist_2)
}