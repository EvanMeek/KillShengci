use eframe::{egui::Vec2, NativeOptions};
use kill_shengci::{app::App, new_ui::NewUI};

const WINDOW_HEIGHT: f32 = 600.;
const WINDOW_WIDTH: f32 = 800.;
fn main() {
    run_app();
}

fn run_app() {
    let app = App::default();
    app.db.init_db();
    let mut native_options = NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT));
    eframe::run_native(Box::new(app), native_options);
}

fn run_newui() {
    let new_ui = NewUI::default();
    let mut native_options = NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT));
    eframe::run_native(Box::new(new_ui), native_options);
}
