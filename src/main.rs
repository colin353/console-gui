use eframe::egui::{FontDefinitions, FontFamily};
use eframe::{egui, epi};

use std::sync::{Arc, Mutex};

const ZOOM_COLOR: egui::Color32 = egui::Color32::from_rgb(0x2D, 0x8C, 0xFF);
const FG: egui::Color32 = egui::Color32::GRAY;

const STROKE: f32 = 1.0;

#[derive(Clone)]
struct App {
    data: Arc<Mutex<AppState>>,
}

enum PageState {
    Home,
    Shortcuts { selected: Option<usize> },
}

struct AppState {
    page: PageState,
    frame: Option<epi::Frame>,
}

impl AppState {
    fn new() -> Self {
        Self {
            page: PageState::Home,
            frame: None,
        }
    }

    fn home_pane(&self, ui: &mut egui::Ui) {
        let mut frame = egui::Frame::none();
        frame.margin = egui::Vec2::new(20.0, 20.0);
        frame.show(ui, |ui| {
            let clip_rect = ui.max_rect().expand(5.0);
            ui.set_clip_rect(clip_rect);
            ui.horizontal(|ui| {
                let mut frame = egui::Frame::none();
                frame.margin = egui::Vec2::new(5.0, 5.0);
                frame = frame.stroke(egui::Stroke::new(STROKE, FG));
                frame.show(ui, |ui| {
                    let desc = egui::Label::new(egui::RichText::new("-30m").monospace());
                    ui.add(desc);
                });

                ui.add_space(20.0);
                ui.heading("Search Sync Extremely Long String");
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                let mut frame = egui::Frame::none();
                frame.margin = egui::Vec2::new(5.0, 5.0);
                frame = frame.stroke(egui::Stroke::new(STROKE, ZOOM_COLOR));
                frame.fill = ZOOM_COLOR;
                frame.show(ui, |ui| {
                    let desc = egui::Label::new(
                        egui::RichText::new("ZOOM")
                            .monospace()
                            .color(egui::Color32::BLACK),
                    );
                    ui.add(desc);
                });

                ui.add_space(20.0);
                ui.heading("2:00pm - 3:00pm");
            });

            ui.add_space(40.0);

            ui.horizontal(|ui| {
                let mut frame = egui::Frame::none();
                frame.margin = egui::Vec2::new(5.0, 5.0);
                frame = frame.stroke(egui::Stroke::new(STROKE, FG));
                frame.show(ui, |ui| {
                    let desc = egui::Label::new(egui::RichText::new("+ 2H").monospace());
                    ui.add(desc);
                });

                ui.add_space(20.0);
                ui.add(egui::Label::new(
                    egui::RichText::new("@look commented").heading(),
                ));
            });
        });
    }

    fn shortcut(&self, ui: &mut egui::Ui) {
        let shortcuts = &[
            &["guvcview", "zoom", "screenshot"],
            &["slack", "something", "shutdown"],
        ];

        let chunk_size = ui.available_height() / 4.0;

        let selected = match self.page {
            PageState::Shortcuts { selected } => selected,
            _ => None,
        };

        for (idx_outer, shortcut_chunk) in shortcuts.iter().enumerate() {
            ui.allocate_ui(egui::Vec2::new(ui.available_width(), chunk_size), |ui| {
                let mut frame = egui::Frame::none();
                frame = frame.stroke(egui::Stroke::new(STROKE, FG));
                frame.show(ui, |ui| {
                    let mut frame = egui::Frame::none();
                    frame = frame.stroke(egui::Stroke::new(STROKE, FG));

                    let mut fg = FG;
                    let mut bg = egui::Color32::BLACK;
                    if selected.is_some() {
                        fg = bg;
                        bg = FG;
                    }

                    frame.fill = fg;
                    frame.show(ui, |ui| {
                        let desc = egui::Label::new(
                            egui::RichText::new(format!(" {} ", idx_outer + 1))
                                .monospace()
                                .color(bg),
                        );
                        ui.add(desc);
                    });

                    for (idx, shortcut) in shortcut_chunk.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.add_space(39.);

                            let mut frame = egui::Frame::none();
                            frame = frame.stroke(egui::Stroke::new(STROKE, FG));

                            let mut fg = FG;
                            let mut bg = egui::Color32::BLACK;
                            if let Some(sel_idx) = selected {
                                if sel_idx == idx_outer {
                                    fg = bg;
                                    bg = FG;
                                }
                            }

                            frame.fill = bg;
                            frame.show(ui, |ui| {
                                let desc = egui::Label::new(
                                    egui::RichText::new(format!(" {} ", idx + 1))
                                        .monospace()
                                        .color(fg),
                                );
                                ui.add(desc);
                            });

                            ui.heading(shortcut.to_string());
                            ui.add_space(ui.available_width() - 10.);
                        });
                    }

                    ui.add_space(ui.available_height());
                });
            });
        }
    }

    fn footer(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let mut frame = egui::Frame::none();
            frame.margin = egui::Vec2::new(5.0, 5.0);
            frame = frame.stroke(egui::Stroke::new(STROKE, FG));
            frame.fill = FG;
            frame.show(ui, |ui| {
                let desc = egui::Label::new(
                    egui::RichText::new(" Feb 17  2:35PM ")
                        .monospace()
                        .color(egui::Color32::BLACK),
                );
                ui.add(desc);
            });
        });
    }

    fn commands(&self) -> &[&str] {
        match self.page {
            PageState::Home => &["JOIN", "PRS", "CAL", "SHCT"],
            PageState::Shortcuts { .. } => &["1", "2", "3", "BACK"],
        }
    }
}

