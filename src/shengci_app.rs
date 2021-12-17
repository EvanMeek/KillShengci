use eframe::{
    egui::{self, Color32, FontDefinitions, FontFamily},
    epi,
};
const PADDING: f32 = 5.;
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
#[derive(Default)]
pub struct ShengCiApp {
    capture_word: String,
}
impl ShengCiApp {
    fn new(self) -> ShengCiApp {
        ShengCiApp {
            capture_word: Default::default(),
        }
    }
    fn configura_font(&self, ctx: &egui::CtxRef) {
        // Load font file with bytes reader.
        let my_font = include_bytes!("../SourceHanMonoSC-Normal.otf");
        // Create FontDefinitions object. That is
        let mut fonts = FontDefinitions::default();
        fonts
            .font_data
            .insert("my_font".to_owned(), std::borrow::Cow::Borrowed(my_font));
        // 设定iosevka字体为最优选字体
        fonts
            .fonts_for_family
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "my_font".to_owned());
        fonts
            .fonts_for_family
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "my_font".to_owned());
        // 设定Headling
        fonts
            .family_and_size
            .insert(egui::TextStyle::Heading, (FontFamily::Proportional, 30.0));
        fonts
            .family_and_size
            .insert(egui::TextStyle::Heading, (FontFamily::Monospace, 30.0));
        fonts
            .family_and_size
            .insert(egui::TextStyle::Body, (FontFamily::Proportional, 25.0));
        fonts
            .family_and_size
            .insert(egui::TextStyle::Body, (FontFamily::Monospace, 25.0));
        ctx.set_fonts(fonts);
    }
    fn handle_capture_word(&self, ctx: &egui::CtxRef) {

    }

    fn handle_delete_dict(&self, ctx: &egui::CtxRef) {
        todo!()
    }

    fn handle_import_dict(&self, ctx: &egui::CtxRef) {
        todo!()
    }
}
impl epi::App for ShengCiApp {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::SidePanel::left("")
            .resizable(false)
            .default_width(100.)
            .width_range(80.0..=150.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Kill ShengCi");
                    // ui.add(egui::Separator::default());
                    ui.add_space(PADDING * 5.);
                    ui.separator();
                    ui.collapsing("Manage dict.", |ui| {
                        if ui.button("Delete dict.").clicked() {
                            self.handle_delete_dict(ctx);
                        };
                        if ui.button("Import dict.").clicked() {
                            self.handle_import_dict(ctx);
                        }
                    });
                    ui.separator();
                    ui.collapsing("ShengCi setting.", |ui| {});
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Dict List");
                ui.separator();
            });
            ui.collapsing("生词", |ui| {});
            ui.collapsing("已背诵", |ui| {});
            ui.collapsing("已复习", |ui| {});
            ui.collapsing("滚瓜烂熟", |ui| {});
        });
        egui::TopBottomPanel::bottom("my_bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.capture_word)
                        .hint_text("Input a word to capture.")
                        .desired_width(500.),
                );
                if ui
                    .add(egui::Button::new("Capture!").text_color(CYAN))
                    .clicked()
                {
                    self.handle_capture_word(ctx);
                }
            });
        });
    }

    fn name(&self) -> &str {
        "消灭生词"
    }

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        self.configura_font(ctx);
    }
}
