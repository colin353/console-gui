use crate::keyboard;
use crate::style;
use crate::{command, AppState, CalendarEvent, Command, PageState};

#[derive(Default)]
pub struct HomeState {
    slider: usize,
    selected: usize,
    scroll: usize,
}

impl AppState {
    pub fn handle_kbd_home(&mut self, key: keyboard::Key) {
        let mut hs = match &mut self.page {
            PageState::Home(hs) => hs,
            _ => unreachable!("wrong page!"),
        };

        match key {
            keyboard::Key::LCD1 => {
                // Join
                if let Some(CalendarEvent {
                    zoom_url: Some(zoom_url),
                    ..
                }) = &self.calendar
                {
                    command::run(&["xdg-open", zoom_url]);
                }
            }
            keyboard::Key::LCD2 => {
                // PRs
                self.page = PageState::pull_requests();
            }
            keyboard::Key::LCD3 => {
                // Calendar
            }
            keyboard::Key::LCD4 => {
                // Shortcuts
                self.page = PageState::Shortcuts { selected: None };
            }
            keyboard::Key::Slider(pos) => {
                hs.slider = 5 - pos;

                if hs.slider == 0 {
                    if hs.scroll > 0 {
                        hs.scroll -= 1;
                    } else {
                        hs.selected = 0;
                    }
                } else if hs.slider < 5 {
                    hs.selected = hs.slider;
                } else if hs.slider == 5 {
                    hs.selected = 5;

                    if hs.scroll >= self.notifications.len() - 5
                        && hs.selected < self.notifications.len()
                    {
                        hs.selected += 1;
                    }

                    if hs.scroll >= self.notifications.len() - 5 {
                        hs.scroll += 1;
                    }
                }
            }
            keyboard::Key::Execute => {
                // Execute selected thingy
                let selected = hs.selected + hs.scroll;
                if let Some(item) = self.notifications.get(selected) {
                    command::open_url(&item.url);
                }
            }
            _ => (),
        }
    }

    pub fn commands_home(&self) -> Vec<Command> {
        let mut commands = vec![
            Command::new("JOIN"),
            Command::new("PRS"),
            Command::new("CAL"),
            Command::new("SHCT"),
        ];

        if let Some(CalendarEvent {
            zoom_url: Some(_), ..
        }) = &self.calendar
        {
            commands[0].selected = true;
        } else {
            commands[0] = Command::empty();
        }
        commands
    }

    pub fn heartbeat_home(&mut self) {
        let mut hs = match &mut self.page {
            PageState::Home(hs) => hs,
            _ => unreachable!("wrong page!"),
        };

        if hs.slider == 0 {
            if hs.scroll > 0 {
                hs.scroll -= 1;
            } else if hs.selected > 0 {
                hs.selected -= 1;
            }
        } else if hs.slider == 5 {
            hs.scroll += 1;
        }
    }

    pub fn render_home(&self, ui: &mut egui::Ui) {
        let hs = match &self.page {
            PageState::Home(hs) => hs,
            _ => unreachable!("wrong page!"),
        };

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

            for (idx, notification) in self.notifications.iter().skip(hs.scroll).enumerate() {
                ui.horizontal(|ui| {
                    let mut frame = egui::Frame::none();
                    frame.margin = egui::Vec2::new(5.0, 5.0);
                    frame = frame.stroke(egui::Stroke::new(style::STROKE, style::FG));
                    if idx == hs.selected {
                        frame.fill = style::FG;
                    }

                    frame.show(ui, |ui| {
                        let desc = egui::Label::new(
                            egui::RichText::new(style::eta(notification.time))
                                .monospace()
                                .color(if idx == hs.selected {
                                    style::BG
                                } else {
                                    style::FG
                                }),
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
