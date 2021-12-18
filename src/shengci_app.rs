use eframe::{
    egui::{self, Color32, FontDefinitions, FontFamily, ScrollArea, SidePanel},
    epi,
};

use crate::{
    dict_manage::{Dict, Familiarity},
    dictcn,
    word::{self, Word},
};
const PADDING: f32 = 5.;
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
pub struct ShengCiApp {
    new_word_dict: Dict,
    familiarity_dict: Dict,
    memorized_dict: Dict,
    capture_word: String,
    setting: bool,
    word_expains_fold: bool,
}
impl ShengCiApp {
    pub fn new() -> ShengCiApp {
        ShengCiApp {
            new_word_dict: Dict::new(Familiarity::NewWord).unwrap(),
            capture_word: Default::default(),
            familiarity_dict: Dict::new(Familiarity::Familiarity).unwrap(),
            memorized_dict: Dict::new(Familiarity::MemorizedDict).unwrap(),
            setting: false,
            word_expains_fold: false,
        }
    }
    fn render_plot(
        &self,
        id: &String,
        distribution_data: &Vec<(u8, Vec<(i64, String)>)>,
        ui: &mut egui::Ui,
    ) {
    }
    fn render_dict(&mut self, headling: &String, ui: &mut egui::Ui) {
        ui.collapsing(headling, |ui| {
            for word in &match headling.as_str() {
                "生词" => self.new_word_dict.words.clone(),
                "熟悉" => self.familiarity_dict.words.clone(),
                "记住" => self.memorized_dict.words.clone(),
                _ => panic!("fuck"),
            } {
                ui.horizontal(|ui| {
                    egui::CollapsingHeader::new(word.keyword.as_ref().unwrap()).show(ui, |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("删除").on_hover_text("永久删除此单词").clicked()
                            {
                                self.handle_delete_word(
                                    Familiarity::NewWord,
                                    &word.keyword.as_ref().unwrap(),
                                );
                            }
                            if ui
                                .button(match headling.as_str() {
                                    "生词" => "熟悉",
                                    "熟悉" => "记住",
                                    "记住" => "熟悉",
                                    _ => "",
                                })
                                .on_hover_text("移动到指定词库")
                                .clicked()
                            {
                                match match headling.as_str() {
                                    "生词" => self.new_word_dict.move_word_to_dict(
                                        &word.keyword.as_ref().unwrap(),
                                        &mut self.familiarity_dict,
                                    ),
                                    "熟悉" => self.familiarity_dict.move_word_to_dict(
                                        &word.keyword.as_ref().unwrap(),
                                        &mut self.memorized_dict,
                                    ),
                                    "记住" => self.memorized_dict.move_word_to_dict(
                                        &word.keyword.as_ref().unwrap(),
                                        &mut self.familiarity_dict,
                                    ),
                                    _ => Ok(false),
                                } {
                                    Ok(_) => println!("移动成功"),
                                    Err(e) => println!("移动失败, Err: {}", e),
                                };
                            }
                        });
                        ui.collapsing("解释", |ui| {
                            for explain in &word.explains {
                                ui.label(&explain.0);
                                ui.label(&explain.1);
                            }
                        });
                        ui.collapsing("音节", |ui| {
                            ui.label(&word.tips.as_ref().unwrap());
                        });
                        ui.collapsing("音标", |ui| {
                            ui.horizontal(|ui| {
                                ui.label("英");
                                ui.label(&word.phonetic.as_ref().unwrap().0);
                            });
                            ui.horizontal(|ui| {
                                ui.label("美");
                                ui.label(&word.phonetic.as_ref().unwrap().1);
                            });
                        });
                        ui.collapsing("起源", |ui| {
                            ui.label(&word.etymons.as_ref().unwrap());
                        });
                        ui.collapsing("用词分布", |ui| {
                            self.render_plot(
                                word.keyword.as_ref().unwrap(),
                                &word.distribution_data,
                                ui,
                            );
                        });
                    });
                });
            }
        });
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
            .insert(1, "my_font".to_owned());
        // 设定Headling
        fonts
            .family_and_size
            .insert(egui::TextStyle::Heading, (FontFamily::Proportional, 25.0));
        fonts
            .family_and_size
            .insert(egui::TextStyle::Heading, (FontFamily::Monospace, 25.0));
        fonts
            .family_and_size
            .insert(egui::TextStyle::Body, (FontFamily::Proportional, 20.0));
        fonts
            .family_and_size
            .insert(egui::TextStyle::Body, (FontFamily::Monospace, 20.0));
        ctx.set_fonts(fonts);
    }
    // 将单词加入到生词表中
    fn handle_capture_word(&mut self) {
        match self
            .new_word_dict
            .add_word(Word::new(dictcn::get_raw_html(&self.capture_word).unwrap()))
        {
            Ok(_) => {
                println!("添加成功");
            }
            Err(e) => println!("添加失败, Err: {}", e),
        }
    }

    fn handle_delete_dict(&self, ctx: &egui::CtxRef) {
        todo!()
    }

    fn handle_import_dict(&self, ctx: &egui::CtxRef) {
        todo!()
    }

    fn handle_delete_word(&mut self, dict_familiarity: Familiarity, keyword: &String) {
        match match dict_familiarity {
            Familiarity::NewWord => self.new_word_dict.delete_word(keyword),
            Familiarity::Familiarity => self.familiarity_dict.delete_word(keyword),
            Familiarity::MemorizedDict => self.memorized_dict.delete_word(keyword),
        } {
            Ok(_) => println!("删除成功"),
            Err(e) => println!("删除失败, Err: {}", e),
        }
    }

    fn handle_move_word_to_dict(
        &self,
        keyword: &String,
        self_dict: &mut Dict,
        other_dict: &mut Dict,
    ) {
        match self_dict.move_word_to_dict(keyword, other_dict) {
            Ok(_) => println!("移动成功"),
            Err(e) => println!("移动失败, Err: {}", e),
        }
    }
}
impl epi::App for ShengCiApp {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        if self.setting {
            SidePanel::left("")
                .resizable(false)
                .default_width(100.)
                .width_range(80.0..=150.0)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
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
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("杀死生词");
                    ui.separator();
                });
                self.render_dict(&"生词".to_string(), ui);
                self.render_dict(&"熟悉".to_string(), ui);
                self.render_dict(&"记住".to_string(), ui);
            });
        });
        egui::TopBottomPanel::bottom("my_bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.setting, "设置");
                let text = ui.add(
                    egui::TextEdit::singleline(&mut self.capture_word)
                        .hint_text("Input a word to capture.")
                        .desired_width(400.)
                        .text_style(egui::TextStyle::Button),
                );
                if ui
                    .add(egui::Button::new("Capture!").text_color(CYAN))
                    .clicked()
                {
                    self.handle_capture_word();
                    self.capture_word = String::new();
                }
            });
        });
    }

    fn name(&self) -> &str {
        "Kill ShengCi"
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
