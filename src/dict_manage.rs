use core::panic;
use std::{
    default,
    env::{self},
    ffi::{OsStr, OsString},
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    str::FromStr,
    string::ParseError,
};

use rusqlite::types::FromSql;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::word::Word;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Dict {
    pub familiarity: Familiarity,
    path: OsString,
    pub words: Vec<Word>,
}

impl Dict {
    pub fn new(familiarity: Familiarity) -> Result<Dict, io::Error> {
        let path = match familiarity {
            Familiarity::NewWord => env::var("SHENGCI_NEW_WORD_DICT"),
            Familiarity::Familiarity => env::var("SHENGCI_FAMILIARITY_DICT"),
            Familiarity::Memorized => env::var("SHENGCI_MEMORIZED_DICT"),
        };
        Ok(Dict {
            familiarity,
            words: Dict::load_dict(&path.as_ref().unwrap().into())?,
            path: path.unwrap().into(),
        })
    }
    // 加载指定词典
    fn load_dict(path: &OsString) -> Result<Vec<Word>, io::Error> {
        let mut f = File::open(path)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        if let Ok(dict) = serde_json::from_str::<Vec<Word>>(&buffer) {
            Ok(dict)
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Opps. Your dict is not stand format",
            ))
        }
    }
    // 刷新自身数据
    pub fn flush(&mut self) -> Result<(), io::Error> {
        self.words = Dict::load_dict(&self.path)?;
        Ok(())
    }
    // 添加一个新的单词
    pub fn add_word(&mut self, word: Word) -> Result<bool, io::Error> {
        if !self.is_contain(word.keyword.as_ref().unwrap())? {
            self.words.push(word);
            self.write()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    // 将words写入到文件中
    fn write(&mut self) -> Result<bool, io::Error> {
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.path)?;
        let mut buffer: String = String::new();
        buffer = serde_json::to_string(&self.words)?;
        // 将新内容写入文件中
        f.write(buffer.as_bytes())?;
        self.flush()?;
        Ok(true)
    }
    // 判断某个单词是否已存在
    fn is_contain(&self, keyword: &String) -> Result<bool, io::Error> {
        for word in &self.words {
            if keyword.eq_ignore_ascii_case(word.keyword.as_ref().unwrap()) {
                return Ok(true);
            }
        }
        Ok(false)
    }
    // 删除单词
    pub fn delete_word(&mut self, keyword: &String) -> Result<bool, io::Error> {
        for (i, word) in self.words.clone().iter().enumerate() {
            if keyword.eq_ignore_ascii_case(word.keyword.as_ref().unwrap()) {
                self.words.remove(i);
                self.write()?;
                return Ok(true);
            }
        }
        Ok(false)
    }
    // 查找单词
    pub fn find_word_by_keyword(&self, keyword: &String) -> Option<Word> {
        for word in &self.words {
            if keyword.eq_ignore_ascii_case(word.keyword.as_ref().unwrap()) {
                return Some(word.clone());
            }
        }
        None
    }
    // 移动单词到其他词库中
    pub fn move_word_to_dict(
        &mut self,
        keyword: &String,
        dict: &mut Dict,
    ) -> Result<bool, io::Error> {
        if let Some(word) = self.find_word_by_keyword(keyword) {
            dict.add_word(word)?;
            self.delete_word(keyword)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
#[derive(Debug, Deserialize, Clone, Serialize)]
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
            Self::NewWord => String::from("new_word"),
            Self::Familiarity => String::from("familiarity"),
            Self::Memorized => String::from("memorized"),
        }
    }
}
impl FromStr for Familiarity {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "new_word" => Ok(Self::NewWord),
            "familiarity" => Ok(Self::Familiarity),
            "memorized" => Ok(Self::Memorized),
            _ => panic!("fuck"),
        }
    }
}

impl FromSql for Familiarity {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(v) => match std::str::from_utf8(v).unwrap() {
                "new_word" => Ok(Self::NewWord),
                "familiarity" => Ok(Self::Familiarity),
                "memorized" => Ok(Self::Memorized),
                _ => panic!("fuck"),
            },
            _ => panic!("fuck"),
        }
    }
}
