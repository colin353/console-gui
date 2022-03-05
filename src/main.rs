use eframe::egui::{FontDefinitions, FontFamily};
use eframe::{egui, epi};

use std::sync::{Arc, Mutex};

mod calendar;
mod github;

const ZOOM_COLOR: egui::Color32 = egui::Color32::from_rgb(0x2D, 0x8C, 0xFF);
const BG: egui::Color32 = egui::Color32::BLACK;
const FG: egui::Color32 = egui::Color32::GRAY;
const FG_MUTED: egui::Color32 = egui::Color32::from_rgb(80, 80, 80);

const STROKE: f32 = 1.0;

#[derive(Clone)]
struct App {
    data: Arc<Mutex<AppState>>,
}

pub struct CalendarEvent {
    title: String,
    time: String,
    start: i64,
    zoom_url: Option<String>,
}

pub struct GitHubNotification {
    title: String,
    action: String,
    repository: String,
    time: i64,
}

fn rounding_div(a: i64, b: i64) -> i64 {
    (a as f64 / b as f64).round() as i64
}

fn eta(time: i64) -> String {
    let now = chrono::prelude::Utc::now().timestamp();
    let eta = now - time;

    let sign = if eta.signum() == 1 { "+" } else { "-" };
    let eta = eta.abs();

    // ETA > 12 hours, show at least 1 day
    const MINUTE: i64 = 60;
    const HOUR: i64 = 60 * MINUTE;
    if eta > 24 * HOUR {
        return format!("{}{:>2}d", sign, rounding_div(eta, 24 * HOUR));
    } else if eta > 12 * HOUR {
        return format!("{} 1d", sign);
    } else if eta > HOUR {
        return format!("{}{:>2}h", sign, rounding_div(eta, HOUR));
    } else {
        return format!("{}{:>2}m", sign, rounding_div(eta, MINUTE));
    }
}

impl CalendarEvent {
    fn as_eta(&self) -> String {
        eta(self.start)
    }
}

enum PageState {
    Home,
    Shortcuts { selected: Option<usize> },
}

pub struct AppState {
    page: PageState,
    frame: Option<epi::Frame>,
    clock: String,
    calendar: Option<CalendarEvent>,
    notifications: Vec<GitHubNotification>,
}

impl AppState {
    fn new() -> Self {
        Self {
            page: PageState::Home,
            frame: None,
            clock: Self::clock_time(),
            calendar: None,
            notifications: Vec::new(),
        }
    }

    fn clock_time() -> String {
        chrono::prelude::Local::now()
            .format(" %h %d  %l:%M%P ")
            .to_string()
    }

    fn home_pane(&self, ui: &mut egui::Ui) {
        let mut frame = egui::Frame::none();
        frame.margin = egui::Vec2::new(20.0, 20.0);
        frame.show(ui, |ui| {
            let clip_rect = ui.max_rect().expand(5.0);
            ui.set_clip_rect(clip_rect);
            if let Some(calendar_event) = self.calendar.as_ref() {
                ui.horizontal(|ui| {
                    let mut frame = egui::Frame::none();
                    frame.margin = egui::Vec2::new(5.0, 5.0);
                    frame = frame.stroke(egui::Stroke::new(STROKE, FG));
                    frame.show(ui, |ui| {
                        let desc = egui::Label::new(
                            egui::RichText::new(calendar_event.as_eta()).monospace(),
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
                    } else {
                        frame = frame.stroke(egui::Stroke::new(STROKE, BG));
                        frame.fill = BG;
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
                    frame = frame.stroke(egui::Stroke::new(STROKE, FG));
                    frame.show(ui, |ui| {
                        let desc = egui::Label::new(
                            egui::RichText::new(eta(notification.time)).monospace(),
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
                        .color(FG_MUTED)
                        .heading(),
                    ));
                });

                ui.add_space(10.0);
            }
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
                    egui::RichText::new(&self.clock)
                        .monospace()
                        .color(egui::Color32::BLACK),
                );
                ui.add(desc);
            });
        });
    }

    fn commands(&self) -> &[&str] {
        match self.page {
            PageState::Home => {
                if let Some(cal) = &self.calendar {
                    if cal.zoom_url.is_some() {
                        return &["JOIN", "PRS", "CAL", "SHCT"];
                    }
                }

                &["", "PRS", "CAL", "SHCT"]
            }
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

    async fn start_async(&self) {
        let data = self.data.clone();
        tokio::spawn(async move {
            calendar::run(data).await;
        });
        let data = self.data.clone();
        tokio::spawn(async move {
            github::run(data).await;
        });

        // Timer to refresh UI
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            let mut _data = self.data.lock().unwrap();
            if let Some(frame) = _data.frame.as_ref() {
                frame.request_repaint();
            }
            _data.clock = AppState::clock_time();
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
            .frame(egui::Frame::none().fill(BG))
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
            .frame(egui::Frame::none().fill(BG))
            .show(ctx, |ui| {
                let app_data = self.data.lock().unwrap();

                let (rect, _) = ui.allocate_exact_size(
                    egui::Vec2::new(ui.available_width(), ui.available_height() - 37.0),
                    egui::Sense::hover(),
                );
                app_data.footer(ui);

                let mut content_ui = ui.child_ui(rect, egui::Layout::top_down(egui::Align::Min));
                match app_data.page {
                    PageState::Home => app_data.home_pane(&mut content_ui),
                    PageState::Shortcuts { selected: _ } => app_data.shortcut(&mut content_ui),
                }
                content_ui.add_space(content_ui.available_height());
            });
    }
}

#[tokio::main]
async fn main() {
    let options = eframe::NativeOptions::default();

    let app = App::new();
    app.bindkeys();

    let _app = app.clone();
    tokio::spawn(async move { _app.start_async().await });
    eframe::run_native(Box::new(app), options);
}
