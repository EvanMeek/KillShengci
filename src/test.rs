use crate::{db_manage::DBManage, dict_manage::Dict, dictcn, word::Word};

#[test]
fn test_connect_db() {
    let db = DBManage::new();
    db.init_table();
    let word = Word::new(dictcn::get_raw_html("fuck").unwrap());
    db.add_word(&word).unwrap();
}
#[test]
fn create_word() {
    let word = Word::new(dictcn::get_raw_html("hello").unwrap());
    println!(
        "Serializated word: {}",
        serde_json::to_string_pretty(&word).unwrap()
    );
}
#[test]
fn add_word() {
    let word = Word::new(dictcn::get_raw_html("hello").unwrap());
    let db = DBManage::new();
    db.add_word(&word).unwrap();
}
#[test]
fn find_word() {
    let db = DBManage::new();
    println!(
        "{:#?}",
        db.find_word_by_keyword(&"hello".to_string()).unwrap()
    );
}
#[test]
fn delete_word() {
    let db = DBManage::new();
    db.delete_word_by_keyword(&"fuck".to_string()).unwrap()
}
