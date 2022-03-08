use crate::keyboard;
use crate::style;
use crate::{AppState, PageState};

impl AppState {
    pub fn handle_kbd_home(&mut self, key: keyboard::Key) {
        match key {
            keyboard::Key::LCD_1 => {
                // Join
            }
            keyboard::Key::LCD_2 => {
                // PRs
                self.page = PageState::PullRequests;
            }
            keyboard::Key::LCD_3 => {
                // Calendar
            }
            keyboard::Key::LCD_4 => {
                // Shortcuts
                self.page = PageState::Shortcuts { selected: None };
            }
            _ => (),
        }
    }

    pub fn render_home(&self, ui: &mut egui::Ui) {
        let mut frame = egui::Frame::none();
        frame.margin = egui::Vec2::new(20.0, 20.0);
        frame.show(ui, |ui| {
            let clip_rect = ui.max_rect().expand(5.0);
            ui.set_clip_rect(clip_rect);
            if let Some(calendar_event) = self.calendar.as_ref() {
                ui.horizontal(|ui| {
                    let mut frame = egui::Frame::none();
                    frame.margin = egui::Vec2::new(5.0, 5.0);
                    frame = frame.stroke(egui::Stroke::new(style::STROKE, style::FG));
                    frame.show(ui, |ui| {
                        let desc = egui::Label::new(
                            egui::RichText::new(style::eta(calendar_event.start)).monospace(),
                        );
                        ui.add(desc);
                    });

                    ui.add_space(20.0);
                    ui.heading(&calendar_event.title);
                });

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    let mut frame = egui::Frame::none();
                    frame.margin = egui::Vec2::new(5.0, 5.0);

                    if let Some(_zoom_url) = calendar_event.zoom_url.as_ref() {
                        frame = frame.stroke(egui::Stroke::new(style::STROKE, style::ZOOM_COLOR));
                        frame.fill = style::ZOOM_COLOR;
                        frame.show(ui, |ui| {
                            let desc = egui::Label::new(
                                egui::RichText::new("ZOOM")
                                    .monospace()
                                    .color(egui::Color32::BLACK),
                            );
                            ui.add(desc);
                        });
                    } else {
                        frame = frame.stroke(egui::Stroke::new(style::STROKE, style::BG));
                        frame.fill = style::BG;
                        frame.show(ui, |ui| {
                            let desc = egui::Label::new(egui::RichText::new("    ").monospace());
                            ui.add(desc);
                        });
                    }

                    ui.add_space(20.0);
                    ui.heading(&calendar_event.time);
                });

                ui.add_space(40.0);
            }

            for notification in &self.notifications {
                ui.horizontal(|ui| {
                    let mut frame = egui::Frame::none();
                    frame.margin = egui::Vec2::new(5.0, 5.0);
                    frame = frame.stroke(egui::Stroke::new(style::STROKE, style::FG));
                    frame.show(ui, |ui| {
                        let desc = egui::Label::new(
                            egui::RichText::new(style::eta(notification.time)).monospace(),
                        );
                        ui.add(desc);
                    });

                    ui.add_space(10.0);
                    ui.add(egui::Label::new(
                        egui::RichText::new(&notification.title).heading(),
                    ));
                });
                ui.horizontal(|ui| {
                    ui.add(egui::Label::new(
                        egui::RichText::new(format!(
                            "{} in {}",
                            notification.action, notification.repository
                        ))
                        .color(style::FG_MUTED)
                        .heading(),
                    ));
                });

                ui.add_space(10.0);
            }
        });
    }
}
