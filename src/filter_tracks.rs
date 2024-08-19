use crate::import_tracks::Track;

pub fn filter_tracks_by_comment(tracks: Vec<Track>, filter: &str) -> Vec<Track> {
    tracks.into_iter().filter(|track| track.comments_vec.contains(&filter.to_string())).collect()
}

pub fn get_all_comments(tracks: Vec<Track>) -> Vec<String> {
    tracks.into_iter().flat_map(|track| track.comments_vec).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::import_tracks;

    #[test]
    fn test_filter_tracks_by_comment() {
        let tracks = import_tracks::import_directory("/home/eggbert/songs");
        let filtered_tracks = filter_tracks_by_comment(tracks, "Moody");
        assert_eq!(filtered_tracks.len(), 2);
    }

    #[test]
    fn test_get_all_comments() {
        let tracks = import_tracks::import_directory("/home/eggbert/songs");
        let comments = get_all_comments(tracks);
        let expected_comments = vec!["Moody", "Atmospheric", "Exciting", "Energetic"];
        
        for comment in expected_comments {
            assert!(comments.contains(&comment.to_string()));
        }
    }
}