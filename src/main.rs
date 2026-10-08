use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Forge")
            .with_inner_size([1200.0, 800.0]),

        ..Default::default()
    };

    eframe::run_native(
        "Forge",
        options,
        Box::new(|_cc| {
            Ok(Box::new(ForgeApp {
                current_page: Page::Files,
                search_text: String::new(),
            }))
        }),
    )
}

struct ForgeApp {
    current_page: Page,
    search_text: String,
}

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Files,
    Search,
    Git,
    Terminal,
}

impl eframe::App for ForgeApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let background_color = egui::Color32::from_rgb(15, 18, 24);

        let sidebar_color = egui::Color32::from_rgb(22, 26, 35);

        let content_color = egui::Color32::from_rgb(27, 32, 43);

        let button_color = egui::Color32::from_rgb(35, 41, 54);

        let accent_color = egui::Color32::from_rgb(70, 110, 240);

        let muted_text = egui::Color32::from_rgb(150, 158, 175);

        // ÜST BAR
        egui::Panel::top("top_bar")
            .exact_size(52.0)
            .frame(egui::Frame::new().fill(sidebar_color))
            .show(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.heading(egui::RichText::new("FORGE").size(20.0).strong());

                    ui.add_space(14.0);

                    ui.label(
                        egui::RichText::new("Developer Workspace")
                            .size(13.0)
                            .color(muted_text),
                    );
                });
            });

        // SOL SIDEBAR
        egui::Panel::left("sidebar")
            .exact_size(170.0)
            .frame(egui::Frame::new().fill(sidebar_color))
            .show(ui, |ui| {
                ui.add_space(12.0);

                ui.label(
                    egui::RichText::new("WORKSPACE")
                        .size(11.0)
                        .color(muted_text),
                );

                ui.add_space(10.0);

                // FILES
                let files_selected = self.current_page == Page::Files;

                let files_color = if files_selected {
                    accent_color
                } else {
                    button_color
                };

                if ui
                    .add_sized([150.0, 38.0], egui::Button::new("Files").fill(files_color))
                    .clicked()
                {
                    self.current_page = Page::Files;
                }

                // SEARCH
                let search_selected = self.current_page == Page::Search;

                let search_color = if search_selected {
                    accent_color
                } else {
                    button_color
                };

                if ui
                    .add_sized(
                        [150.0, 38.0],
                        egui::Button::new("Search").fill(search_color),
                    )
                    .clicked()
                {
                    self.current_page = Page::Search;
                }

                // GIT
                let git_selected = self.current_page == Page::Git;

                let git_color = if git_selected {
                    accent_color
                } else {
                    button_color
                };

                if ui
                    .add_sized([150.0, 38.0], egui::Button::new("Git").fill(git_color))
                    .clicked()
                {
                    self.current_page = Page::Git;
                }

                // TERMINAL
                let terminal_selected = self.current_page == Page::Terminal;

                let terminal_color = if terminal_selected {
                    accent_color
                } else {
                    button_color
                };

                if ui
                    .add_sized(
                        [150.0, 38.0],
                        egui::Button::new("Terminal").fill(terminal_color),
                    )
                    .clicked()
                {
                    self.current_page = Page::Terminal;
                }
            });

        // ANA İÇERİK
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(background_color))
            .show(ui, |ui| {
                ui.add_space(18.0);

                egui::Frame::new().fill(content_color).show(ui, |ui| {
                    ui.add_space(16.0);

                    match self.current_page {
                        Page::Files => {
                            ui.heading(egui::RichText::new("Files").size(26.0).strong());

                            ui.add_space(12.0);

                            ui.label(
                                egui::RichText::new("Project Explorer")
                                    .size(12.0)
                                    .color(muted_text),
                            );

                            ui.add_space(16.0);

                            ui.label(egui::RichText::new("▾ src").monospace());

                            ui.label(
                                egui::RichText::new("    main.rs")
                                    .monospace()
                                    .color(muted_text),
                            );

                            ui.label(
                                egui::RichText::new("Cargo.toml")
                                    .monospace()
                                    .color(muted_text),
                            );
                        }

                        Page::Search => {
                            ui.heading(egui::RichText::new("Search").size(26.0).strong());

                            ui.add_space(12.0);

                            ui.label(
                                egui::RichText::new("Search files in your workspace")
                                    .color(muted_text),
                            );

                            ui.add_space(16.0);

                            ui.add_sized(
                                [400.0, 36.0],
                                egui::TextEdit::singleline(&mut self.search_text)
                                    .hint_text("Search..."),
                            );

                            ui.add_space(12.0);

                            if !self.search_text.is_empty() {
                                ui.label(format!("Searching for: {}", self.search_text));
                            }
                        }

                        Page::Git => {
                            ui.heading(egui::RichText::new("Git").size(26.0).strong());

                            ui.add_space(12.0);

                            ui.label(egui::RichText::new("Source Control").color(muted_text));

                            ui.add_space(16.0);

                            ui.label(egui::RichText::new("M  src/main.rs").monospace());

                            ui.label(egui::RichText::new("M  Cargo.toml").monospace());
                        }

                        Page::Terminal => {
                            ui.heading(egui::RichText::new("Terminal").size(26.0).strong());

                            ui.add_space(12.0);

                            ui.label(egui::RichText::new("zsh").size(12.0).color(muted_text));

                            ui.add_space(16.0);

                            ui.label(
                                egui::RichText::new("$ cargo run\nHello from Forge!").monospace(),
                            );
                        }
                    }

                    ui.add_space(16.0);
                });
            });
    }
}
