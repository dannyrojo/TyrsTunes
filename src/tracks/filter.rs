use super::models::Track;
use itertools::Itertools;

enum FilterMode {
    And,
    Or,
    Both,
}

fn filter_tracks_by_comments(tracks: Vec<Track>, filters: &[&str], mode: FilterMode) -> Vec<Track> {
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

fn get_all_comments_unique(tracks: Vec<Track>) -> Vec<String> {
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
    use std::path::PathBuf;

    fn create_test_tracks() -> Vec<Track> {
        vec![
            Track::new(
                "Divine1".to_string(),
                "Gar".to_string(),
                vec!["Moody".to_string(), "Atmospheric".to_string()],
                PathBuf::from("/fake/path/divine1.mp3"),
            ),
            Track::new(
                "Divine2".to_string(),
                "Yar".to_string(),
                vec!["Moody".to_string(), "Exciting".to_string()],
                PathBuf::from("/fake/path/divine2.mp3"),
            ),
            Track::new(
                "Powerful".to_string(),
                "Gar".to_string(),
                vec!["Atmospheric".to_string(), "Energetic".to_string()],
                PathBuf::from("/fake/path/powerful.mp3"),
            ),
        ]
    }

    #[test]
    fn test_filter_tracks_by_comments() {
        let tracks = create_test_tracks();

        let filtered_tracks = filter_tracks_by_comments(tracks, &["Moody", "Atmospheric"], FilterMode::And);
        assert_eq!(filtered_tracks.len(), 1);
        assert_eq!(filtered_tracks[0].title, "Divine1");
        assert_eq!(filtered_tracks[0].artist, "Gar");
        assert_eq!(filtered_tracks[0].comments_vec, vec!["Moody".to_string(), "Atmospheric".to_string()]);
    }

    #[test]
    fn test_get_all_comments_unique() {
        let tracks = create_test_tracks();

        let comments = get_all_comments_unique(tracks);
        let expected_comments = vec!["Atmospheric".to_string(), "Energetic".to_string(), "Exciting".to_string(), "Moody".to_string()];
        assert_eq!(comments, expected_comments);
    }
}