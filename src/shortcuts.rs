use crate::keyboard;
use crate::style;
use crate::{command, AppState, Command, PageState};

impl AppState {
    pub fn handle_kbd_shortcuts(&mut self, key: keyboard::Key) {
        let selected = match self.page {
            PageState::Shortcuts { selected } => selected,
            _ => unreachable!("wrong page!"),
        };

        match key {
            keyboard::Key::LCD1 => {
                if let Some(s) = selected {
                    match s {
                        0 => {
                            command::run(&["/home/colinwm/bin/fix-video.sh"]);
                        }
                        1 => {
                            command::run(&["slack"]);
                        }
                        _ => println!("unknown!"),
                    }
                    self.page = PageState::Shortcuts { selected: None };
                } else {
                    self.page = PageState::Shortcuts { selected: Some(0) };
                }
            }
            keyboard::Key::LCD2 => {
                if let Some(s) = selected {
                    match s {
                        0 => {
                            let zoom_url =
                                std::env::var("ZOOM_URL").expect("must provide $ZOOM_URL env var");
                            if let Err(e) = std::process::Command::new("xdg-open")
                                .args([&zoom_url])
                                .spawn()
                            {
                                eprintln!("unable to start zoom! {:?}", e);
                            }
                        }
                        1 => println!("something"),
                        _ => println!("unknown!"),
                    }
                    self.page = PageState::Shortcuts { selected: None };
                } else {
                    self.page = PageState::Shortcuts { selected: Some(1) };
                }
            }
            keyboard::Key::LCD3 => {
                if let Some(s) = selected {
                    match s {
                        0 => {
                            if let Err(e) =
                                std::process::Command::new("/home/colinwm/bin/screenshot.sh")
                                    .spawn()
                            {
                                eprintln!("unable to screenshot! {:?}", e);
                            }
                        }
                        1 => {
                            if let Err(e) = std::process::Command::new("shutdown")
                                .args(["-h", "now"])
                                .spawn()
                            {
                                eprintln!("unable to shut down! {:?}", e);
                            }
                        }
                        _ => println!("unknown!"),
                    }
                    self.page = PageState::Shortcuts { selected: None };
                } else {
                    self.page = PageState::Shortcuts { selected: Some(2) };
                }
            }
            keyboard::Key::Abort | keyboard::Key::LCD4 => {
                if selected.is_some() {
                    self.page = PageState::Shortcuts { selected: None };
                } else {
                    self.page = PageState::Home(crate::home::HomeState::default());
                }
            }
            _ => (),
        }
    }

    pub fn commands_shortcuts(&self) -> Vec<Command> {
        vec![
            Command::new("1"),
            Command::new("2"),
            Command::new("3"),
            Command::new("BACK"),
        ]
    }

    pub fn render_shortcuts(&self, ui: &mut egui::Ui) {
        let shortcuts = &[
            &["fix-video.sh", "zoom personal room", "screenshot"],
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
                frame = frame.stroke(egui::Stroke::new(style::STROKE, style::FG));
                frame.show(ui, |ui| {
                    let mut frame = egui::Frame::none();
                    frame = frame.stroke(egui::Stroke::new(style::STROKE, style::FG));

                    let mut fg = style::FG;
                    let mut bg = egui::Color32::BLACK;
                    if selected.is_some() {
                        fg = bg;
                        bg = style::FG;
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
                            frame = frame.stroke(egui::Stroke::new(style::STROKE, style::FG));

                            let mut fg = style::FG;
                            let mut bg = egui::Color32::BLACK;
                            if let Some(sel_idx) = selected {
                                if sel_idx == idx_outer {
                                    fg = bg;
                                    bg = style::FG;
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
}
