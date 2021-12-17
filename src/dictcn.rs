const API_URL: &str = "http://dict.cn/";
pub async fn get_raw_html(word: &str) -> Result<String, reqwest::Error> {
    let url = format!("{}{}", API_URL, word);
    if let Ok(url) = reqwest::Url::parse(&url) {
        let raw_html = reqwest::Client::new()
            .get(url)
            .header("User-Agent", "Rust")
            .send()
            .await?
            .text()
            .await?;
        Ok(raw_html)
    } else {
        Ok("".to_string())
    }
}
