use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use eframe::egui::Vec2;
use kill_shengci::{
    dict_manage::{self, Dict},
    dictcn,
    shengci_app::ShengCiApp,
    word::Word,
};
use serde::{Deserialize, Serialize};

fn main() {
    // demo3();
    // return;
    let app = ShengCiApp::new();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(300., 500.));
    eframe::run_native(Box::new(app), native_options);
}
fn demo() {
    #[derive(Debug, Deserialize, Serialize)]
    struct Student {
        name: String,
        age: i32,
    }
    let mut f = File::open("/home/evanmeek/fuck.json").unwrap();
    let mut buffer: String = String::new();
    f.read_to_string(&mut buffer).unwrap();
    println!("buffer: {}", buffer);
    let mut stus: Vec<Student> = vec![];
    let stujs = serde_json::from_str::<Vec<Student>>(&buffer).unwrap();
    stus = stujs;
    stus.push(Student {
        name: "Fuck".to_string(),
        age: 20,
    });
    println!("{:#?}", stus);
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(false)
        .open("/home/evanmeek/fuck.json")
        .unwrap();
    let mut buffer: String = String::new();
    buffer = serde_json::to_string_pretty(&stus).unwrap();
    f.write(buffer.as_bytes()).unwrap();
}
#[derive(Debug, Deserialize, Serialize)]
struct Student<'a> {
    name: &'a str,
    age: i32,
    sex: bool,
}
fn demo2() {
    let mut stus: Vec<Student> = vec![];
    stus.push(Student {
        name: "张三",
        age: 10,
        sex: true,
    });
    stus.push(Student {
        name: "李四",
        age: 11,
        sex: false,
    });
    stus.push(Student {
        name: "王五",
        age: 12,
        sex: true,
    });
    io(&stus);
    stus.remove(1);
    io(&stus);
}
fn io(stus: &Vec<Student>) {
    if let Ok(mut f) = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("/home/evanmeek/.config/shengci/test.json")
    {
        let mut buffer = serde_json::to_string_pretty(&stus).unwrap();
        f.write(&mut buffer.as_bytes());
        let mut buffer = String::new();
        f.read_to_string(&mut buffer);
        println!("test.json\n{}", buffer);
    }
}

fn demo3() {
    let w = Word::new(dictcn::get_raw_html("hello").unwrap());
    let mut dict = Dict::new(dict_manage::Familiarity::NewWord).unwrap();
    dict.add_word(w).unwrap();
}
