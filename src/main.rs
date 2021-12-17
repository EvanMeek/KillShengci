use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use eframe::egui::Vec2;
use egui_demo::shengci_app::ShengCiApp;
use serde::{Deserialize, Serialize};

fn main() {
    // demo();
    // return;
    let app = ShengCiApp::new();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(700., 500.));
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
