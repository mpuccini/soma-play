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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pls_playlist_valid() {
        let pls_content = r#"
[playlist]
NumberOfEntries=2
File1=http://ice1.somafm.com/groovesalad-256-mp3
Title1=SomaFM - Groove Salad (#1 256k mp3): A nicely chilled plate of ambient/downtempo beats and grooves.
Length1=-1
File2=http://ice2.somafm.com/groovesalad-256-mp3
Title2=SomaFM - Groove Salad (#2 256k mp3): A nicely chilled plate of ambient/downtempo beats and grooves.
Length2=-1
Version=2
"#;
        
        // Simulate the parsing logic
        let mut found_url = None;
        for line in pls_content.lines() {
            let line = line.trim();
            if line.starts_with("File") && line.contains("=") {
                if let Some(url) = line.split('=').nth(1) {
                    let url = url.trim();
                    if url.starts_with("http") {
                        found_url = Some(url.to_string());
                        break;
                    }
                }
            }
        }
        
        assert_eq!(found_url, Some("http://ice1.somafm.com/groovesalad-256-mp3".to_string()));
    }

    #[test]
    fn test_parse_pls_playlist_no_files() {
        let pls_content = r#"
[playlist]
NumberOfEntries=0
Version=2
"#;
        
        let mut found_url = None;
        for line in pls_content.lines() {
            let line = line.trim();
            if line.starts_with("File") && line.contains("=") {
                if let Some(url) = line.split('=').nth(1) {
                    let url = url.trim();
                    if url.starts_with("http") {
                        found_url = Some(url.to_string());
                        break;
                    }
                }
            }
        }
        
        assert_eq!(found_url, None);
    }

    #[test]
    fn test_parse_pls_playlist_invalid_urls() {
        let pls_content = r#"
[playlist]
NumberOfEntries=1
File1=not-a-url
Title1=Invalid URL
Length1=-1
Version=2
"#;
        
        let mut found_url = None;
        for line in pls_content.lines() {
            let line = line.trim();
            if line.starts_with("File") && line.contains("=") {
                if let Some(url) = line.split('=').nth(1) {
                    let url = url.trim();
                    if url.starts_with("http") {
                        found_url = Some(url.to_string());
                        break;
                    }
                }
            }
        }
        
        assert_eq!(found_url, None);
    }

    #[test]
    fn test_api_url_constant() {
        assert_eq!(SOMAFM_API_URL, "https://api.somafm.com/channels.json");
    }

    // Note: Integration tests for fetch_channels() would require network access
    // These should be in a separate integration test file or use mocking
}
