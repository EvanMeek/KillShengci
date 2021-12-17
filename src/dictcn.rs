const API_URL: &str = "http://dict.cn/";
pub fn get_raw_html(word: &str) -> Result<String, reqwest::Error> {
    let url = format!("{}{}", API_URL, word);
    if let Ok(url) = reqwest::Url::parse(&url) {
        let raw_html = reqwest::blocking::get(url)?.text()?;
        Ok(raw_html)
    } else {
        Ok("".to_string())
    }
}
