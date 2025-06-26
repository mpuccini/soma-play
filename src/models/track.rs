#[derive(Debug, Clone)]
pub struct TrackInfo {
    pub artist: String,
    pub title: String,
}

impl Default for TrackInfo {
    fn default() -> Self {
        Self {
            artist: "Unknown".to_string(),
            title: "Loading...".to_string(),
        }
    }
}

/// Parses track info from ICY stream title
pub fn parse_track_info(stream_title: &str) -> TrackInfo {
    // Try to split on " - " to separate artist and title
    if let Some(dash_pos) = stream_title.find(" - ") {
        let artist = stream_title[..dash_pos].trim().to_string();
        let title = stream_title[dash_pos + 3..].trim().to_string();
        
        if !artist.is_empty() && !title.is_empty() {
            return TrackInfo {
                artist,
                title,
            };
        }
    }
    
    // If no " - " found, use the entire string as title
    TrackInfo {
        artist: "Unknown".to_string(),
        title: stream_title.to_string(),
    }
}
