//! Track information and metadata parsing.
//!
//! This module provides data structures and functions for handling
//! track metadata from ICY streams.

/// Represents information about the currently playing track.
///
/// This information is typically extracted from ICY metadata
/// sent by the audio stream.
#[derive(Debug, Clone)]
pub struct TrackInfo {
    /// The artist name
    pub artist: String,
    /// The track title
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

/// Parses track information from an ICY stream title.
///
/// ICY metadata typically comes in the format "Artist - Title".
/// This function attempts to parse that format, falling back to
/// using the entire string as the title if no artist is found.
///
/// # Arguments
///
/// * `stream_title` - The raw ICY stream title metadata
///
/// # Returns
///
/// A `TrackInfo` struct with parsed artist and title information.
/// If parsing fails, artist defaults to "Unknown".
///
/// # Examples
///
/// ```rust
/// use soma_player::models::parse_track_info;
///
/// let track = parse_track_info("Radiohead - Paranoid Android");
/// assert_eq!(track.artist, "Radiohead");
/// assert_eq!(track.title, "Paranoid Android");
///
/// let track = parse_track_info("Just a title");
/// assert_eq!(track.artist, "Unknown");
/// assert_eq!(track.title, "Just a title");
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_info_default() {
        let track = TrackInfo::default();
        assert_eq!(track.artist, "Unknown");
        assert_eq!(track.title, "Loading...");
    }

    #[test]
    fn test_parse_track_info_with_artist_and_title() {
        let stream_title = "Radiohead - Paranoid Android";
        let track = parse_track_info(stream_title);
        
        assert_eq!(track.artist, "Radiohead");
        assert_eq!(track.title, "Paranoid Android");
    }

    #[test]
    fn test_parse_track_info_with_multiple_dashes() {
        let stream_title = "Nine Inch Nails - The Hand That Feeds - Remix";
        let track = parse_track_info(stream_title);
        
        assert_eq!(track.artist, "Nine Inch Nails");
        assert_eq!(track.title, "The Hand That Feeds - Remix");
    }

    #[test]
    fn test_parse_track_info_no_artist() {
        let stream_title = "Just a Song Title";
        let track = parse_track_info(stream_title);
        
        assert_eq!(track.artist, "Unknown");
        assert_eq!(track.title, "Just a Song Title");
    }

    #[test]
    fn test_parse_track_info_empty_parts() {
        let stream_title = " - ";
        let track = parse_track_info(stream_title);
        
        assert_eq!(track.artist, "Unknown");
        assert_eq!(track.title, " - ");
    }

    #[test]
    fn test_parse_track_info_with_whitespace() {
        let stream_title = "  Artist Name  -  Song Title  ";
        let track = parse_track_info(stream_title);
        
        assert_eq!(track.artist, "Artist Name");
        assert_eq!(track.title, "Song Title");
    }

    #[test]
    fn test_parse_track_info_empty_string() {
        let stream_title = "";
        let track = parse_track_info(stream_title);
        
        assert_eq!(track.artist, "Unknown");
        assert_eq!(track.title, "");
    }

    #[test]
    fn test_parse_track_info_special_characters() {
        let stream_title = "Björk - Jóga";
        let track = parse_track_info(stream_title);
        
        assert_eq!(track.artist, "Björk");
        assert_eq!(track.title, "Jóga");
    }
}
