use crate::keyboard;
use crate::style;
use crate::{AppState, PageState};

impl AppState {
    pub fn handle_kbd_pull_requests(&mut self, key: keyboard::Key) {
        match key {
            keyboard::Key::LCD1 => {
                // Join
            }
            keyboard::Key::LCD2 => {
                // PRs
                self.page = PageState::PullRequests;
            }
            keyboard::Key::LCD3 => {
                // Calendar
            }
            keyboard::Key::LCD4 => {
                // Shortcuts
                self.page = PageState::Shortcuts { selected: None };
            }
            keyboard::Key::Abort => self.page = PageState::Home,
            _ => (),
        }
    }

    pub fn render_pull_requests(&self, ui: &mut egui::Ui) {
        let mut frame = egui::Frame::none();
        frame.margin = egui::Vec2::new(20.0, 20.0);
        frame.show(ui, |ui| {
            let clip_rect = ui.max_rect().expand(5.0);
            ui.set_clip_rect(clip_rect);
            ui.horizontal(|ui| {
                let mut frame = egui::Frame::none();
                frame.margin = egui::Vec2::new(5.0, 5.0);
                frame = frame.stroke(egui::Stroke::new(style::STROKE, style::FG));
                frame.show(ui, |ui| {
                    let desc = egui::Label::new(egui::RichText::new("+1d").monospace());
                    ui.add(desc);
                });

                ui.add_space(10.0);
                ui.add(egui::Label::new(
                    egui::RichText::new("Add calendar, github notifications").heading(),
                ));
            });
            ui.horizontal(|ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new("blackbird-mw")
                        .color(style::FG_MUTED)
                        .heading(),
                ));
            });

            ui.add_space(10.0);
        });
    }
}
