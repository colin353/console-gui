use crate::command;
use crate::keyboard;
use crate::style;
use crate::{AppState, Command, PageState, PullRequest};

#[derive(PartialEq)]
pub enum Page {
    InProgress,
    Submitted,
    Review,
}

pub struct PullRequestsState {
    page: Page,
    selected: usize,
    scroll: usize,
    slider: usize,
}

impl PullRequestsState {
    pub fn default() -> Self {
        Self {
            page: Page::InProgress,
            selected: 0,
            scroll: 0,
            slider: 0,
        }
    }
}

impl AppState {
    pub fn handle_kbd_pull_requests(&mut self, key: keyboard::Key) {
        let num_pulls = self.get_pulls().len();
        let mut s = match &mut self.page {
            PageState::PullRequests(s) => s,
            _ => unreachable!("wrong page!"),
        };

        match key {
            keyboard::Key::LCD1 => {
                // In progress
                s.page = Page::InProgress;
            }
            keyboard::Key::LCD2 => {
                // Submitted
                s.page = Page::Submitted;
            }
            keyboard::Key::LCD3 => {
                s.page = Page::Review;
            }
            keyboard::Key::Abort => self.page = PageState::home(),
            keyboard::Key::Execute => {
                // Execute selected thingy
                let selected = s.selected + s.scroll;
                if let Some(item) = self.get_pulls().get(selected) {
                    command::open_url(&item.url);
                }
            }
            keyboard::Key::Slider(pos) => {
                s.slider = 5 - pos;

                if s.slider == 0 {
                    if s.scroll > 0 {
                        s.scroll -= 1;
                    } else {
                        s.selected = 0;
                    }
                } else if s.slider < 5 {
                    s.selected = s.slider;
                } else if s.slider == 5 {
                    s.selected = 5;

                    if s.scroll >= num_pulls - 5 && s.selected < num_pulls {
                        s.selected += 1;
                    }

                    if s.scroll >= num_pulls - 5 {
                        s.scroll += 1;
                    }
                }
            }
            _ => (),
        }
    }

    pub fn heartbeat_pulls(&mut self) {
        let mut s = match &mut self.page {
            PageState::PullRequests(s) => s,
            _ => unreachable!("wrong page!"),
        };

        if s.slider == 0 {
            if s.scroll > 0 {
                s.scroll -= 1;
            } else if s.selected > 0 {
                s.selected -= 1;
            }
        } else if s.slider == 5 {
            s.scroll += 1;
        }
    }

    pub fn get_pulls(&self) -> &[PullRequest] {
        let s = match &self.page {
            PageState::PullRequests(s) => s,
            _ => unreachable!("wrong page!"),
        };
        match s.page {
            Page::InProgress => self.open_prs.as_slice(),
            Page::Submitted => self.closed_prs.as_slice(),
            Page::Review => &[],
        }
    }

    pub fn render_pull_requests(&self, ui: &mut egui::Ui) {
        let s = match &self.page {
            PageState::PullRequests(s) => s,
            _ => unreachable!("wrong page!"),
        };
        let mut frame = egui::Frame::none();
        frame.margin = egui::Vec2::new(20.0, 20.0);
        frame.show(ui, |ui| {
            let clip_rect = ui.max_rect().expand(5.0);
            ui.set_clip_rect(clip_rect);

            for (idx, pr) in self.get_pulls().iter().skip(s.scroll).enumerate() {
                ui.horizontal(|ui| {
                    let mut frame = egui::Frame::none();
                    frame.margin = egui::Vec2::new(5.0, 5.0);
                    frame = frame.stroke(egui::Stroke::new(style::STROKE, style::FG));

                    if idx == s.selected {
                        frame.fill = style::FG;
                    }
                    frame.show(ui, |ui| {
                        let desc = egui::Label::new(
                            egui::RichText::new(style::eta(pr.updated_at))
                                .monospace()
                                .color(if idx == s.selected {
                                    style::BG
                                } else {
                                    style::FG
                                }),
                        );
                        ui.add(desc);
                    });

                    ui.add_space(10.0);
                    ui.add(egui::Label::new(egui::RichText::new(&pr.title).heading()));
                });
                ui.horizontal(|ui| {
                    ui.add(egui::Label::new(
                        egui::RichText::new(&pr.repo_name)
                            .color(style::FG_MUTED)
                            .heading(),
                    ));
                });
            }

            ui.add_space(10.0);
        });
    }

    pub fn commands_prs(&self) -> Vec<Command> {
        let s = match &self.page {
            PageState::PullRequests(s) => s,
            _ => unreachable!("wrong page!"),
        };

        vec![
            Command {
                name: "IPR",
                selected: s.page == Page::InProgress,
            },
            Command {
                name: "SUB",
                selected: s.page == Page::Submitted,
            },
            Command {
                name: "REV",
                selected: s.page == Page::Review,
            },
            Command::empty(),
        ]
    }
}
