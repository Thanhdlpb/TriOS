use reqwest::blocking::Client;
use serde_json::Value;

pub struct WebAgent { client: Client }

impl WebAgent {
    pub fn new() -> Self { Self { client: Client::new() } }
    
    pub fn fetch_json(&self, url: &str) -> Result<String, String> {
        self.client.get(url).send()
            .map_err(|e| format!("HTTP: {}", e))?
            .text().map_err(|e| format!("Đọc: {}", e))
    }
    
    pub fn search(&self, query: &str) -> Result<String, String> {
        let url = format!("https://api.duckduckgo.com/?q={}&format=json&no_html=1", 
            urlencoding::encode(query));
        let json_str = self.fetch_json(&url)?;
        let json: Value = serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
        Ok(json["AbstractText"].as_str().unwrap_or("Không tìm thấy.").to_string())
    }
}