impl App {
    fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(AppState::new())),
        }
    }

    fn bindkeys(&self) {
        let _self = self.clone();
        inputbot::KeybdKey::LShiftKey.bind(move || {
            let mut state = _self.data.lock().unwrap();
            match state.page {
                PageState::Home => state.page = PageState::Shortcuts { selected: None },
                PageState::Shortcuts { .. } => state.page = PageState::Home,
            };

            if let Some(frame) = state.frame.as_ref() {
                frame.request_repaint();
            }
        });

        let _self = self.clone();
        inputbot::KeybdKey::RShiftKey.bind(move || {
            let mut state = _self.data.lock().unwrap();
            match state.page {
                PageState::Shortcuts { .. } => {
                    state.page = PageState::Shortcuts { selected: Some(1) }
                }
                _ => return,
            };

            if let Some(frame) = state.frame.as_ref() {
                frame.request_repaint();
            }
        });

        std::thread::spawn(|| {
            inputbot::handle_input_events();
        });
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "console GUI"
    }

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Create a font definition object
        let mut font_def = FontDefinitions::default();
        font_def
            .family_and_size
            .insert(egui::TextStyle::Heading, (FontFamily::Proportional, 24.));

        font_def
            .family_and_size
            .insert(egui::TextStyle::Monospace, (FontFamily::Monospace, 24.));
        // Set the size of text styles
        // Load the font using ctx
        ctx.set_fonts(font_def);

        let mut app_data = self.data.lock().unwrap();
        app_data.frame = Some(frame.clone());
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        egui::SidePanel::right("right_panel")
            .frame(egui::Frame::none())
            .default_width(30.0)
            .max_width(30.0)
            .min_width(30.0)
            .show(ctx, |ui| {
                let app_data = self.data.lock().unwrap();

                let spacing = ui.available_height() / 4.0;

                for cmd in app_data.commands() {
                    ui.allocate_ui(egui::Vec2::new(20.0, spacing), |ui| {
                        let padding = 10.0;
                        let text_space = cmd.len() as f32 * 25.0 + 2.0 * padding;

                        ui.add_space((spacing - text_space) / 2.0);
                        let mut frame = egui::Frame::none();
                        frame.margin = egui::Vec2::new(5.0, padding);
                        if cmd == &"JOIN" {
                            frame = frame.stroke(egui::Stroke::new(STROKE, FG));
                        };

                        frame.show(ui, |ui| {
                            let desc =
                                egui::Label::new(egui::RichText::new(cmd.to_string()).monospace());
                            ui.add(desc);
                        });
                        ui.add_space((spacing - text_space) / 2.0);
                    });
                }
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                let app_data = self.data.lock().unwrap();

                ui.allocate_ui(
                    egui::Vec2::new(ui.available_width(), ui.available_height() - 37.0),
                    |ui| {
                        match app_data.page {
                            PageState::Home => app_data.home_pane(ui),
                            PageState::Shortcuts { selected: _ } => app_data.shortcut(ui),
                        }
                        ui.add_space(ui.available_height());
                    },
                );

                app_data.footer(ui);
            });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();

    let app = App::new();
    app.bindkeys();
    eframe::run_native(Box::new(app), options);
}
