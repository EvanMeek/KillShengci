use eframe::{
    egui::{
        self, Button, CentralPanel, Color32, CtxRef, FontDefinitions, FontFamily, ScrollArea,
        SidePanel, TextEdit, TextStyle, TopBottomPanel,
    },
    epi::{self, Frame, Storage},
};

use crate::{
    db_manage::DBManage,
    dictcn,
    word::{Word, Familiarity},
};

const _PADDING: f32 = 5.;
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
#[derive(Default)]
pub struct App {
    pub db: DBManage,
    setting: bool,
    capture_word: String,
    msg: String,
}
impl App {
    fn new() -> Self {
        Self::default()
    }
    fn handle_delete_dict(&mut self, familiarity: &Familiarity) {
        match self.db.delete_word_by_familiarity(familiarity) {
            Ok(_) => self.msg = format!("删除{}词库成功!", familiarity.to_string()),
            Err(_) => self.msg = format!("删除{}词库失败!", familiarity.to_string()),
        }
    }

    fn handle_import_dict(&self, ctx: &eframe::egui::CtxRef) {
        todo!()
    }

    fn handle_capture_word(&mut self) {
        match self.db.add_word(&Word::new(
            dictcn::get_raw_html(&self.capture_word).unwrap(),
        )) {
            Ok(_) => self.msg = String::from("成功添加新单词!"),
            Err(_) => self.msg = String::from("添加新单词失败!"),
        };
    }
    // 渲染词库
    fn render_dict(
        &mut self,
        heading: &str,
        familiarity: &Familiarity,
        other_familiarity: &Familiarity,
        ui: &mut egui::Ui,
    ) {
        ui.collapsing(heading, |ui| {
            for word in self.db.get_words(familiarity).unwrap() {
                ui.horizontal(|ui| {
                    egui::CollapsingHeader::new(word.keyword.as_ref().unwrap()).show(ui, |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("删除").on_hover_text("永久删除此单词").clicked()
                            {
                                self.handle_delete_word(&word.keyword.as_ref().unwrap());
                            }
                            if ui
                                .button(&other_familiarity.to_string())
                                .on_hover_text("移动到指定词库")
                                .clicked()
                            {
                                self.handle_change_word_familiarity(
                                    &word.keyword.as_ref().unwrap(),
                                    other_familiarity,
                                );
                            }
                        });
                        ui.collapsing("解释", |ui| {
                            for explain in &word.explains {
                                ui.add(
                                    egui::Label::new(&explain.0)
                                        .text_style(egui::TextStyle::Button),
                                );
                                ui.add(
                                    egui::Label::new(&explain.1)
                                        .text_style(egui::TextStyle::Button),
                                );
                            }
                        });
                        ui.collapsing("音节", |ui| {
                            ui.add(
                                egui::Label::new(&word.tips.as_ref().unwrap())
                                    .text_style(egui::TextStyle::Button),
                            );
                        });
                        ui.collapsing("音标", |ui| {
                            ui.horizontal(|ui| {
                                ui.add(egui::Label::new("英").text_style(egui::TextStyle::Button));
                                ui.add(
                                    egui::Label::new(&word.phonetic.as_ref().unwrap().0)
                                        .text_style(egui::TextStyle::Button),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.add(egui::Label::new("美").text_style(egui::TextStyle::Button));
                                ui.add(
                                    egui::Label::new(&word.phonetic.as_ref().unwrap().1)
                                        .text_style(egui::TextStyle::Button),
                                );
                            });
                        });
                        ui.collapsing("起源", |ui| {
                            ui.add(
                                egui::Label::new(&word.etymons.as_ref().unwrap())
                                    .text_style(egui::TextStyle::Button),
                            );
                        });
                        ui.collapsing("用词分布", |ui| {});
                    });
                });
            }
        });
    }
    fn configuration_fonts(&self, ctx: &CtxRef) {
        let my_font = include_bytes!("../SourceHanSansCN-Medium.otf");

        let mut fonts = FontDefinitions::default();
        // Create FontDefinitions object.
        fonts
            .font_data
            .insert("my_font".to_owned(), std::borrow::Cow::Borrowed(my_font));
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

    fn handle_delete_word(&mut self, keyword: &str) {
        match self.db.delete_word_by_keyword(&keyword.to_string()) {
            Ok(_) => self.msg = String::from("删除单词成功!"),
            Err(_) => self.msg = String::from("删除单词失败!"),
        }
    }

    fn handle_change_word_familiarity(&mut self, keyword: &str, familiarity: &Familiarity) {
        match self.db.change_word_familiarity(keyword, familiarity) {
            Ok(_) => self.msg = String::from("移动单词词库成功!"),
            Err(_) => self.msg = String::from("移动单词词库失败!"),
        }
    }
}
impl epi::App for App {
    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut Frame<'_>,
        _storage: Option<&dyn Storage>,
    ) {
        self.configuration_fonts(ctx);
    }
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        if self.setting {
            SidePanel::left("")
                .resizable(false)
                .default_width(50.)
                .width_range(50.0..=70.)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.collapsing("管理词库", |ui| {
                            ui.collapsing("删除词库", |ui| {
                                if ui.button("生词").clicked() {
                                    self.handle_delete_dict(&Familiarity::NewWord);
                                };
                                if ui.button("熟练").clicked() {
                                    self.handle_delete_dict(&Familiarity::Familiarity);
                                };
                                if ui.button("记住").clicked() {
                                    self.handle_delete_dict(&Familiarity::Memorized);
                                };
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

        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(&self.msg);
                    ui.separator();
                });
                self.render_dict(
                    &"生词".to_string(),
                    &Familiarity::NewWord,
                    &Familiarity::Familiarity,
                    ui,
                );
                self.render_dict(
                    &"熟悉".to_string(),
                    &Familiarity::Familiarity,
                    &Familiarity::Memorized,
                    ui,
                );
                self.render_dict(
                    &"记住".to_string(),
                    &Familiarity::Memorized,
                    &Familiarity::Familiarity,
                    ui,
                );
            });
        });
        TopBottomPanel::bottom("my_bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.setting, "设置");
                let _text = ui.add(
                    TextEdit::singleline(&mut self.capture_word)
                        .hint_text("Input a word to capture.")
                        .desired_width(100.)
                        .text_style(TextStyle::Button),
                );
                if ui.add(Button::new("Capture!").text_color(CYAN)).clicked() {
                    self.handle_capture_word();
                    self.capture_word = String::new();
                }
            });
        });
    }

    fn name(&self) -> &str {
        "Kill ShengCi"
    }
}
