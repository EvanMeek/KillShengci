use std::{str::FromStr, string::ParseError};

use rusqlite::types::FromSql;
use select::{
    document::Document,
    predicate::{Attr, Class, Name},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Word {
    pub keyword: Option<String>,
    // 音节划分
    pub tips: Option<String>,
    // 词汇等级
    pub level: Option<String>,
    // 词汇音标
    pub phonetic: Option<(String, String)>,
    // 词汇解释
    pub explains: Vec<(String, String)>,
    // 词汇来源
    pub etymons: Option<String>,
    // 词汇分布统计
    pub distribution_data: Vec<(i64, String)>,
    // 熟练程度
    pub familiarity: Familiarity,
}

impl Word {
    pub fn new(eng: String) -> Word {
        let document = Document::from(eng.as_str());

        let mut keyword: Option<String> = Some("".to_string());
        let mut tips: Option<String> = Some("".to_string());
        let mut level: Option<String> = Some("".to_string());
        let mut phonetic: Option<(String, String)> = Some(("".to_string(), "".to_string()));
        let mut explains: Vec<(String, String)> = vec![];
        let mut etymons: Option<String> = Some("".to_string());
        let mut distribution_data: Vec<(i64, String)> = vec![];
        // 词续
        for node in document.find(Class("word-cont")) {
            let keyword_node = node.find(Class("keyword")).next().unwrap();
            // 关键词文本
            keyword = Some(keyword_node.text());
            // 音节划分
            if let Some(keyword_tips) = keyword_node.attr("tip") {
                tips = Some(keyword_tips.to_string());
            }
            // 单词等级
            if let Some(level_title_node) = node.find(Class("level-title")).next() {
                if let Some(level_title_text) = level_title_node.attr("level") {
                    level = Some(level_title_text.to_string());
                }
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
                for d in json_data.as_object().unwrap().values() {
                    distribution_data
                        .push((d["percent"].as_i64().unwrap(), d["sense"].to_string()));
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
            familiarity: Familiarity::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
pub enum Familiarity {
    NewWord,
    Familiarity,
    Memorized,
}

impl Default for Familiarity {
    fn default() -> Self {
        Self::NewWord
    }
}
impl ToString for Familiarity {
    fn to_string(&self) -> String {
        match self {
            Self::NewWord => String::from("生词"),
            Self::Familiarity => String::from("熟练"),
            Self::Memorized => String::from("记住"),
        }
    }
}
impl FromStr for Familiarity {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "生词" => Ok(Self::NewWord),
            "熟练" => Ok(Self::Familiarity),
            "记住" => Ok(Self::Memorized),
            _ => panic!("fuck"),
        }
    }
}

impl FromSql for Familiarity {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(v) => match std::str::from_utf8(v).unwrap() {
                "生词" => Ok(Self::NewWord),
                "熟练" => Ok(Self::Familiarity),
                "记住" => Ok(Self::Memorized),
                _ => panic!("fuck"),
            },
            _ => panic!("fuck"),
        }
    }
}
