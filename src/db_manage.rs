use std::sync::Arc;

use rusqlite::{params, types::FromSql, Connection, Rows};
use serde::__private::de::IdentifierDeserializer;

use crate::{
    word::{Word, Familiarity},
};
pub struct DBManage {
    connection: Connection,
}

impl Default for DBManage {
    fn default() -> Self {
        Self::new()
    }
}
impl DBManage {
    pub fn new() -> Self {
        Self {
            connection: Connection::open("dict.db").expect("创建数据库出错"),
        }
    }
    pub fn get_words(&self, familiarity: &Familiarity) -> Result<Vec<Word>, rusqlite::Error> {
        let mut stmt = self
            .connection
            .prepare(r#"SELECT keyword FROM word WHERE familiarity=:familiarity;"#)?;
        let words = stmt
            .query_map::<Word, _, _>(&[(":familiarity", &familiarity.to_string())], |row| {
                self.find_word_by_keyword(&row.get::<_, String>(0).unwrap())
            })?
            .collect::<Result<Vec<Word>, _>>()?;
        Ok(words)
    }
    pub fn init_db(&self) {
        if let Ok(_result) = self.connection.execute(
            r#"SELECT * FROM sqlite_master where type='table' and name='word'"#,
            [],
        ) {
            match self.connection.execute(
                r#"
                CREATE TABLE "word"(
                    "keyword" TEXT,
                    "tips" TEXT,
                    "level" TEXT,
                    "phonetic_us" TEXT,
                    "phonetic_uk" TEXT,
                    "etymons" TEXT,
                    "familiarity" TEXT,
                    PRIMARY KEY("keyword")
                );
                "#,
                [],
            ) {
                Ok(result) => println!("result: {}", result),
                Err(e) => println!("execute sql failed: {}", e),
            }
        }
        if let Ok(_result) = self.connection.execute(
            r#"SELECT * FROM sqlite_master where type='table' and name='explains'"#,
            [],
        ) {
            match self.connection.execute(
                r#"
                CREATE TABLE "explains" (
                    "keyword"   TEXT NOT NULL,
                    "part_of_speech"    TEXT NOT NULL,
                    "explain"   TEXT NOT NULL,
                    FOREIGN KEY("keyword") REFERENCES "word"("keyword")
                );
                "#,
                [],
            ) {
                Ok(result) => println!("result: {}", result),
                Err(e) => println!("execute sql failed: {}", e),
            }
        }
        if let Ok(_result) = self.connection.execute(
            r#"SELECT * FROM sqlite_master where type='table' and name='distribution_data'"#,
            [],
        ) {
            match self.connection.execute(
                r#"
                CREATE TABLE "distribution_data"(
                    "keyword" TEXT NOT NULL,
                    "frequency" INTEGER NOT NULL,
                    "keyword_explain" TEXT NOT NULL
                );
                "#,
                [],
            ) {
                Ok(result) => println!("result: {}", result),
                Err(e) => println!("execute sql failed: {}", e),
            }
        }
    }
    pub fn add_word(&self, word: &Word) -> Result<(), rusqlite::Error> {
        self.connection.execute(
            r#"
            INSERT INTO "word"
            VALUES
            (?1,?2,?3,?4,?5,?6,?7);
            "#,
            [
                word.keyword.as_ref().unwrap(),
                word.tips.as_ref().unwrap(),
                word.level.as_ref().unwrap(),
                &word.phonetic.as_ref().unwrap().0,
                &word.phonetic.as_ref().unwrap().1,
                word.etymons.as_ref().unwrap(),
                &word.familiarity.to_string(),
            ],
        )?;
        for explain in &word.explains {
            self.connection.execute(
                r#"
            INSERT INTO "explains"
            VALUES
            (?1,?2,?3);
            "#,
                [word.keyword.as_ref().unwrap(), &explain.0, &explain.1],
            )?;
        }
        for data in &word.distribution_data {
            self.connection.execute(
                r#"
                INSERT INTO "distribution_data"
                VALUES
                (?1,?2,?3);
                "#,
                [word.keyword.as_ref().unwrap(), &data.0.to_string(), &data.1],
            )?;
        }
        Ok(())
    }
    pub fn find_word_by_keyword(&self, keyword: &String) -> Result<Word, rusqlite::Error> {
        let mut word = Word::default();
        word = self.connection.query_row(
            r#"SELECT keyword,tips,level,phonetic_us,phonetic_uk,etymons,familiarity FROM word WHERE keyword=?1"#,
            [keyword],
            |row| {
                    word.keyword= row.get_unwrap(0);
                    word.tips= row.get_unwrap(1);
                    word.level= row.get_unwrap(2);
                    word.phonetic= Some((row.get_unwrap(3), row.get_unwrap(4)));
                    word.explains= vec![];
                    word.etymons= row.get_unwrap(5);
                    word.distribution_data= vec![];
                word.familiarity= Familiarity::from(row.get_unwrap(6));
                Ok(word)
            },
        ).unwrap();

        let mut stmt = self
            .connection
            .prepare(r#"SELECT part_of_speech, explain FROM explains WHERE keyword=?"#)?;
        let mut explain_rows = stmt.query(params![&word.keyword])?;
        while let Some(row) = explain_rows.next()? {
            word.explains.push((
                row.get_unwrap::<_, String>(0),
                row.get_unwrap::<_, String>(1),
            ));
        }

        let mut stmt = self.connection.prepare(
            r#"SELECT frequency, keyword_explain FROM distribution_data WHERE keyword=?"#,
        )?;
        let mut distribution_data_rows = stmt.query(params![&word.keyword])?;
        while let Some(row) = distribution_data_rows.next()? {
            word.distribution_data
                .push((row.get_unwrap::<_, i64>(0), row.get_unwrap::<_, String>(1)))
        }
        Ok(word)
    }
    pub fn delete_word_by_keyword(&self, keyword: &String) -> Result<(), rusqlite::Error> {
        self.connection.execute(
            r#"
            DELETE FROM word WHERE keyword=?
            "#,
            [&keyword],
        )?;
        self.connection.execute(
            r#"
            DELETE FROM explains WHERE keyword=?
            "#,
            [keyword],
        )?;
        self.connection.execute(
            r#"
            DELETE FROM distribution_data WHERE keyword=?
            "#,
            [keyword],
        )?;
        Ok(())
    }
    pub fn change_word_familiarity(
        &self,
        keyword: &str,
        familiarity: &Familiarity,
    ) -> Result<(), rusqlite::Error> {
        self.connection.execute(
            r#"
            UPDATE word
            SET familiarity = ?1
            WHERE keyword = ?2
            "#,
            [familiarity.to_string(), keyword.to_string()],
        )?;
        Ok(())
    }
    pub fn delete_word_by_familiarity(
        &self,
        familiarity: &Familiarity,
    ) -> Result<(), rusqlite::Error> {
        for word in self.get_words(familiarity)? {
            self.delete_word_by_keyword(&word.keyword.unwrap())?;
        }
        Ok(())
    }
}
