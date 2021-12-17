use select::{
    document::Document,
    predicate::{Attr, Class, Name},
};
use serde_json::Value;
#[derive(Debug)]
pub struct Word {
    keyword: Option<String>,
    // 音节划分
    tips: Option<String>,
    // 词汇等级
    level: Option<String>,
    // 词汇音标
    phonetic: Option<(String, String)>,
    // 词汇解释
    explains: Vec<(String, String)>,
    // 词汇来源
    etymons: Option<String>,
    // 词汇分布统计
    distribution_data: Vec<(u8, Vec<(i64, String)>)>,
}
impl Word {
    pub fn new(eng: String) -> Word {
        let document = Document::from(eng.as_str());

        let mut keyword: Option<String> = None;
        let mut tips: Option<String> = None;
        let mut level: Option<String> = None;
        let mut phonetic: Option<(String, String)> = None;
        let mut explains: Vec<(String, String)> = vec![];
        let mut etymons: Option<String> = None;
        let mut distribution_data: Vec<(u8, Vec<(i64, String)>)> = vec![];
        // 词续
        for node in document.find(Class("word-cont")) {
            let keyword_node = node.find(Class("keyword")).next().unwrap();
            // 关键词文本
            keyword = Some(keyword_node.text());
            // 音节划分
            if let Some(keyword_tips) = keyword_node.attr("tip") {
                tips = Some(keyword_tips.to_string());
            }
            let level_title_node = node.find(Class("level-title")).next().unwrap();
            // 单词等级
            if let Some(level_title_text) = level_title_node.attr("level") {
                level = Some(level_title_text.to_string());
            }
        }
        // 词汇音标
        for node in document.find(Class("phonetic")) {
            if let Some(phonetic_node) = node.find(Attr("lang", "EN-US")).next() {
                let en_uk = phonetic_node.text();
                let en_us = phonetic_node.text();
                phonetic = Some((en_uk, en_us));
            }
        }
        // 单词解释
        for node in document.find(Class("dict-basic-ul")) {
            for explain in node.find(Name("li")) {
                // 词性
                // 中文解释
                if let (Some(word_type), Some(zh)) = (
                    explain.find(Name("span")).next(),
                    explain.find(Name("strong")).next(),
                ) {
                    // println!("{:#?}", explains.as_mut());
                    explains.push((word_type.text(), zh.text()));
                }
            }
        }
        // 起源
        if let Some(w_etymons) = document.find(Attr("class", "layout etm")).next() {
            if let Some(w_etymons) = w_etymons.find(Name("li")).next() {
                etymons = Some(w_etymons.text());
            }
        }
        // 解释用途统计
        if let Some(distribution_chart) = document.find(Attr("id", "dict-chart-basic")).next() {
            if let Some(w_distribution_data) = distribution_chart.attr("data") {
                let json_data = serde_json::from_str::<Value>(
                    &urlencoding::decode(w_distribution_data).unwrap(),
                )
                .unwrap();
                for (i, d) in json_data.as_object().unwrap().values().enumerate() {
                    let mut data = vec![];
                    data.push((d["percent"].as_i64().unwrap(), d["sense"].to_string()));
                    distribution_data.push((i as u8, data));
                }
            }
        };
        Word {
            keyword,
            tips,
            level,
            phonetic,
            explains,
            etymons,
            distribution_data,
        }
    }
}
