use eframe::{
    egui::{
        self, color_picker::show_color, Button, CentralPanel, Color32, CtxRef, FontData,
        FontDefinitions, FontFamily, RichText, ScrollArea, SidePanel, TextEdit, TextStyle,
        TopBottomPanel,
    },
    epi::App,
};

use crate::word::Familiarity;
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
#[derive(Default)]
pub struct NewUI {
    setting: bool,
    capture_word: String,
    radio_familiarity: Familiarity,
}
impl NewUI {
    fn configuration_fonts(&self, ctx: &CtxRef) {
        let my_font = include_bytes!("../SourceHanSansCN-Medium.otf");

        let mut fonts = FontDefinitions::default();
        // Create FontDefinitions object.
        fonts
            .font_data
            .insert("my_font".to_owned(), FontData::from_static(my_font));
        // 设定iosevka字体为最优选字体
        fonts
            .fonts_for_family
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "my_font".to_owned());

        // 设定不同样式的字体大小
        fonts
            .family_and_size
            .insert(egui::TextStyle::Button, (FontFamily::Monospace, 25.0));
        fonts
            .family_and_size
            .insert(egui::TextStyle::Heading, (FontFamily::Monospace, 30.0));
        fonts
            .family_and_size
            .insert(egui::TextStyle::Body, (FontFamily::Monospace, 25.0));
        ctx.set_fonts(fonts);
    }
}
impl App for NewUI {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        if self.setting {
            SidePanel::left("")
                .resizable(false)
                .default_width(50.)
                .width_range(50.0..=70.)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.collapsing("管理词库", |ui| {
                            ui.collapsing("删除词库", |ui| {
                                if ui.button("生词").clicked() {};
                                if ui.button("熟练").clicked() {};
                                if ui.button("记住").clicked() {};
                            });

                            if ui.button("导入单词").clicked() {
                                // self.handle_import_dict(ctx);
                            }
                        });
                        ui.separator();
                        ui.collapsing("ShengCi setting", |_ui| {});
                    });
                });
        }
        CentralPanel::default().show(ctx, |ui| ScrollArea::new([false, true]).show(ui, |ui| {}));
        TopBottomPanel::bottom("my_bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.setting, "设置");
                ui.separator();
                let _text = ui.add(
                    TextEdit::singleline(&mut self.capture_word)
                        .hint_text("请输入一个单词以捕获")
                        .desired_width(250.)
                        .text_style(TextStyle::Button),
                );
                if ui
                    .add(Button::new(RichText::new("添加到生词表！").color(CYAN)))
                    .clicked()
                {
                    self.capture_word = String::new();
                }
                ui.separator();

                ui.horizontal(|ui| {
                    ui.radio_value(
                        &mut self.radio_familiarity,
                        Familiarity::NewWord,
                        "显示生词",
                    );
                    ui.radio_value(
                        &mut self.radio_familiarity,
                        Familiarity::Familiarity,
                        "显示熟词",
                    );
                    ui.radio_value(
                        &mut self.radio_familiarity,
                        Familiarity::Memorized,
                        "显示记住",
                    );
                });
            });
        });
    }

    fn name(&self) -> &str {
        "New UI Ddemo"
    }

    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configuration_fonts(ctx);
    }
}
