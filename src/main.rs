use eframe::{egui::Vec2, NativeOptions};
use kill_shengci::app::App;

fn main() {
    let app = App::default();
    let mut native_options = NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(500., 700.));
    eframe::run_native(Box::new(app), native_options);
}
