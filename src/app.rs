use eframe::{
    egui::{
        self,
        plot::{Bar, BarChart, Legend, Plot},
        style, Button, CentralPanel, Color32, CtxRef, FontData, FontDefinitions, FontFamily,
        RichText, ScrollArea, SidePanel, TextEdit, TextStyle, TopBottomPanel, Vec2,
    },
    epi::{self, Frame, Storage},
};

use crate::{
    db_manage::DBManage,
    dictcn,
    word::{Familiarity, Word},
};

const PADDING: f32 = 20.;
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
#[derive(Default)]
pub struct App {
    pub db: DBManage,
    setting: bool,
    capture_word: String,
    msg: String,
    radio_familiarity: Familiarity,
    show_word_info: bool,
    current_word: Word,
    search_word: String,
    word_list: Vec<Word>,
}
impl App {
    fn new() -> Self {
        Self {
            msg: "杀死生词".to_string(),
            ..Self::default()
        }
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
        ui.vertical(|ui| {
            ScrollArea::vertical()
                .auto_shrink([true; 2])
                .show(ui, |ui| {
                    let search_text = ui.add(
                        TextEdit::singleline(&mut self.search_word)
                            .hint_text("模糊搜索")
                            .desired_width(100.),
                    );
                    // 如果单词列表为空，那么将其设置为数据库中的单词列表
                    if self.word_list.len() == 0 || self.search_word == "" {
                        self.word_list = self.db.get_words(familiarity).unwrap();
                    }
                    if search_text.changed() {
                        if &self.search_word != "" {
                            match self.db.get_words_by_regexp_keyword(&self.search_word) {
                                Ok(words) => self.word_list = words,
                                Err(_) => unreachable!(),
                            }
                        }
                    }
                    let word_list = self.word_list.clone();
                    for word in word_list {
                        let word_resp = ui.add(Button::new(word.keyword.as_ref().unwrap()).small());
                        if word_resp.clicked() {
                            self.show_word_info = true;
                            self.current_word = word.clone();
                        }
                        word_resp.context_menu(|ui| {
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
                    }
                });
        });
        ui.add_space(PADDING);
        // println!("{:#?}", self.current_word);
        if self.show_word_info {
            ScrollArea::vertical()
                .auto_shrink([true; 2])
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.collapsing("解释", |ui| {
                            for explain in &self.current_word.explains {
                                ui.add(egui::Label::new(
                                    RichText::new(&explain.0).text_style(egui::TextStyle::Button),
                                ));
                                ui.add(egui::Label::new(
                                    RichText::new(&explain.1).text_style(egui::TextStyle::Button),
                                ));
                            }
                        });
                        ui.collapsing("音节", |ui| {
                            ui.add(egui::Label::new(
                                RichText::new(self.current_word.tips.as_ref().unwrap())
                                    .text_style(TextStyle::Button),
                            ));
                        });
                        ui.collapsing("音标", |ui| {
                            ui.horizontal(|ui| {
                                ui.add(egui::Label::new(
                                    RichText::new("英").text_style(egui::TextStyle::Button),
                                ));
                                ui.add(egui::Label::new(
                                    RichText::new(&self.current_word.phonetic.as_ref().unwrap().0)
                                        .text_style(egui::TextStyle::Button),
                                ));
                            });
                            ui.horizontal(|ui| {
                                ui.add(egui::Label::new(
                                    RichText::new("美").text_style(egui::TextStyle::Button),
                                ));
                                ui.add(egui::Label::new(
                                    RichText::new(&self.current_word.phonetic.as_ref().unwrap().1)
                                        .text_style(egui::TextStyle::Button),
                                ));
                            });
                        });
                        ui.collapsing("起源", |ui| {
                            ui.add(egui::Label::new(
                                RichText::new(self.current_word.etymons.as_ref().unwrap())
                                    .text_style(egui::TextStyle::Button),
                            ));
                        });
                        ui.collapsing("多形式", |ui| {
                            for shape in &self.current_word.shape {
                                ui.horizontal(|ui| {
                                    ui.add(egui::Label::new(
                                        RichText::new(&shape.0).text_style(egui::TextStyle::Button),
                                    ));
                                    ui.add(egui::Label::new(
                                        RichText::new(&shape.1).text_style(egui::TextStyle::Button),
                                    ));
                                });
                            }
                        });
                        ui.collapsing("词组", |ui| {
                            for phrase in &self.current_word.phrase {
                                ui.add(egui::Hyperlink::from_label_and_url(&phrase.0, &phrase.1));
                            }
                        });
                        ui.collapsing("用词分布", |ui| {
                            let mut data_chart: Vec<BarChart> = vec![];
                            for (i, data) in self.current_word.distribution_data.iter().enumerate()
                            {
                                let chart =
                                    BarChart::new(vec![Bar::new((i * 20) as f64, data.0 as f64)])
                                        .name(&data.1)
                                        .width(20.);
                                data_chart.push(chart);
                            }
                            Plot::new("用词分布")
                                .legend(Legend::default())
                                .data_aspect(1.)
                                .height(200.)
                                .width(400.)
                                .show(ui, |plot_ui| {
                                    for chart in data_chart {
                                        plot_ui.bar_chart(chart);
                                    }
                                });
                        });
                    });
                });
        }
    }
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
            .insert(egui::TextStyle::Button, (FontFamily::Monospace, 17.0));
        fonts
            .family_and_size
            .insert(egui::TextStyle::Heading, (FontFamily::Monospace, 30.0));
        fonts
            .family_and_size
            .insert(egui::TextStyle::Body, (FontFamily::Monospace, 15.0));
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
        TopBottomPanel::bottom("my_bottom_panel")
            .min_height(10.)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.setting, "设置");
                    ui.separator();
                    let text_resp = ui.add(
                        TextEdit::singleline(&mut self.capture_word)
                            .hint_text("请输入一个单词以捕获|回车或点击按钮")
                            .desired_width(250.)
                            .text_style(TextStyle::Button),
                    );
                    // 回车时进行捕获
                    if text_resp.lost_focus() {
                        if self.capture_word != "" {
                            self.handle_capture_word();
                            self.capture_word = String::new();
                        }
                    }
                    // 点击按钮时捕获
                    if ui
                        .add(Button::new(RichText::new("添加到生词表！").color(CYAN)))
                        .clicked()
                    {
                        self.handle_capture_word();
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
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(&self.msg);
                ui.separator();
            });
            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                match self.radio_familiarity {
                    Familiarity::NewWord => self.render_dict(
                        &"生词".to_string(),
                        &Familiarity::NewWord,
                        &Familiarity::Familiarity,
                        ui,
                    ),
                    Familiarity::Familiarity => self.render_dict(
                        &"熟悉".to_string(),
                        &Familiarity::Familiarity,
                        &Familiarity::Memorized,
                        ui,
                    ),
                    Familiarity::Memorized => self.render_dict(
                        &"记住".to_string(),
                        &Familiarity::Memorized,
                        &Familiarity::Familiarity,
                        ui,
                    ),
                }
            });
        });
    }

    fn name(&self) -> &str {
        "Kill ShengCi"
    }
}
