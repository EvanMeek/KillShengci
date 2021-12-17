use eframe::egui::{vec2, Vec2};
use egui_demo::{shengci_app::ShengCiApp, word::Word};

#[tokio::main]
async fn main() {
    // let raw_html = egui_demo::dictcn::get_raw_html("hello").await.unwrap();
    // let w = Word::new(raw_html);
    // egui_demo::word::get_word_info(raw_html);
    // println!("{}", raw_html);
    // println!("{:#?}", w);
    let app = ShengCiApp::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(700., 500.));
    eframe::run_native(Box::new(app), native_options);
}
