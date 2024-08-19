use crate::import_tracks::Track;
use itertools::Itertools; // Added this import at the top of the file

pub enum FilterMode {
    And,
    Or,
    Both,
}

pub fn filter_tracks_by_comments(tracks: Vec<Track>, filters: &[&str], mode: FilterMode) -> Vec<Track> {
    tracks.into_iter().filter(|track| {
        match mode {
            FilterMode::And => filters.iter().all(|&filter| track.comments_vec.contains(&filter.to_string())),
            FilterMode::Or => filters.iter().any(|&filter| track.comments_vec.contains(&filter.to_string())),
            FilterMode::Both => {
                let and_result = filters.iter().all(|&filter| track.comments_vec.contains(&filter.to_string()));
                let or_result = filters.iter().any(|&filter| track.comments_vec.contains(&filter.to_string()));
                and_result || or_result
            }
        }
    }).collect()
}

pub fn get_all_comments_unique(tracks: Vec<Track>) -> Vec<String> {
    let mut comments: Vec<String> = tracks
        .into_iter()
        .flat_map(|track| track.comments_vec)
        .unique()
        .collect();
    comments.sort();
    comments
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::import_tracks;

    #[test]
    fn test_filter_tracks_by_comments() {
        let tracks = import_tracks::import_directory("/home/eggbert/songs");
        let filtered_tracks = filter_tracks_by_comments(tracks, &["Energetic", "Atmospheric"], FilterMode::And);
        assert_eq!(filtered_tracks.len(), 1);
    }


    #[test]
    fn test_get_all_comments_unique() {
        let tracks = import_tracks::import_directory("/home/eggbert/songs");
        let comments = get_all_comments_unique(tracks);
        let expected_comments = vec!["Atmospheric", "Energetic", "Exciting", "Moody"];
        assert_eq!(comments, expected_comments);
    }
}