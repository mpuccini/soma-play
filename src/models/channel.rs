use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub title: String,
    pub description: String,
    pub playlists: Vec<Playlist>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub url: String,
    pub format: String,
    pub quality: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SomaFmResponse {
    pub channels: Vec<Channel>,
}
