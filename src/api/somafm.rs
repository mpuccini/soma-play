use crate::models::{Channel, SomaFmResponse};

const SOMAFM_API_URL: &str = "https://api.somafm.com/channels.json";

/// Fetches the list of SomaFM channels from the API.
pub async fn fetch_channels() -> Result<Vec<Channel>, Box<dyn std::error::Error>> {
    let response = reqwest::get(SOMAFM_API_URL).await?.json::<SomaFmResponse>().await?;
    Ok(response.channels)
}

/// Parses a .pls playlist file and returns the first stream URL
pub async fn parse_pls_playlist(pls_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get(pls_url).send().await?;
    let pls_content = response.text().await?;
    
    // Parse the .pls file to find File1, File2, etc.
    for line in pls_content.lines() {
        let line = line.trim();
        if line.starts_with("File") && line.contains("=") {
            if let Some(url) = line.split('=').nth(1) {
                let url = url.trim();
                if url.starts_with("http") {
                    return Ok(url.to_string());
                }
            }
        }
    }
    
    Err("No valid stream URL found in .pls playlist".into())
}
